<script>
	import { project, workbenchIndex, workbenchIsStale } from "shared/stores"
	import { renameWorkbench } from "shared/projectUtils"

	// prettier-ignore
	const log = (function () { const context = "[BottomBar.svelte]"; const color="gray"; return Function.prototype.bind.call(console.log, console, `%c${context}`, `font-weight:bold;color:${color};`)})()
	var new_workbench_name = ""

	$: workbenches = $project.workbenches ?? []
</script>

<div class="bg-gray-100 h-[45px] flex">
	{#each workbenches as wb, i (wb.name)}
		{#if wb.renaming}
			<input
				class="bg-gray-300 text-gray-700 py-2 px-4"
				type="text"
				bind:value={new_workbench_name}
				on:blur={() => {
					log("Renaming workbench index aborted")
					wb.renaming = false
				}}
				on:keydown={(e) => {
					if (e.key === "Enter") {
						log("Renaming workbench index:", i)
						renameWorkbench(new_workbench_name)
						wb.name = new_workbench_name
						workbenchIsStale.set(true)
						wb.renaming = false
					}
				}}
			/>
		{:else}
			<button
				class="{$workbenchIndex === i ? 'bg-gray-300' : 'bg-gray-200'} hover:bg-sky-300 text-gray-700 py-2 px-4"
				type="button"
				on:dblclick={() => {
					if ($workbenchIndex !== i) {
						return
					}

					log("Renaming workbench index:", i)
					wb.renaming = true
					new_workbench_name = wb.name
				}}
				on:click={() => {
					log("Setting new workbench index:", i)
					workbenchIndex.set(i)
					workbenchIsStale.set(true)
				}}>{wb.name}</button
			>
		{/if}
	{/each}
</div>

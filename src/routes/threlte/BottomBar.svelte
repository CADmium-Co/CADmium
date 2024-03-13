<script>
	import { project, workbenchIndex, workbenchIsStale } from './stores'

	const log = (function () {
		const context = '[BottomBar.svelte]'
		return Function.prototype.bind.call(console.log, console, `%c${context}`, "font-weight:bold;color:gray;")
	})()

	$: workbenches = $project.workbenches ?? []
</script>

<div class="bg-gray-100 h-[45px] flex">
	{#each workbenches as wb, i (wb.name)}
		<button
			class="{$workbenchIndex === i
				? 'bg-gray-300'
				: 'bg-gray-200'} hover:bg-sky-300 text-gray-700 py-2 px-4"
			type="button"
			on:click={() => {
				log('Setting new workbench index:', i)
				workbenchIndex.set(i)
				workbenchIsStale.set(true)
			}}>{wb.name}</button
		>
	{/each}
</div>

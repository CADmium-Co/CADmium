<script>
	import { project, workbenchIndex } from './stores'
	$: workbenches = $project.workbenches ?? []
</script>

<div class="bg-gray-100 h-[45px] flex">
	{#each workbenches as wb, i}
		<button
			class="{$workbenchIndex === i
				? 'bg-gray-300'
				: 'bg-gray-200'} hover:bg-sky-300 text-gray-700 py-2 px-4"
			type="button">{wb.name}</button
		>
	{/each}
</div>

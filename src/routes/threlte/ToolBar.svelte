<script>
	import { sketchBeingEdited, sketchTool } from './stores'
	import { newExtrusion } from './projectUtils'

	let solving = false
	const solveSketch = () => {}
	const createNewExtrusion = () => {
		newExtrusion()
	}
	const createNewSketch = () => {}
	const stepSketch = () => {}

	let actions = [
		{
			alt: 'new sketch',
			src: '/actions/sketch_min.svg',
			text: 'New Sketch',
			handler: createNewSketch
		},
		{ alt: 'extrude', src: '/actions/extrude_min.svg', handler: createNewExtrusion },
		{ alt: 'plane', src: '/actions/plane_min.svg' }
	]

	let sketchActions = [
		{ alt: 'solve', src: '/actions/solve_min.svg', text: 'Solve', handler: solveSketch },
		{ alt: 'step', src: '/actions/step_min.svg', text: 'Step', handler: stepSketch },
		{ alt: 'line', src: '/actions/line.svg', handler: () => ($sketchTool = 'line') },
		{ alt: 'circle', src: '/actions/circle.svg', handler: () => ($sketchTool = 'circle') },
		{ alt: 'rectangle', src: '/actions/rectangle.svg', handler: () => ($sketchTool = 'rectangle') }
	]
</script>

<div class="col-span-2 flex flex-none items-center gap-1 bg-gray-100 h-[45px] select-none">
	{#if $sketchBeingEdited}
		{#each sketchActions as action}
			<button
				class="inline-flex items-center p-1 {$sketchTool === action.alt
					? 'bg-gray-400'
					: 'hover:bg-gray-200'} p-1"
				on:click={action.handler}
			>
				<img class="h-8 w-8" src={action.src} alt={action.alt} />{action.text ? action.text : ''}
			</button>
		{/each}
	{:else}
		{#each actions as action}
			<button
				class="inline-flex items-center {action.text === 'Solve' && solving
					? 'bg-gray-400'
					: ''} hover:bg-gray-200 p-1"
				on:click={action.handler}
			>
				<img class="h-8 w-8" src={action.src} alt={action.alt} />{action.text ? action.text : ''}
			</button>
		{/each}
	{/if}
</div>

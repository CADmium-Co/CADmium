<script>
	let solving = false
	const solveSketch = () => {}
	const createNewExtrusion = () => {}
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
		{ alt: 'plane', src: '/actions/plane_min.svg' },
		{ alt: 'step', src: '/actions/step_min.svg', text: 'Step', handler: stepSketch },
		{ alt: 'solve', src: '/actions/solve_min.svg', text: 'Solve', handler: solveSketch }
	]
</script>

<div class="col-span-2 flex flex-none items-center gap-1 bg-gray-100 h-[45px] select-none">
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
</div>

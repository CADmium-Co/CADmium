<script>
	let solving = false
	const solve_sketch = () => {}
	const create_new_extrusion = () => {}
	const highlightSolid = (solid_id) => {}
	const unhighlightSolid = (solid_id) => {}
	const create_new_sketch = () => {}
	const step_sketch = () => {}

	let actions = [
		{
			alt: 'new sketch',
			src: '/actions/sketch_min.svg',
			text: 'New Sketch',
			handler: create_new_sketch
		},
		{ alt: 'extrude', src: '/actions/extrude_min.svg', handler: create_new_extrusion },
		{ alt: 'plane', src: '/actions/plane_min.svg' },
		{ alt: 'step', src: '/actions/step_min.svg', text: 'Step', handler: step_sketch },
		{ alt: 'solve', src: '/actions/solve_min.svg', text: 'Solve', handler: solve_sketch }
	]
</script>

<div class="col-span-2 flex items-center gap-1 bg-gray-50 h-[45px]">
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

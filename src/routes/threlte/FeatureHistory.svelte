<script>
	import { workbench, realization } from './stores'
	import PointFeature from './PointFeature.svelte'
	import PlaneFeature from './PlaneFeature.svelte'
	import SketchFeature from './SketchFeature.svelte'
	import ExtrusionFeature from './ExtrusionFeature.svelte'
	import SolidItem from './SolidItem.svelte'

	let height = 450
	let minHeight = 30
	let maxHeight = 1200
	let initialHeight = height
	let resizing = false
	let initialPosition
	let innerWidth = 0
	let innerHeight = 0
	$: overallHeight = innerHeight > 10 ? innerHeight - 45 * 3 : 300
	$: partsHeight = overallHeight - height - 12

	$: history = $workbench.history ?? []
	$: solids = $realization.solids ?? {}

	export let setCameraFocus

	function onMouseDown(event) {
		initialPosition = { x: event.pageX, y: event.pageY }
		initialHeight = height
		resizing = true
	}

	function onMouseUp() {
		resizing = false
	}

	function onMouseMove(event) {
		if (!resizing) return

		let delta = event.pageY - initialPosition.y
		height = initialHeight + delta

		if (height < minHeight) height = minHeight
		if (height > maxHeight) height = maxHeight

		event.preventDefault()
	}
</script>

<div class="flex flex-col select-none">
	<div style="height:{Math.min(height, overallHeight - 12)}px" class="overflow-y-auto">
		<div class="font-bold text-sm px-2 py-2">History ({history.length})</div>
		{#each history as feature, featureIdx (feature.data.type + ':' + feature.unique_id)}
			<div>
				{#if feature.data.type === 'Point'}
					<PointFeature name={feature.name} index={featureIdx} />
				{:else if feature.data.type === 'Plane'}
					<PlaneFeature
						name={feature.name}
						index={featureIdx}
						plane={feature.data.plane}
						{setCameraFocus}
					/>
				{:else if feature.data.type === 'Sketch'}
					<SketchFeature name={feature.name} index={featureIdx} id={feature.unique_id} />
				{:else if feature.data.type === 'Extrusion'}
					<ExtrusionFeature name={feature.name} index={featureIdx} data={feature.data.extrusion} />
				{:else}
					TODO: {feature.name} {feature.data.type}
				{/if}
			</div>
		{/each}
	</div>
	<!-- svelte-ignore a11y-no-static-element-interactions -->
	<div class="h-[12px] cursor-row-resize border-b-2 border-b-gray-300" on:mousedown={onMouseDown} />
	<div style="height:{partsHeight}px" class="overflow-y-auto">
		<div class="font-bold text-sm px-2 py-2">
			Solids ({solids ? Object.keys(solids).length : 0})
		</div>
		{#each Object.keys(solids) as name (name)}
			<SolidItem {name} />
		{/each}
	</div>
</div>

<svelte:window on:mousemove={onMouseMove} on:mouseup={onMouseUp} bind:innerWidth bind:innerHeight />

<script lang="ts">
	import { workbench, realization } from "shared/stores"
	import PointFeature from "./features/Point.svelte"
	import PlaneFeature from "./features/Plane.svelte"
	import SketchFeature from "./features/Sketch.svelte"
	import ExtrusionFeature from "./features/Extrusion.svelte"
	import SolidItem from "./SolidItem.svelte"
	import { isPoint, isPlane, isExtrusion, isSketch } from "shared/projectUtils"
	import type { SetCameraFocus } from "shared/types"
	import FeatureInstance from "./features/FeatureInstance.svelte"

	// prettier-ignore
	const log = (function () { const context = "[FeatureHistory.svelte]"; const color="pink"; return Function.prototype.bind.call(console.log, console, `%c${context}`, `font-weight:bold;color:${color};`)})()

	const minHeight = 30
	const maxHeight = 1200
	let height = 450
	let initialHeight = height
	let resizing = false
	let initialPosition = { x: 0, y: 0 }
	let innerWidth = 0
	let innerHeight = 0
	$: overallHeight = innerHeight > 10 ? innerHeight - 45 * 3 : 300
	$: partsHeight = overallHeight - height - 12

	$: history = $workbench.history ?? []
	$: solids = $realization.solids ?? {}

	$: $workbench, log("[$workbench]", $workbench)
	$: $workbench.history, log("[$workbench.history]", $workbench.history)
	$: $realization, log("[$realization]", $realization)

	export let setCameraFocus: SetCameraFocus

	function onMouseDown(event: MouseEvent) {
		initialPosition = { x: event.pageX, y: event.pageY }
		initialHeight = height
		resizing = true
	}

	function onMouseUp() {
		resizing = false
	}

	function onMouseMove(event: MouseEvent) {
		if (!resizing) return

		const delta = event.pageY - initialPosition.y
		height = initialHeight + delta

		if (height < minHeight) height = minHeight
		if (height > maxHeight) height = maxHeight

		event.preventDefault()
	}
</script>

<div class="flex flex-col select-none">
	<div style="height:{Math.min(height, overallHeight - 12)}px" class="overflow-y-auto">
		<div id="history" class="font-bold text-sm px-2 py-2">History ({history.length})</div>
		{#each history as feature, featureIdx (feature.data.type + ":" + feature.unique_id)}
			<FeatureInstance {feature} {featureIdx} />
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

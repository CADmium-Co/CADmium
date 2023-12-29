<script>
	// The main display includes the feature history, parts manager, and 3D viewport
	// It is all contained in this file, which is imported by the main page
	// It had to be done this way to manage the resizing of the feature history and viewport
	import FeatureHistory from './FeatureHistory.svelte'
	import { Canvas } from '@threlte/core'
	import Scene from './Scene.svelte'
	import { sketchTool, selectingFor } from './stores'

	let width = 250 // px
	let minWidth = 150
	let maxWidth = 600
	let initialWidth = width
	let initialPosition
	let resizing = false
	let innerWidth = 0
	let innerHeight = 0
	$: viewportWidth = innerWidth - width - 10
	$: height = innerHeight > 135 ? innerHeight - 45 * 3 : 300

	let setCameraFocus

	function onMouseDown(event) {
		initialPosition = { x: event.pageX, y: event.pageY }
		initialWidth = width
		resizing = true
	}

	function onMouseUp(event) {
		resizing = false
	}

	function onMouseMove(event) {
		if (!resizing) return

		let delta = event.pageX - initialPosition.x
		width = initialWidth + delta

		if (width < minWidth) width = minWidth
		if (width > maxWidth) width = maxWidth

		event.preventDefault()
	}
</script>

<div style="width:{width}px; height:{height}px">
	<FeatureHistory {setCameraFocus} />
</div>
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div class="w-[12px] cursor-col-resize border-r-gray-300 border-r-2" on:mousedown={onMouseDown} />

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div
	class="bg-white {$sketchTool === 'line' || $sketchTool === 'circle' || $sketchTool === 'rectangle'
		? 'cursor-crosshair'
		: ''}"
	style="width:{viewportWidth}px; height:{height}px"
	on:mousedown={(e) => {
		if ($selectingFor.length > 0) {
			// If the user is selecting shapes, then click events on the 3D screen
			// should not steal focus away from form inputs
			e.preventDefault()
		}
	}}
>
	<Canvas>
		<Scene bind:setCameraFocus />
	</Canvas>
</div>

<svelte:window on:mousemove={onMouseMove} on:mouseup={onMouseUp} bind:innerWidth bind:innerHeight />

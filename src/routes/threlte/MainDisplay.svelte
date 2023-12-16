<script>
	import FeatureHistory from './FeatureHistory.svelte'
	import Viewport from './Viewport.svelte'
	import { Canvas } from '@threlte/core'
	import Scene from './Scene.svelte'

	let width = 250 // px
	let min_width = 150
	let max_width = 600
	let initialWidth = width
	let initialPos
	let resizing = false
	let innerWidth = 0
	let innerHeight = 0
	$: vp_width = innerWidth - width - 10
	$: height = innerHeight > 135 ? innerHeight - 45 * 3 : 300
	$: console.log(height)

	function onMouseDown(event) {
		initialPos = { x: event.pageX, y: event.pageY }
		initialWidth = width
		resizing = true
	}

	function onMouseUp(event) {
		resizing = false
	}

	function onMouseMove(event) {
		if (!resizing) return

		let delta = event.pageX - initialPos.x
		width = initialWidth + delta

		if (width < min_width) width = min_width
		if (width > max_width) width = max_width

		event.preventDefault()
	}
</script>

<div style="width:{width}px; height:{height}px">
	<FeatureHistory />
</div>
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div class="w-[12px] cursor-col-resize border-r-gray-300 border-r-2" on:mousedown={onMouseDown} />

<div class="bg-white" style="width:{vp_width}px; height:{height}px">
	<Canvas>
		<Scene />
	</Canvas>
</div>

<svelte:window on:mousemove={onMouseMove} on:mouseup={onMouseUp} bind:innerWidth bind:innerHeight />

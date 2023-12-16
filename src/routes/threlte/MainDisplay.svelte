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
	$: height = innerHeight > 135 ? innerHeight - 45 * 3 : 200
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

	function onResize(event) {
		// console.log('resize', event)
	}
</script>

<div class="bg-green-100" style="width:{width}px" />
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div class="w-[10px] cursor-col-resize bg-green-100 flex justify-end" on:mousedown={onMouseDown}>
	<div class="w-[2px] bg-gray-700" />
</div>

<div class="bg-white" style="width:{vp_width}px">
	<!-- <Canvas>
		<Scene />
	</Canvas> -->
</div>

<svelte:window
	on:resize={onResize}
	on:mousemove={onMouseMove}
	on:mouseup={onMouseUp}
	bind:innerWidth
	bind:innerHeight
/>

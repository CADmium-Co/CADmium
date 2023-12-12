<script>
	let width = 250 // px
	let min_width = 150
	let max_width = 600
	let initialWidth = width
	let initialPos
	let resizing = false

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
	}
</script>

<div class="bg-green-100" style="width:{width}px" />
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div
	class="w-[10px] cursor-col-resize bg-green-100 flex justify-end"
	on:mousedown={onMouseDown}
	on:mouseup={onMouseUp}
>
	<div class="w-[1px] bg-gray-700" />
</div>

<svelte:window on:mousemove={onMouseMove} on:mouseup={onMouseUp} />

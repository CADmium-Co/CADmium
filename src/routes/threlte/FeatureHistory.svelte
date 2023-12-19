<script>
	let height = 200
	let minHeight = 100
	let maxHeight = 1200
	let initialHeight = height
	let resizing = false
	let initialPosition
	let innerWidth = 0
	let innerHeight = 0
	$: overallHeight = innerHeight > 10 ? innerHeight - 45 * 3 : 300
	$: partsHeight = overallHeight - height - 12

	function onMouseDown(event) {
		initialPosition = { x: event.pageX, y: event.pageY }
		initialHeight = height
		resizing = true
	}

	function onMouseUp(event) {
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

<div class="flex flex-col">
	<div style="height:{Math.min(height, overallHeight - 12)}px" class="overflow-y-auto">
		Feature History that is maybe a little longer than the line is supposed to be<br />a<br />c<br
		/>c<br />c<br />c<br />c<br />c<br />c<br />c<br />c
	</div>
	<!-- svelte-ignore a11y-no-static-element-interactions -->
	<div class="h-[12px] cursor-row-resize border-b-2 border-b-gray-300" on:mousedown={onMouseDown} />
	<div style="height:{partsHeight}px" class="overflow-y-auto">
		Parts <br /> and <br /> stuff <br />a<br />a<br />a<br />a
	</div>
</div>

<svelte:window on:mousemove={onMouseMove} on:mouseup={onMouseUp} bind:innerWidth bind:innerHeight />

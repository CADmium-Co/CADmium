<script>
	let height = 200
	let min_height = 100
	let max_height = 1200
	let initialHeight = height
	let resizing = false
	let initialPos
	let innerWidth = 0
	let innerHeight = 0
	$: overall_height = innerHeight > 10 ? innerHeight - 45 * 3 : 300
	$: parts_height = overall_height - height - 12

	function onMouseDown(event) {
		initialPos = { x: event.pageX, y: event.pageY }
		initialHeight = height
		resizing = true
	}

	function onMouseUp(event) {
		resizing = false
	}

	function onMouseMove(event) {
		if (!resizing) return

		let delta = event.pageY - initialPos.y
		height = initialHeight + delta

		if (height < min_height) height = min_height
		if (height > max_height) height = max_height

		event.preventDefault()
	}
</script>

<div class="flex flex-col">
	<div style="height:{Math.min(height, overall_height - 12)}px" class="overflow-scroll">
		Feature History that is maybe a little longer than the line is supposed to be<br />a<br />c<br
		/>c<br />c<br />c<br />c<br />c<br />c<br />c<br />c
	</div>
	<!-- svelte-ignore a11y-no-static-element-interactions -->
	<div class="h-[12px] cursor-row-resize border-b-2 border-b-gray-300" on:mousedown={onMouseDown} />
	<div style="height:{parts_height}px" class="overflow-scroll">
		Parts <br /> and <br /> stuff <br />a<br />a<br />a<br />a
	</div>
</div>

<svelte:window on:mousemove={onMouseMove} on:mouseup={onMouseUp} bind:innerWidth bind:innerHeight />

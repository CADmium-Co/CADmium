<script>
	import PassiveSketch from './PassiveSketch.svelte'
	import { currentlySelected, previewGeometry, sketchTool } from './stores.js'

	export let uniqueId
	export let name
	export let sketchTuple
	export let editing
	export let plane

	export let dashedLineMaterial,
		dashedHoveredMaterial,
		solidLineMaterial,
		solidHoveredMaterial,
		solidSelectedMaterial,
		collisionLineMaterial

	function onKeyDown(event) {
		if (!editing) return

		if (event.key === 'l') {
			$sketchTool = 'line'
			$currentlySelected = []
			$previewGeometry = []
		} else if (event.key === 'r') {
			$sketchTool = 'rectangle'
			$currentlySelected = []
			$previewGeometry = []
		} else if (event.key === 'c') {
			$sketchTool = 'circle'
			$currentlySelected = []
			$previewGeometry = []
		} else if (event.key === 'Escape') {
			$sketchTool = 'select'
			$currentlySelected = []
			$previewGeometry = []
		}
	}
</script>

{#if editing}
	<PassiveSketch
		{name}
		{uniqueId}
		sketch={sketchTuple[0]}
		plane={plane.plane}
		editing
		{solidLineMaterial}
		{solidHoveredMaterial}
		{solidSelectedMaterial}
		{dashedHoveredMaterial}
		{dashedLineMaterial}
		{collisionLineMaterial}
	/>
{:else}
	<PassiveSketch
		{name}
		{uniqueId}
		sketch={sketchTuple[1]}
		plane={plane.plane}
		{solidLineMaterial}
		{solidHoveredMaterial}
		{solidSelectedMaterial}
		{dashedHoveredMaterial}
		{dashedLineMaterial}
		{collisionLineMaterial}
	/>
{/if}

<svelte:window on:keydown={onKeyDown} />

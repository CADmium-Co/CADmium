<script lang="ts">
	import type { LineMaterial } from "three/examples/jsm/lines/LineMaterial.js"
	import PassiveSketch from "./PassiveSketch.svelte"
	import { currentlySelected, previewGeometry, sketchTool } from "./stores"
	import type { PlaneRealized, SketchTuple } from "../../types"

	// prettier-ignore
	const log = (function () { const context = "[Sketch.svelte]"; const color="gray"; return Function.prototype.bind.call(console.log, console, `%c${context}`, `font-weight:bold;color:${color};`)})()

	export let uniqueId: string,
		name: string,
		sketchTuple: SketchTuple,
		editing: boolean,
		plane: PlaneRealized

	// prettier-ignore
	log("[props]", "uniqueId:", uniqueId, "name:", name, "sketchTuple", sketchTuple, "editing", editing, "plane", plane)

	export let dashedLineMaterial: LineMaterial,
		dashedHoveredMaterial: LineMaterial,
		solidLineMaterial: LineMaterial,
		solidHoveredMaterial: LineMaterial,
		solidSelectedMaterial: LineMaterial,
		collisionLineMaterial: LineMaterial

	function onKeyDown(event: KeyboardEvent) {
		if (!editing) return
		switch (event.key) {
			case "l":
				$sketchTool = "line"
				$currentlySelected = []
				$previewGeometry = []
				break
			case "r":
				$sketchTool = "rectangle"
				$currentlySelected = []
				$previewGeometry = []
				break
			case "c":
				$sketchTool = "circle"
				$currentlySelected = []
				$previewGeometry = []
				break
			case "Escape":
				$sketchTool = "select"
				$currentlySelected = []
				$previewGeometry = []
				break
			default:
				break
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

<script lang="ts">
  import type {LineMaterial} from "three/examples/jsm/lines/LineMaterial.js"
  import PassiveSketch from "./PassiveSketch.svelte"
  import {currentlySelected, previewGeometry, sketchTool} from "shared/stores"
  import type {PlaneRealized, SketchTuple} from "shared/types"

  const log = (function () { const context = "[Sketch.svelte]"; const color="gray"; return Function.prototype.bind.call(console.log, console, `%c${context}`, `font-weight:bold;color:${color};`)})() // prettier-ignore

  export let uniqueId: string, name: string, sketchTuple: SketchTuple, editing: boolean, plane: PlaneRealized

  // log("[props]", "uniqueId:", uniqueId, "name:", name, "sketchTuple", sketchTuple, "editing", editing, "plane", plane)

  export let dashedLineMaterial: LineMaterial,
    dashedHoveredMaterial: LineMaterial,
    solidLineMaterial: LineMaterial,
    solidHoveredMaterial: LineMaterial,
    solidSelectedMaterial: LineMaterial,
    collisionLineMaterial: LineMaterial

  function setTool(tool: string): void {
    $sketchTool = tool
    $currentlySelected = []
    $previewGeometry = []
  }

  function onKeyDown(event: KeyboardEvent) {
    if (!editing) return
    switch (event.key) {
      case "l":
        setTool("line")
        break
      case "r":
        setTool("rectangle")
        break
      case "c":
        setTool("circle")
        break
      case "Escape":
        setTool("select")
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

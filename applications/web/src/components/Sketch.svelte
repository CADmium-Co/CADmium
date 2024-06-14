<script lang="ts">
  import PassiveSketch from "./PassiveSketch.svelte"

  import type {LineMaterial} from "three/examples/jsm/lines/LineMaterial.js"

  import {currentlySelected, previewGeometry, sketchTool} from "shared/stores"
  import type {Face} from "shared/types"
  import type {ISketch} from "cadmium"

  // @ts-ignore
  const log = (function () { const context = "[Sketch.svelte]"; const color="gray"; return Function.prototype.bind.call(console.log, console, `%c${context}`, `font-weight:bold;color:${color};`)})() // prettier-ignore

  export let hash: string, name: string, sketch: ISketch, faces: Face[], editing: boolean = false

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

<PassiveSketch
  {name}
  {hash}
  plane={sketch.plane}
  {faces}
  editing
  {solidLineMaterial}
  {solidHoveredMaterial}
  {solidSelectedMaterial}
  {dashedHoveredMaterial}
  {dashedLineMaterial}
  {collisionLineMaterial}
/>

<svelte:window on:keydown={onKeyDown} />

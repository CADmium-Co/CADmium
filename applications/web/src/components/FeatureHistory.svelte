<script lang="ts">
  import {workbench} from "shared/stores"
  import PointFeature from "./features/Point.svelte"
  import PlaneFeature from "./features/Plane.svelte"
  import SketchFeature from "./features/Sketch.svelte"
  import ExtrusionFeature from "./features/Extrusion.svelte"
  import SolidItem from "./SolidItem.svelte"
  import {isPointStep, isPlaneStep, isSolidStep, isSketchStep, isExtrusionStep} from "shared/stepTypeGuards"
  import type {SetCameraFocus} from "shared/types"

  const log = (function () { const context = "[FeatureHistory.svelte]"; const color="pink"; return Function.prototype.bind.call(console.log, console, `%c${context}`, `font-weight:bold;color:${color};`)})() // prettier-ignore

  const minHeight = 30
  const maxHeight = 1200
  let height = 450
  let initialHeight = height
  let resizing = false
  let initialPosition = {x: 0, y: 0}
  let innerWidth = 0
  let innerHeight = 0
  $: overallHeight = innerHeight > 10 ? innerHeight - 45 * 3 : 300
  $: partsHeight = overallHeight - height - 12

  $: history = $workbench.history ?? []
  $: solids = []

  $: $workbench, log("[$workbench]", $workbench)
  $: $workbench.history, log("[$workbench.history]", $workbench.history)

  export let setCameraFocus: SetCameraFocus

  function onMouseDown(event: MouseEvent) {
    initialPosition = {x: event.pageX, y: event.pageY}
    initialHeight = height
    resizing = true
  }

  function onMouseUp() {
    resizing = false
  }

  function onMouseMove(event: MouseEvent) {
    if (!resizing) return

    const delta = event.pageY - initialPosition.y
    height = initialHeight + delta

    if (height < minHeight) height = minHeight
    if (height > maxHeight) height = maxHeight

    event.preventDefault()
  }
</script>

<div class="flex flex-col select-none dark:text-gray-300">
  <div style="height:{Math.min(height, overallHeight - 12)}px" class="overflow-y-auto">
    <div id="history" class="font-bold text-sm px-2 py-2">History ({history.length})</div>
    {#each history as step}
      <div>
        {#if isPointStep(step)}
          <PointFeature name={step.name} index={step.id} />
        {:else if isPlaneStep(step)}
          <PlaneFeature name={step.name} index={step.id} plane={step.interop_node.Plane} {setCameraFocus} />
        {:else if isSketchStep(step)}
          <SketchFeature name={step.name} index={step.id} id={`${step.id}`} plane_desc={step.data.WorkbenchSketchAdd.plane_description} />
        {:else if isExtrusionStep(step)}
          <ExtrusionFeature name={step.name} index={step.id} data={step.data} id={`${step.id}`} />
        {:else}
          TODO: {step.name} {step.interop_node}
        {/if}
      </div>
    {/each}
  </div>
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <div class="h-[12px] cursor-row-resize border-b-2 border-b-gray-300" on:mousedown={onMouseDown}></div>
  <div style="height:{partsHeight}px" class="overflow-y-auto">
    <div class="font-bold text-sm px-2 py-2">
      Solids ({solids ? Object.keys(solids).length : 0})
    </div>
    {#each history as step}
      {#if isSolidStep(step)}
        <SolidItem name={step.name} />
      {/if}
    {/each}
  </div>
</div>

<svelte:window on:mousemove={onMouseMove} on:mouseup={onMouseUp} bind:innerWidth bind:innerHeight />

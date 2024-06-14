<script>
  import {currentlySelected, currentlyMousedOver, selectingFor, featureIndex, sketchBeingEdited, sketchTool, workbench, hiddenSketches} from "shared/stores"
  import {newExtrusion, newSketchOnPlane} from "shared/projectUtils"
  import {base} from "../base"

  const log = (function () { const context = "[ToolBar.svelte]"; const color="gray"; return Function.prototype.bind.call(console.log, console, `%c${context}`, `font-weight:bold;color:${color};`)})() // prettier-ignore

  let solving = false
  // todo ask Matt why is this a no-op?
  const solveSketch = () => {}
  const createNewExtrusion = () => {
    newExtrusion()
    // set that as the current feature being edited
    $featureIndex = $workbench.history.length - 1
  }
  const createNewSketch = () => {
    // log('Create new sketch')
    newSketchOnPlane()
    $featureIndex = $workbench.history.length - 1
  }
  const stepSketch = () => {}
  const debugging = false

  const actions = [
    {
      alt: "new sketch",
      src: `${base}/actions/sketch_min.svg`,
      text: "New Sketch",
      handler: createNewSketch,
    },
    {alt: "extrude", src: `${base}/actions/extrude_min.svg`, handler: createNewExtrusion},
    // { alt: 'plane', src: '/actions/plane_min.svg' }
  ]

  const sketchActions = [
    {alt: "solve", src: `${base}/actions/solve_min.svg`, text: "Solve", handler: solveSketch},
    {alt: "step", src: `${base}/actions/step_min.svg`, text: "Step", handler: stepSketch},
    {alt: "line", src: `${base}/actions/line.svg`, handler: () => ($sketchTool = "line")},
    {alt: "circle", src: `${base}/actions/circle.svg`, handler: () => ($sketchTool = "circle")},
    {alt: "rectangle", src: `${base}/actions/rectangle.svg`, handler: () => ($sketchTool = "rectangle")},
  ]
</script>

<div class="col-span-2 flex flex-none items-center gap-1 bg-gray-100 dark:bg-gray-800 dark:text-gray-300 h-[45px] select-none">
  {#if $sketchBeingEdited !== ""}
    {#each sketchActions as action}
      <button class="inline-flex items-center p-1 {$sketchTool === action.alt ? 'bg-gray-400' : 'hover:bg-gray-200'} p-1" on:click={action.handler}>
        <img class="h-8 w-8" src={action.src} alt={action.alt} />{action.text ? action.text : ""}
      </button>
    {/each}
  {:else}
    {#each actions as action}
      <button class="inline-flex items-center hover:bg-gray-200 p-1 rounded dark:hover:bg-gray-600 dark:hover:text-gray-300" on:click={action.handler}>
        <img class="h-8 w-8" src={action.src} alt={action.alt} />{action.text ? action.text : ""}
      </button>
    {/each}
  {/if}

  {#if debugging}
    Selecting For [
    {#each $selectingFor as sf}
      <div>
        {sf},
      </div>
    {/each}
    ] Currently Selected [
    {#each $currentlySelected as cs}
      <div>
        {cs.type}
        {cs.id},
      </div>
    {/each}
    ] Moused Over [
    {#each $currentlyMousedOver as cm}
      <div>
        {cm.type}
        {cm.id},
      </div>
    {/each}
    ] Hidden Sketches [
    {#each $hiddenSketches as hs}
      <div>
        {hs},
      </div>
    {/each}
    ]
  {/if}
</div>

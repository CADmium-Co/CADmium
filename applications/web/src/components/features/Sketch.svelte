<script lang="ts">
  import {slide} from "svelte/transition"
  import {quintOut} from "svelte/easing"
  import {renameStep, setSketchPlane} from "shared/projectUtils"
  import {
    hiddenSketches,
    featureIndex,
    selectionMax,
    selectionMin,
    currentlySelected,
    selectingFor,
    sketchBeingEdited,
    sketchTool,
    currentlyMousedOver,
  } from "shared/stores"
  import EyeSlash from "phosphor-svelte/lib/EyeSlash"
  import Eye from "phosphor-svelte/lib/Eye"
  import X from "phosphor-svelte/lib/X"
  import type {Entity} from "shared/types"
  import {base} from "../../base"

  const log = (function () { const context = "[SketchFeature.svelte]"; const color="gray"; return Function.prototype.bind.call(console.log, console, `%c${context}`, `font-weight:bold;color:${color};`)})() // prettier-ignore

  export let name: string, index: number, id: string, plane_id: string

  // $: name, log("[props] name:", name, "index:", index, "id:", id, "plane_id:", plane_id)

  const source = `${base}/actions/sketch_min.svg`

  let surface: Entity | null = null
  let selectingForSketchPlane = false

  $: {
    if (plane_id !== "") {
      surface = {type: "plane", id: plane_id}
    } else {
      surface = null
      engageSearchForPlane()
    }
  }

  // $: $featureIndex, log("[$featureIndex]", typeof $featureIndex, $featureIndex)

  const closeAndRefresh = () => {
    log("closing, refreshing")
    $featureIndex = 1000
    $sketchBeingEdited = ""
    $sketchTool = ""
    $selectingFor = []
    $selectionMax = 1000
    $selectionMin = 0
    $currentlySelected = []
  }

  $: if ($featureIndex === index) $sketchBeingEdited = id

  // $: $sketchBeingEdited,
  // 	log("[$sketchBeingEdited]", `${$sketchBeingEdited === "" ? "empty" : ""}`, $sketchBeingEdited)

  const engageSearchForPlane = () => {
    // log("engage search!")
    $sketchTool = ""
    $selectingFor = ["plane", "meshFace"]
    $selectionMax = 1
    $selectionMin = 1

    if (surface !== null) {
      $currentlySelected = [surface]
    }
    selectingForSketchPlane = true
    // log("search is engaged")
  }

  const disengageSearchForPlane = () => {
    // log("Disengage search!")
    $currentlySelected = []
    $selectingFor = []
    $selectionMax = 1000
    $selectionMin = 0
    selectingForSketchPlane = false
    $sketchTool = "select"
    $currentlyMousedOver = []
    // log("search is disengaged")
  }

  currentlySelected.subscribe(() => {
    if (!selectingForSketchPlane) return
    if (!id) return
    if (!$currentlySelected.length) return
    // log("CS changed when selecting for Sketch Plane:", $currentlySelected)

    let thingSelected = $currentlySelected[0]
    if (thingSelected.type === "plane") {
      setSketchPlane(id, thingSelected.id)
    } else if (thingSelected.type === "meshFace") {
      log("HOW DO I HANDLE THIS?")
      log(thingSelected)
      // setSketchPlane(id, $currentlySelected[0].id)
    }

    disengageSearchForPlane()
  })
</script>

<div
  class="flex items-center text-sm hover:bg-sky-200 dark:hover:bg-gray-600"
  role="button"
  tabindex="0"
  on:dblclick={() => {
    if ($featureIndex === index) {
      closeAndRefresh()
    } else {
      $featureIndex = index
      $sketchTool = "select"
    }
  }}
>
  {#if $featureIndex < index}
    <img class="h-8 w-8 px-1 opacity-50" src={source} alt={name} />
    <span class="italic opacity-50">{name}</span>
  {:else}
    <img class="h-8 w-8 px-1" src={source} alt={name} />
    <span>{name}</span>
  {/if}

  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <div
    class="ml-auto mr-2 bg-gray-100 hover:bg-gray-200 dark:bg-gray-800 dark:hover:bg-gray-500 px-1 py-1 rounded"
    on:click={() => {
      if ($hiddenSketches.includes(id)) {
        // cool, unhide
        hiddenSketches.update(sketches => {
          return sketches.filter(sketch => sketch !== id)
        })
      } else {
        // cool, hide
        hiddenSketches.update(sketches => {
          return [...sketches, id]
        })
      }
    }}
  >
    {#if $hiddenSketches.includes(id)}
      <EyeSlash weight="light" size="18px" />
    {:else}
      <Eye weight="light" size="18px" />
    {/if}
  </div>
</div>

{#if $featureIndex === index}
  <div transition:slide={{delay: 0, duration: 400, easing: quintOut, axis: "y"}}>
    <form
      on:submit|preventDefault={() => {
        // editing = false
        closeAndRefresh()
      }}
      class="px-3 py-2 bg-gray-100 dark:bg-gray-600 flex flex-col space-y-2"
      autocomplete="off"
    >
      <label>
        Name
        <input
          autocomplete="off"
          data-1p-ignore
          class="shadow appearance-none border w-full py-2 px-3 text-gray-700 leading-tight focus:border focus:border-sky-500"
          bind:value={name}
        />
      </label>

      <!-- svelte-ignore a11y-no-noninteractive-tabindex -->
      Surface
      <div
        tabindex="0"
        class="bg-gray-50 rounded flex shadow border focus:ring focus:border-blue-500 min-h-8 flex-wrap"
        on:focusin={engageSearchForPlane}
        on:focusout={disengageSearchForPlane}
      >
        <div class="h-8" />
        {#if surface !== null}
          <div class="bg-sky-200 pl-2 py-0.5 m-1 rounded text-sm">
            {surface.type}:{surface.id}<button
              on:click|preventDefault={() => {
                surface = null
              }}><X /></button
            >
          </div>
        {/if}
      </div>

      <div class="flex space-x-1.5">
        <button
          class="flex-grow bg-sky-500 hover:bg-sky-700 text-white font-bold py-1.5 px-1 shadow"
          on:click={() => {
            // This is a form button so remember that it triggers the form's on:submit
            renameStep(index, name)
          }}>Done</button
        >

        <button class="bg-transparent hover:bg-sky-700 text-sky-500 font-semibold hover:text-white py-1.5 px-4 border border-sky-500 hover:border-transparent"
          >Cancel</button
        >
      </div>
    </form>
  </div>
{/if}

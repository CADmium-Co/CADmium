<script lang="ts">
  import {slide} from "svelte/transition"
  import {quintOut} from "svelte/easing"
  import {arraysEqual, renameStep, updateExtrusion} from "shared/projectUtils"
  import {selectingFor, workbenchIsStale, featureIndex, currentlySelected, hiddenSketches} from "shared/stores"
  import X from "phosphor-svelte/lib/X"
  import type {ExtrusionData} from "shared/types"
  import {base} from "../../base"

  const log = (function () { const context = "[ExtrusionFeature.svelte]"; const color="gray"; return Function.prototype.bind.call(console.log, console, `%c${context}`, `font-weight:bold;color:${color};`)})() // prettier-ignore

  export let name: string, index: number, id: string, data: ExtrusionData["data"]["extrusion"]

  // $: data, log("[props]", "name:", name, "index:", index, "id:", id, "data:", data)
  // $: data, log("[props]", "typeof id:", typeof id, "id:", id)
  // $: data, log("[props]", "typeof data.face_ids[0]:", typeof data.face_ids[0], "data.face_ids:", data.face_ids)

  // coerce from number[] to string[] for frontend as we use strings for ids here
  let faceIdsFromInputs = data.face_ids.sort().map(e => e + "")

  // reactive update of selected faces
  $: if (data && data.face_ids) faceIdsFromInputs = data.face_ids.map(e => e + "").sort()

  let length = data.length

  const closeAndRefresh = () => {
    // log("[closeAndRefresh] extrusion feature closing")
    $featureIndex = 1000
    $currentlySelected = []
    $selectingFor = []
    // hide the sketch that this extrusion uses
    if (!$hiddenSketches.includes(data.sketch_id)) {
      // log("[closeAndRefresh] Oh, we're hiding the sketch that this extrusion uses")
      $hiddenSketches = [...$hiddenSketches, data.sketch_id]
    }

    workbenchIsStale.set(true)
  }

  function sendUpdate(specificFaceIds?: string[]) {
    if (specificFaceIds) {
      updateExtrusion(id, data.sketch_id, length, specificFaceIds)
    } else {
      const faceIdsFromSelection = $currentlySelected
        .filter(e => e.type === "face")
        .map(e => e.id)
        .sort()
      updateExtrusion(id, data.sketch_id, length, faceIdsFromSelection)
    }
  }

  currentlySelected.subscribe(store => {
    if ($featureIndex !== index) return

    const faceIdsFromSelection = store
      .filter(e => e.type === "face")
      .map(e => e.id)
      .sort()

    if (arraysEqual(faceIdsFromInputs, faceIdsFromSelection)) {
      // log("[closeAndRefresh] face ids are the same, no update")
    } else {
      // log("[closeAndRefresh] triggering update to new face Ids:", faceIdsFromSelection)
      sendUpdate(faceIdsFromSelection)
    }
  })

  // $: log($currentlySelected)
  // $: faceIds = $currentlySelected.filter((e) => e.type === 'face').map((e) => e.id)

  const source = `${base}/actions/extrude_min.svg`

  $: if ($featureIndex === index) {
    $selectingFor = ["face"]
    $currentlySelected = faceIdsFromInputs.map(id => ({type: "face", id}))
    // log("[$currentlySelected]", $currentlySelected)
  }
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
      // $selectingFor = []
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
</div>

{#if $featureIndex === index}
  <div transition:slide={{delay: 0, duration: 400, easing: quintOut, axis: "y"}}>
    <form
      on:submit|preventDefault={() => {
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

      <label>
        Length
        <input
          autocomplete="off"
          data-1p-ignore
          class="shadow appearance-none border w-full py-2 px-3 text-gray-700 leading-tight focus:border focus:border-sky-500"
          type="number"
          bind:value={length}
          on:input={() => {
            sendUpdate()
          }}
        />
      </label>

      Faces
      <!-- svelte-ignore a11y-no-noninteractive-tabindex -->
      <div tabindex="0" class="bg-gray-50 rounded flex shadow border focus:ring focus:border-blue-500 min-h-8 flex-wrap">
        <div class="h-8"></div>
        {#each faceIdsFromInputs as faceId}
          <div class="bg-sky-200 pl-2 py-0.5 m-1 rounded text-sm">
            {faceId}<button
              on:click|preventDefault={() => {
                $currentlySelected = $currentlySelected.filter(item => !(item.id === faceId && item.type === "face"))
              }}><X /></button
            >
          </div>
        {/each}
      </div>

      <div class="flex space-x-1.5">
        <button
          class="flex-grow bg-sky-500 hover:bg-sky-700 text-white font-bold py-1.5 px-1 shadow"
          on:click={() => {
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

<script lang="ts">
  // import { browser } from "$app/environment"
  import {onMount} from "svelte"
  import {default as init, Project as WasmProject} from "cadmium"
  import AppBar from "./components/AppBar.svelte"
  import BottomBar from "./components/BottomBar.svelte"
  import MainDisplay from "./components/MainDisplay.svelte"
  import ToolBar from "./components/ToolBar.svelte"
  import {workbenchIsStale, wasmProject, project, projectIsStale, featureIndex} from "shared/stores"

  // prettier-ignore
  const log = (function () { const context = "[+page.svelte]"; const color="gray"; return Function.prototype.bind.call(console.log, console, `%c${context}`, `font-weight:bold;color:${color};`)})()

  const userName = "mattferraro.dev"
  let newFileContent: string | null = null

  // if (browser) {
  onMount(() => {
    init().then(() => {
      wasmProject.set(new WasmProject("First Project"))
      // log('made a new project')
      projectIsStale.set(true)
    })
  })
  // }

  // $: $wasmProject, log("[$wasmProject]", $wasmProject)
  // $: $project, log("[$project]", $project)

  $: if (newFileContent) {
    log("[newFileContent] received new file", newFileContent)
    const newWasmProject = WasmProject.from_json(newFileContent)
    wasmProject.set(newWasmProject)
    projectIsStale.set(true)
    newFileContent = null
  }

  // log('featureIndex changed to', val)
  // refresh workbench when featureIndex mutates
  featureIndex.subscribe(val => $wasmProject["get_workbench"] && workbenchIsStale.set(true))
</script>

<div class="w-[100vw] h-[100vh] block" style="overflow: hidden;">
  <AppBar {userName} project={$project} bind:newFileContent />
  <ToolBar />
  <div class="flex">
    <MainDisplay />
  </div>
  <BottomBar />
</div>

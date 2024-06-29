<script lang="ts">
  // import { browser } from "$app/environment"
  import {onMount} from "svelte"
  import * as cadmium from "cadmium"
  import AppBar from "./components/AppBar.svelte"
  import BottomBar from "./components/BottomBar.svelte"
  import MainDisplay from "./components/MainDisplay.svelte"
  import ToolBar from "./components/ToolBar.svelte"
  import {projectIndex} from "shared/stores"

  // prettier-ignore
  const log = (function () { const context = "[+page.svelte]"; const color="gray"; return Function.prototype.bind.call(console.log, console, `%c${context}`, `font-weight:bold;color:${color};`)})()

  let newFileContent: string | null = null

  onMount(() => {
    cadmium.default().then(() => {
      projectIndex.set(cadmium.create_project("First Project"))
    })
  })

  $: if (newFileContent) {
    log("[newFileContent] received new file", newFileContent)
    projectIndex.set(cadmium.load_project_from_str(newFileContent))
    newFileContent = null
  }
</script>

<div class="w-[100vw] h-[100vh] block" style="overflow: hidden;">
  <AppBar bind:newFileContent />
  <ToolBar />
  <div class="flex">
    <MainDisplay />
  </div>
  <BottomBar />
</div>

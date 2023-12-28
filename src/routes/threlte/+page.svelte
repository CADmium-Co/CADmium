<script>
	import { browser } from '$app/environment'
	import { onMount } from 'svelte'

	import { default as init, Project } from 'cadmium'

	import AppBar from './AppBar.svelte'
	import BottomBar from './BottomBar.svelte'
	import MainDisplay from './MainDisplay.svelte'
	import ToolBar from './ToolBar.svelte'
	import { workbenchIsStale, wasmProject, project, projectIsStale, featureIndex } from './stores.js'

	let userName = 'mattferraro.dev'
	let newFileContent = null

	if (browser) {
		onMount(() => {
			init().then(() => {
				let p = new Project('First Project')
				wasmProject.set(p)
				// console.log('made a new project')
				projectIsStale.set(true)
			})
		})
	}

	$: if (newFileContent) {
		// console.log('received new file')
		let newWasmProject = Project.from_json(newFileContent)
		wasmProject.set(newWasmProject)
		projectIsStale.set(true)
		newFileContent = null
	}

	featureIndex.subscribe((val) => {
		if ($wasmProject.get_workbench) {
			// console.log('featureIndex changed to', val)
			workbenchIsStale.set(true)
		}
	})
</script>

<div class="w-[100vw] h-[100vh] block">
	<AppBar {userName} project={$project} bind:newFileContent />
	<ToolBar />
	<div class="flex">
		<MainDisplay />
	</div>
	<BottomBar />
</div>

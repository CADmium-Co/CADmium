<script>
	import { browser } from '$app/environment'
	import { onMount } from 'svelte'

	import { default as init, Project } from 'cadmium'

	import AppBar from './AppBar.svelte'
	import BottomBar from './BottomBar.svelte'
	import MainDisplay from './MainDisplay.svelte'
	import ToolBar from './ToolBar.svelte'
	import {
		wasmProject,
		project,
		realization,
		wasmRealization,
		workbench,
		workbenchIndex,
		featureIndex,
		realizationIsStale
	} from './stores.js'

	let userName = 'mattferraro.dev'

	if (browser) {
		onMount(() => {
			init().then(() => {
				let p = new Project('First Project')
				wasmProject.set(p)
				realizationIsStale.set(true)
			})
		})
	}

	$: if ($realizationIsStale) {
		console.log('getting new realization')
		project.set(JSON.parse($wasmProject.json))
		workbenchIndex.set(0)
		workbench.set($project.workbenches[$workbenchIndex])

		const maxStep = $featureIndex >= 0 ? $featureIndex + 1 : 1000
		wasmRealization.set($wasmProject.get_realization(0, maxStep))
		realization.set(JSON.parse($wasmRealization.to_json()))
		realizationIsStale.set(false)
	}
</script>

<div class="w-[100vw] h-[100vh] block">
	<AppBar {userName} project={$project} />
	<ToolBar />
	<div class="flex">
		<MainDisplay />
	</div>
	<BottomBar />
</div>

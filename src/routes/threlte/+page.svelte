<script>
	import { browser } from '$app/environment'
	import { onMount } from 'svelte'

	import { default as init, Project } from 'cadmium'

	import AppBar from './AppBar.svelte'
	import BottomBar from './BottomBar.svelte'
	import MainDisplay from './MainDisplay.svelte'
	import ToolBar from './ToolBar.svelte'
	import { wasmProject, project, projectIsStale } from './stores.js'

	let userName = 'mattferraro.dev'

	if (browser) {
		onMount(() => {
			init().then(() => {
				let p = new Project('First Project')
				wasmProject.set(p)
				projectIsStale.set(true)
			})
		})
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

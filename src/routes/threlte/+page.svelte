<script>
	import AppBar from './AppBar.svelte'
	import BottomBar from './BottomBar.svelte'
	import FeatureHistory from './FeatureHistory.svelte'
	import MainDisplay from './MainDisplay.svelte'
	import ToolBar from './ToolBar.svelte'
	import Viewport from './Viewport.svelte'

	let username = 'mattferraro.dev'
	let project = { name: 'Threlte Project' }
</script>

<div class="w-[100vw] h-[100vh] block">
	<AppBar {username} {project} />
	<ToolBar />
	<div class="flex">
		<MainDisplay />
	</div>
	<BottomBar />
</div>

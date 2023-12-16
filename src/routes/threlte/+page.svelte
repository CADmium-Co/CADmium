<script>
	import AppBar from './AppBar.svelte'
	import FeatureHistory from './FeatureHistory.svelte'
	import MainDisplay from './MainDisplay.svelte'
	import ToolBar from './ToolBar.svelte'
	import Viewport from './Viewport.svelte'

	let username = 'mattferraro.dev'
	let project = { name: 'Threlte Project' }
</script>

<div class="w-[100vw] h-[100vh] bg-slate-500 flex flex-col">
	<AppBar {username} {project} />
	<ToolBar />
	<div class="flex flex-1">
		<MainDisplay />
	</div>
	<div class="bg-gray-500 h-[45px] flex-none">Bottom Bar</div>
</div>

<script>
	import { Canvas } from '@threlte/core'
	import Scene from './Scene.svelte'

	function onResize(event) {
		console.log(event)
	}

	let c_width, c_height

	// $: console.log(c_width)
</script>

<div class="bg-red-200 flex-grow" bind:clientWidth={c_width} bind:clientHeight={c_height}>
	<Canvas>
		<Scene />
	</Canvas>
</div>

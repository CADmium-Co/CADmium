<script>
	export let realization
	import { onMount } from 'svelte'
	import { browser } from '$app/environment'
	import { createScene, setRealization, setCameraViewPlane, setOutlined } from '$lib/scene.js'

	import { outlined_solids, sketch_being_edited } from './stores'

	let el
	if (browser) {
		onMount(async () => {
			// console.log('onMount', el)
			createScene(el)
		})
	}

	$: if (realization && realization.planes && el) {
		setRealization(realization, $sketch_being_edited)
	}

	export function setCameraViewPlane2(plane) {
		console.log('Setting view plane: ', plane)
		setCameraViewPlane(plane)
	}

	$: if (outlined_solids) {
		setOutlined($outlined_solids)
	}

	const onResize = (e) => {
		// TODO: for some reason the canvas itself never gets resized...gotta fix this
		console.log('Resized:', e)
	}
</script>

<div class="h-[100%]">
	<canvas class="w-[100%]" bind:this={el} on:resize={(e) => onResize(e)} />
</div>

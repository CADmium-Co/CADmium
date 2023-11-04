<script>
	export let realization
	import { onMount } from 'svelte'
	import { browser } from '$app/environment'
	import { createScene, setRealization, setCameraViewPlane, setOutlined } from '$lib/scene.js'

	import { outlined_solids } from './stores'

	let el
	if (browser) {
		onMount(async () => {
			// console.log('onMount', el)
			createScene(el)
		})
	}

	$: if (realization && realization.planes && el) {
		setRealization(realization)
	}

	export function setCameraViewPlane2(plane) {
		console.log('Setting view plane: ', plane)
		setCameraViewPlane(plane)
	}

	$: if (outlined_solids) {
		setOutlined($outlined_solids)
	}
</script>

<div class="h-[100%]">
	<canvas class="w-[100%]" bind:this={el} />
</div>

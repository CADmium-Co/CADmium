<script>
	export let realization
	import { onMount } from 'svelte'
	import { browser } from '$app/environment'
	import { createScene, setRealization } from '$lib/scene.js'

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
</script>

<div class="h-[100%]">
	<canvas class="w-[100%]" bind:this={el} />
</div>

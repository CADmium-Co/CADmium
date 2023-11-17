<script>
	export let realization
	import { onMount } from 'svelte'
	import { browser } from '$app/environment'
	import {
		set_looking_for,
		createScene,
		setRealization,
		setCameraViewPlane,
		setOutlined,
		setOnFound
	} from '$lib/scene.js'

	import { looking_for, found, outlined_solids, sketch_being_edited } from './stores'

	const on_found = (item) => {
		console.log('found:', item)
		found.set([item])
	}

	let el
	if (browser) {
		onMount(async () => {
			// console.log('onMount', el)
			createScene(el)
			setOnFound(on_found)
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

	$: set_looking_for($looking_for)
</script>

<div class="h-[100%]">
	<canvas class="w-[100%]" bind:this={el} on:resize={(e) => onResize(e)} />
</div>

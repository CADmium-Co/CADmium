<script>
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { createScene } from '$lib/scene.js';

	// import init from '../../rust/cadmium/pkg/cadmium_bg.wasm?init';

	let el;
	if (browser) {
		onMount(async () => {
			// init().then((instance) => {
			// 	const result = instance.exports.add(2, 3);
			// 	console.log('result: ' + result);
			// });

			createScene(el);
		});
	}
</script>

<div class="h-[100%]">
	<canvas class="w-[100%]" bind:this={el} />
</div>

<script>
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { createScene } from '$lib/scene.js';

	let el;
	if (browser) {
		onMount(() => {
			createScene(el);
		});
	}
</script>

<div class="main">
	<div>Hello Canvas</div>
	<canvas bind:this={el} />
</div>

<style>
	canvas {
		width: 80%;
		height: 80%;
		border: 1px solid black;
	}
	.main {
		background-color: #f0f0f0;
		padding: 20px;
		margin: 20px;
		border-radius: 5px;
	}
</style>

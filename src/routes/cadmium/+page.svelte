<script>
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { createScene } from '$lib/scene.js';

	// import init from '../../wasm/cadmium_bg.wasm?init';
	// import init from '../../rust/cadmium/pkg/cadmium_bg.wasm?init';
	import init from '../../rust/cadmium/pkg/cadmium_bg.wasm?init';
	// import init from '../../../cadmium/pkg/cadmium_bg.wasm?init';

	let el;
	if (browser) {
		onMount(async () => {
			init().then((instance) => {
				const result = instance.exports.add(2, 3);
				console.log('result: ' + result);
			});

			createScene(el);
		});
	}
</script>

<div class="main">
	<div class="header">CADmium</div>
	<div class="row-2">
		<div class="side" />
		<div>
			<canvas bind:this={el} />
		</div>
	</div>
	<div class="footer" />
</div>

<style>
	canvas {
		width: calc(100vw - 300px);
		height: 100%;
		margin: 0px;
		padding: 0px;
	}
	.main {
		background-color: #ddd;
		padding: 0px;
		margin: 0px;
		display: flex;
		flex-direction: column;
	}
	.header {
		font-size: 2em;
		font-weight: bold;
		height: 50px;
	}
	.row-2 {
		display: flex;
		flex-direction: row;
		height: calc(100vh - 100px);
		margin: 0px;
		padding: 0px;
	}
	.side {
		width: 300px;
		height: 100%;
	}
	.footer {
		height: 50px;
	}
</style>

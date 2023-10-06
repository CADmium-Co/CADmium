<script>
	import MainCanvas from './mainCanvas.svelte';
	import { browser } from '$app/environment';
	import { onMount } from 'svelte';
	import { project } from './stores.js';
	// import init from '../../rust/cadmium/pkg/cadmium_bg.wasm?init';
	import { default as init, Project } from 'cadmium';

	if (browser) {
		onMount(() => {
			init().then((instance) => {
				const p = new Project('First Project');
				// console.log(p);
				// console.log(p.name);
				project.set(p);
			});
		});
	}

	let username = 'mattferraro.dev';

	console.log($project);

	let actions = [
		{ alt: 'new sketch', src: '/actions/sketch_min.svg', text: 'New Sketch' },
		{ alt: 'extrude', src: '/actions/extrude_min.svg' },
		{ alt: 'chamfer', src: '/actions/chamfer_min.svg' },
		{ alt: 'hole', src: '/actions/hole_min.svg' },
		{ alt: 'fillet', src: '/actions/fillet_min.svg' },
		{ alt: 'revolve', src: '/actions/revolve_min.svg' }
	];
</script>

<div id="container" class="bg-gray-50 grid grid-cols-editor grid-rows-editor h-[100vh]">
	<header class="col-span-2 bg-gray-100">
		<div class="flex items-center gap-4">
			<div class="shrink-0 select-none">
				<img class="object-cover h-10 w-10 ml-4" alt="logo" src="/cadmium_logo_min.svg" />
			</div>
			<div class="select-none">CADmium</div>
			<div class="text-xl font-medium">{$project.name || ''}</div>

			<div class="flex-grow flex flex-row-reverse gap-4 mr-4">
				<div>
					<a href="https://github.com/mattferraro/cadmium"
						><img class="h-6 w-6" src="/github-mark.svg" alt="github logo" /></a
					>
				</div>
				<div>{username}</div>
			</div>
		</div>
	</header>
	<toolbar class="col-span-2 flex items-center gap-1">
		{#each actions as action}
			<button class="inline-flex items-center hover:shadow-md p-1 rounded focus:bg-gray-400">
				<img class="h-8 w-8" src={action.src} alt={action.alt} />{action.text ? action.text : ''}
			</button>
		{/each}
	</toolbar>
	<aside class="bg-gray-100">
		<div class="flex">
			<div>History</div>
		</div>
	</aside>
	<main class="h-[100%]">
		<MainCanvas />
	</main>
	<footer class="col-span-2">Footer</footer>
</div>

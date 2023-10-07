<script>
	import MainCanvas from './mainCanvas.svelte';
	import { browser } from '$app/environment';
	import { onMount } from 'svelte';
	import { project_rust, project, active_workbench_index, workbench } from './stores.js';
	// import init from '../../rust/cadmium/pkg/cadmium_bg.wasm?init';
	import { default as init, Project } from 'cadmium';

	let num_steps_applied = 1000;
	let realization = {};

	if (browser) {
		onMount(() => {
			init().then(() => {
				let p = new Project('First Project');
				project_rust.set(p);
				project.set(JSON.parse(p.json));
				active_workbench_index.set(0);
			});
		});
	}

	let username = 'mattferraro.dev';

	$: if ($project && $project.workbenches) {
		workbench.set($project.workbenches[$active_workbench_index]);
		console.log('WB: ', $workbench);
		let realization = $project_rust.get_realization(0, 1000);
		console.log('Realization:', realization);
	}

	// $: realization = $project_rust.get_realization(0, num_steps_applied);
	// $: console.log('realization: ', realization);

	let actions = [
		{ alt: 'new sketch', src: '/actions/sketch_min.svg', text: 'New Sketch' },
		{ alt: 'extrude', src: '/actions/extrude_min.svg' },
		{ alt: 'plane', src: '/actions/plane_min.svg' }
		// { alt: 'chamfer', src: '/actions/chamfer_min.svg' },
		// { alt: 'hole', src: '/actions/hole_min.svg' },
		// { alt: 'fillet', src: '/actions/fillet_min.svg' },
		// { alt: 'revolve', src: '/actions/revolve_min.svg' }
	];

	let icon_mapping = {
		Sketch: '/actions/sketch_min.svg',
		Plane: '/actions/plane_min.svg'
	};
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
		<div class="flex flex-col select-none">
			<div class="font-bold text-sm px-2 py-2">History ({$workbench.history.length})</div>
			<div>
				{#each $workbench.history as item}
					<div class="flex items-center text-sm">
						<img class="h-8 w-8 px-1" src={icon_mapping[item.data.type]} alt={item.name} />
						{item['name']}
					</div>
				{/each}
			</div>
		</div>
	</aside>
	<main class="h-[100%]">
		<MainCanvas />
	</main>
	<footer class="col-span-2">Footer</footer>
</div>

<script>
	import MainCanvas from './mainCanvas.svelte'
	import { browser } from '$app/environment'
	import { onMount } from 'svelte'
	import { project_rust, project, active_workbench_index, workbench } from './stores.js'
	// import init from '../../rust/cadmium/pkg/cadmium_bg.wasm?init';
	import { default as init, Project } from 'cadmium'

	let num_steps_applied = 1000
	let realization = {}

	let current_step = {}

	if (browser) {
		onMount(() => {
			init().then(() => {
				let p = new Project('First Project')
				project_rust.set(p)
				project.set(JSON.parse(p.json))
				active_workbench_index.set(0)
			})
		})
	}

	let username = 'mattferraro.dev'

	$: if ($project && $project.workbenches) {
		workbench.set($project.workbenches[$active_workbench_index])
		realization = JSON.parse($project_rust.get_realization(0, 1000))
		
	}

	const create_new_sketch = () => {
		// let messages = [
		// 	{"NewPointOnSketch":{"workbench_id":0,"sketch_name":"Sketch 1","point_id":100,"x":-0.6,"y":0.5}},
		// 	{"NewPointOnSketch":{"workbench_id":0,"sketch_name":"Sketch 1","point_id":101,"x":-0.2,"y":0.4}},
		// 	{"NewPointOnSketch":{"workbench_id":0,"sketch_name":"Sketch 1","point_id":102,"x":-0.2,"y":0.1}},
		// 	{"NewPointOnSketch":{"workbench_id":0,"sketch_name":"Sketch 1","point_id":103,"x":-0.6,"y":0.1}},
		// 	{"NewLineOnSketch":{"workbench_id":0,"sketch_name":"Sketch 1","line_id":100,"start_point_id":100,"end_point_id":101}},
		// 	{"NewLineOnSketch":{"workbench_id":0,"sketch_name":"Sketch 1","line_id":101,"start_point_id":101,"end_point_id":102}},
		// 	{"NewLineOnSketch":{"workbench_id":0,"sketch_name":"Sketch 1","line_id":102,"start_point_id":102,"end_point_id":103}},
		// 	{"NewLineOnSketch":{"workbench_id":0,"sketch_name":"Sketch 1","line_id":103,"start_point_id":103,"end_point_id":100}},
		// ]

		// let overall_success = true
		// for (let message_obj of messages) {
		// 	let result = $project_rust.send_message(JSON.stringify(message_obj))
		// 	if (result === "success") {
		// 		console.log("success of message: ", result)
		// 	} else {
		// 		console.log("failure of message: ", result)
		// 		overall_success = false
		// 		break
		// 	}
		// }
	}

	const step_sketch = () => {
		let message_obj = {"StepSketch": {"workbench_id": 0, "sketch_name": "Sketch 1", "steps": 1}}
		let result = $project_rust.send_message(JSON.stringify(message_obj))
		console.log("result of message: ", result)
		
		project.set(JSON.parse($project_rust.json))
	}
	const create_new_extrusion = () => {
		console.log('okay!')
	}

	let actions = [
		{ alt: 'new sketch', src: '/actions/sketch_min.svg', text: 'New Sketch', handler: create_new_sketch },
		{ alt: 'extrude', src: '/actions/extrude_min.svg', handler: create_new_extrusion },
		{ alt: 'plane', src: '/actions/plane_min.svg' },
		{ alt: 'step', src: '/actions/step_min.svg', text: "Step", handler: step_sketch },
		// { alt: 'hole', src: '/actions/hole_min.svg' },
		// { alt: 'fillet', src: '/actions/fillet_min.svg' },
		// { alt: 'revolve', src: '/actions/revolve_min.svg' }
	]

	let icon_mapping = {
		Sketch: '/actions/sketch_min.svg',
		Plane: '/actions/plane_min.svg',
		Point: '/actions/point_min_icon.svg'
	}
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
			<button class="inline-flex items-center hover:bg-gray-200 p-1" on:click={action.handler}>
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
		<MainCanvas {realization} />
	</main>
	<footer class="col-span-2">Footer</footer>
</div>

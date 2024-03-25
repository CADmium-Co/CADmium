<script>
	export let item
	import { step_being_edited, project_rust, active_workbench_index, new_realization_needed } from "./stores"
	console.log(item)
	const update_extrusion_length = (e) => {
		console.log("updating length to: ", e.target.value)

		let message = {
			UpdateExtrusionLength: {
				workbench_id: $active_workbench_index,
				extrusion_name: item.name,
				length: parseFloat(e.target.value)
			}
		}
		console.log("message:", message)

		let result = $project_rust.send_message(JSON.stringify(message))
		console.log("updating extrusion length result: ", result)
		new_realization_needed.set(true)
	}
</script>

<div class="px-3 py-2 bg-gray-300 flex flex-col space-y-2">
	<div class="text-sm">Faces</div>
	<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
	<div tabindex="0" class="bg-gray-50 rounded flex shadow border focus:ring focus:border-blue-500">
		{#each item.data.extrusion.face_ids as face_id}
			<div class="bg-sky-200 pl-2 py-0.5 m-1 rounded text-sm">
				{item.data.extrusion.sketch_name}:{face_id}
				<button class="rounded px-1 hover:bg-sky-400"><i class="fa-solid fa-x fa-xs" /></button>
			</div>
		{/each}
	</div>
	<div class="text-sm">Depth (m)</div>
	<input
		class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:border focus:border-blue-500"
		value={item.data.extrusion.length}
		on:change={update_extrusion_length}
	/>
	<button
		class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-1 px-1 rounded shadow"
		on:click={() => {
			step_being_edited.set(-1)
			new_realization_needed.set(true)
		}}>Done</button
	>
</div>

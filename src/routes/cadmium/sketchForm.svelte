<script>
	export let item
	import { new_realization_needed, step_being_edited, sketch_being_edited } from './stores'
	sketch_being_edited.set(item.name)
	// console.log('Sketch form item:', item.data.plane_name)
</script>

<div class="px-3 py-2 bg-gray-300 flex flex-col space-y-2">
	<div class="text-sm">Plane</div>
	<div class="bg-gray-50 rounded flex shadow border">
		<div class="bg-sky-200 pl-2 py-0.5 m-1 rounded text-sm">
			{item.data.plane_name}
			<button class="rounded px-1 hover:bg-sky-400"><i class="fa-solid fa-x fa-xs" /></button>
		</div>
	</div>
	<button
		class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-1 px-1 rounded shadow"
		on:click={() => {
			step_being_edited.set(-1)
			new_realization_needed.set(true)
			sketch_being_edited.set(null)
		}}>Done</button
	>
</div>

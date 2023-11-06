<script>
	export let item
	import { new_realization_needed, step_being_edited, sketch_being_edited } from './stores'
	sketch_being_edited.set(item.name)
</script>

<div class="px-3 py-2 bg-gray-300 flex flex-col space-y-2">
	Sketch
	<button
		class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-1 px-1 rounded shadow"
		on:click={() => {
			step_being_edited.set(-1)
			new_realization_needed.set(true)
			sketch_being_edited.set(null)
		}}>Done</button
	>
</div>

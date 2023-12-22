<script>
	import { slide } from 'svelte/transition'
	import { quintOut } from 'svelte/easing'
	import { renameStep } from './projectUtils'
	import { workbenchIsStale } from './stores.js'

	export let name
	let editing = false

	let source = '/actions/point_min_icon.svg'

	const closeAndRefresh = () => {
		console.log('closing, refreshing')
		workbenchIsStale.set(true)
	}
</script>

<div
	class="flex items-center text-sm hover:bg-sky-200"
	role="button"
	tabindex="0"
	on:dblclick={() => {
		editing = !editing

		if (editing === false) {
			closeAndRefresh()
		}
	}}
>
	<img class="h-8 w-8 px-1" src={source} alt={name} />
	{name}
</div>

{#if editing}
	<div transition:slide={{ delay: 0, duration: 400, easing: quintOut, axis: 'y' }}>
		<form
			on:submit|preventDefault={() => {
				editing = false
				closeAndRefresh()
			}}
			class="px-3 py-2 bg-gray-100 flex flex-col space-y-2"
			autocomplete="off"
		>
			<label>
				Name
				<input
					autocomplete="off"
					data-1p-ignore
					class="shadow appearance-none border w-full py-2 px-3 text-gray-700 leading-tight focus:border focus:border-blue-500"
					bind:value={name}
				/>
			</label>

			<div class="flex space-x-1.5">
				<button
					class="flex-grow bg-blue-500 hover:bg-blue-700 text-white font-bold py-1.5 px-1 shadow"
					on:click={() => {
						renameStep(name)
					}}>Done</button
				>

				<button
					class="bg-transparent hover:bg-blue-500 text-blue-700 font-semibold hover:text-white py-1.5 px-4 border border-blue-500 hover:border-transparent"
					>Cancel</button
				>
			</div>
		</form>
	</div>
{/if}

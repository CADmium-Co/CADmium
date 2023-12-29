<script>
	import { slide } from 'svelte/transition'
	import { quintOut } from 'svelte/easing'
	import { renameStep } from './projectUtils'
	import { selectingFor, workbenchIsStale, featureIndex, currentlySelected } from './stores.js'
	import X from 'phosphor-svelte/lib/X'

	export let name, index
	export let data
	// looks like: {sketch_id: 'Sketch-0', face_ids: Array(0), length: 0.25, offset: 0, direction: 'Normal'}

	let faceIds = data.face_ids
	// faceIds = ['face1', 'face2', 'face3', 'face4', 'face5', 'face6', 'face7']
	let length = data.length
	let direction = data.direction

	const closeAndRefresh = () => {
		console.log('extrusion feature closing')
		$featureIndex = 1000
		$currentlySelected = []
		workbenchIsStale.set(true)
	}

	currentlySelected.subscribe((e) => {
		if ($featureIndex !== index) return

		console.log('currently selected has changed to:', $currentlySelected)
	})

	// $: console.log($currentlySelected)
	// $: faceIds = $currentlySelected.filter((e) => e.type === 'face').map((e) => e.id)

	let source = '/actions/extrude_min.svg'
</script>

<div
	class="flex items-center text-sm hover:bg-sky-200"
	role="button"
	tabindex="0"
	on:dblclick={() => {
		if ($featureIndex === index) {
			closeAndRefresh()
		} else {
			$featureIndex = index
			// $selectingFor = []
		}
	}}
>
	{#if $featureIndex < index}
		<img class="h-8 w-8 px-1 opacity-50" src={source} alt={name} />
		<span class="italic opacity-50">{name}</span>
	{:else}
		<img class="h-8 w-8 px-1" src={source} alt={name} />
		<span>{name}</span>
	{/if}
</div>

{#if $featureIndex === index}
	<div transition:slide={{ delay: 0, duration: 400, easing: quintOut, axis: 'y' }}>
		<form
			on:submit|preventDefault={() => {
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
					class="shadow appearance-none border w-full py-2 px-3 text-gray-700 leading-tight focus:border focus:border-sky-500"
					bind:value={name}
				/>
			</label>

			<label>
				Length
				<input
					autocomplete="off"
					data-1p-ignore
					class="shadow appearance-none border w-full py-2 px-3 text-gray-700 leading-tight focus:border focus:border-sky-500"
					bind:value={length}
				/>
			</label>

			Faces
			<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
			<div
				tabindex="0"
				class="bg-gray-50 rounded flex shadow border focus:ring focus:border-blue-500 min-h-8 flex-wrap"
				on:focusin={() => {
					$selectingFor = ['face']
					$currentlySelected = faceIds.map((id) => ({ type: 'face', id }))
				}}
				on:focusout={() => {
					$selectingFor = []
				}}
			>
				<div class="h-8" />
				{#each faceIds as faceId}
					<div class="bg-sky-200 pl-2 py-0.5 m-1 rounded text-sm">
						{faceId}<button
							on:click|preventDefault={() => {
								faceIds = faceIds.filter((id) => id !== faceId)
							}}><X /></button
						>
					</div>
				{/each}
			</div>

			<div class="flex space-x-1.5">
				<button
					class="flex-grow bg-sky-500 hover:bg-sky-700 text-white font-bold py-1.5 px-1 shadow"
					on:click={() => {
						renameStep(index, name)
					}}>Done</button
				>

				<button
					class="bg-transparent hover:bg-sky-700 text-sky-500 font-semibold hover:text-white py-1.5 px-4 border border-sky-500 hover:border-transparent"
					>Cancel</button
				>
			</div>
		</form>
	</div>
{/if}

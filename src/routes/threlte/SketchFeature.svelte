<script>
	import { slide } from 'svelte/transition'
	import { quintOut } from 'svelte/easing'
	import { renameStep, setSketchPlane } from './projectUtils'
	import {
		hiddenSketches,
		featureIndex,
		selectionMax,
		selectionMin,
		currentlySelected,
		selectingFor,
		sketchBeingEdited,
		sketchTool
	} from './stores.js'
	import EyeSlash from 'phosphor-svelte/lib/EyeSlash'
	import Eye from 'phosphor-svelte/lib/Eye'
	import X from 'phosphor-svelte/lib/X'

	export let name, index, id, plane_id

	let source = '/actions/sketch_min.svg'

	let surface = null
	let selectingForSketchPlane = false

	$: {
		if (plane_id !== '') {
			surface = { type: 'plane', id: plane_id }
		} else {
			surface = null
		}
	}
	console.log('A Sketch Feature: ', name, index, id, plane_id)

	const closeAndRefresh = () => {
		console.log('closing, refreshing')
		$featureIndex = 1000
		$sketchBeingEdited = null
		$sketchTool = null
		$selectingFor = []
		$selectionMax = 1000
		$selectionMin = 0
		$currentlySelected = []
	}

	$: if ($featureIndex === index) {
		$sketchBeingEdited = id
	}

	currentlySelected.subscribe(() => {
		if (!selectingForSketchPlane) return

		console.log('CS changed when selecting for Sketch Plane:', $currentlySelected)

		setSketchPlane(id, $currentlySelected[0].id)
	})
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
			$sketchTool = 'select'
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

	<!-- svelte-ignore a11y-click-events-have-key-events -->
	<!-- svelte-ignore a11y-no-static-element-interactions -->
	<div
		class="ml-auto mr-2 bg-slate-100 px-1 py-1 rounded"
		on:click={() => {
			if ($hiddenSketches.includes(id)) {
				// cool, unhide
				hiddenSketches.update((sketches) => {
					return sketches.filter((sketch) => sketch !== id)
				})
			} else {
				// cool, hide
				hiddenSketches.update((sketches) => {
					return [...sketches, id]
				})
			}
		}}
	>
		{#if $hiddenSketches.includes(id)}
			<EyeSlash weight="light" size="18px" />
		{:else}
			<Eye weight="light" size="18px" />
		{/if}
	</div>
</div>

{#if $featureIndex === index}
	<div transition:slide={{ delay: 0, duration: 400, easing: quintOut, axis: 'y' }}>
		<form
			on:submit|preventDefault={() => {
				// editing = false
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

			<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
			Surface
			<div
				tabindex="0"
				class="bg-gray-50 rounded flex shadow border focus:ring focus:border-blue-500 min-h-8 flex-wrap"
				on:focusin={() => {
					$sketchTool = null
					$selectingFor = ['plane', 'meshFace']
					$selectionMax = 1
					$selectionMin = 1

					if (surface !== null) {
						$currentlySelected = [surface]
					}
					selectingForSketchPlane = true
				}}
				on:focusout={() => {
					$sketchTool = null
					$selectingFor = []
					$selectionMax = 1000
					$selectionMin = 0
					selectingForSketchPlane = false
				}}
			>
				<div class="h-8" />
				{#if surface !== null}
					<div class="bg-sky-200 pl-2 py-0.5 m-1 rounded text-sm">
						{surface.type}:{surface.id}<button
							on:click|preventDefault={() => {
								surface = null
							}}><X /></button
						>
					</div>
				{/if}
			</div>

			<div class="flex space-x-1.5">
				<button
					class="flex-grow bg-sky-500 hover:bg-sky-700 text-white font-bold py-1.5 px-1 shadow"
					on:click={() => {
						// This is a form button so remember that it triggers the form's on:submit
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

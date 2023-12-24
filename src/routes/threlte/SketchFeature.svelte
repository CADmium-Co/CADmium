<script>
	import { slide } from 'svelte/transition'
	import { quintOut } from 'svelte/easing'
	import { renameStep } from './projectUtils'
	import { hiddenSketches, sketchMode, featureIndex } from './stores.js'
	import EyeSlash from 'phosphor-svelte/lib/EyeSlash'
	import Eye from 'phosphor-svelte/lib/Eye'

	export let name
	export let index
	export let id
	// let hidden = true

	let sketch_modes = [{ name: 'Select' }, { name: 'Draw' }, { name: 'Constrain' }]

	let source = '/actions/sketch_min.svg'

	const closeAndRefresh = () => {
		console.log('closing, refreshing')
		$featureIndex = 1000
		$sketchMode = 'Select'
	}
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
			$sketchMode = 'Select'
		}
	}}
>
	<img class="h-8 w-8 px-1" src={source} alt={name} />
	{name}
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

			<label>
				Mode
				<div class="flex">
					{#each sketch_modes as sketch_mode}
						<input
							hidden
							type="radio"
							id={sketch_mode.name}
							name="mode"
							value={sketch_mode.name}
							on:click={() => ($sketchMode = sketch_mode.name)}
						/>
						{#if sketch_mode.name === $sketchMode}
							<label
								class="text-sm py-1 bg-white flex-grow border-solid border-2 border-sky-500 text-center"
								for={sketch_mode.name}>{sketch_mode.name}</label
							>
						{:else}
							<label
								class="text-sm py-1 bg-gray-200 flex-grow border-solid border-2 border-gray-500 text-center hover:bg-gray-300"
								for={sketch_mode.name}>{sketch_mode.name}</label
							>
						{/if}

						<br />
					{/each}
				</div>
			</label>

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

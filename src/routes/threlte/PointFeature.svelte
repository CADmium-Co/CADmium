<script lang="ts">
	import { slide } from "svelte/transition"
	import { quintOut } from "svelte/easing"
	import { renameStep } from "./projectUtils"
	import { workbenchIsStale, featureIndex } from "./stores"

	const log = (function () {
		const context = "[PointFeature.svelte]"
		return Function.prototype.bind.call(
			console.log,
			console,
			`%c${context}`,
			"font-weight:bold;color:gray;"
		)
	})()

	export let name: string, index: number

	let source = "/actions/point_min_icon.svg"

	const closeAndRefresh = () => {
		log("closing, refreshing")
		workbenchIsStale.set(true)
		$featureIndex = 1000
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
		}
	}}
>
	<img class="h-8 w-8 px-1" src={source} alt={name} />
	{name}
</div>

{#if $featureIndex === index}
	<div transition:slide={{ delay: 0, duration: 400, easing: quintOut, axis: "y" }}>
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

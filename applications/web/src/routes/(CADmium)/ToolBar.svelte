<script lang="ts">
	import {
		currentlySelected,
		currentlyMousedOver,
		selectingFor,
		featureIndex,
		sketchBeingEdited,
		sketchTool,
		workbench,
		hiddenSketches
	} from "shared/stores"
	import * as AllTools from "./tools";
	import * as AllFeatures from "./features";
	import { newExtrusion, newSketchOnPlane } from "shared/projectUtils"
	import { base } from "$app/paths"
	import type { ToolType } from "shared/types"
	import { createFeatureList } from "./features"

	// prettier-ignore
	const log = (function () { const context = "[ToolBar.svelte]"; const color="gray"; return Function.prototype.bind.call(console.log, console, `%c${context}`, `font-weight:bold;color:${color};`)})()

	let solving = false
	const debugging = false

	interface ActionType {
		alt: string
		src: string
		text: string
		// tooltip: string // TODO
		handler: () => void
	}

	const actions: ActionType[] = Object.values(createFeatureList).map((feature): ActionType => ({
		alt: feature.name,
		src: `${base}/actions/${feature.name.toLowerCase()}_min.svg`,
		text: feature.name,
		handler: () => {
			feature.new()
			$featureIndex = $workbench.history.length - 1
		}
	}))

	const sketchActions: ActionType[] = Object.keys(AllTools).filter(toolName => toolName !== "Select").map((toolName: string): ActionType => ({
		alt: toolName,
		src: `${base}/actions/${toolName.toLowerCase()}.svg`,
		text: toolName,
		handler: () => ($sketchTool = (toolName as ToolType))
	}))
</script>

<div class="col-span-2 flex flex-none items-center gap-1 bg-gray-100 h-[45px] select-none">
	{#if $sketchBeingEdited !== ""}
		{#each sketchActions as action}
			<button
				class="inline-flex items-center p-1 {$sketchTool === action.alt ? 'bg-gray-400' : 'hover:bg-gray-200'} p-1"
				on:click={action.handler}
			>
				<img class="h-8 w-8" src={action.src} alt={action.alt} />{action.text ? action.text : ""}
			</button>
		{/each}
	{:else}
		{#each actions as action}
			<button
				class="inline-flex items-center {action.text === 'Solve' && solving ? 'bg-gray-400' : ''} hover:bg-gray-200 p-1"
				on:click={action.handler}
			>
				<img class="h-8 w-8" src={action.src} alt={action.alt} />{action.text ? action.text : ""}
			</button>
		{/each}
	{/if}

	{#if debugging}
		Selecting For [
		{#each $selectingFor as sf}
			<div>
				{sf},
			</div>
		{/each}
		] Currently Selected [
		{#each $currentlySelected as cs}
			<div>
				{cs.type}
				{cs.id},
			</div>
		{/each}
		] Moused Over [
		{#each $currentlyMousedOver as cm}
			<div>
				{cm.type}
				{cm.id},
			</div>
		{/each}
		] Hidden Sketches [
		{#each $hiddenSketches as hs}
			<div>
				{hs},
			</div>
		{/each}
		]
	{/if}
</div>

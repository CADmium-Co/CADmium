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
	import { newExtrusion, newSketchOnPlane } from "shared/projectUtils"
	import { base } from "$app/paths"
	import type { ToolType } from "shared/types"

	// prettier-ignore
	const log = (function () { const context = "[ToolBar.svelte]"; const color="gray"; return Function.prototype.bind.call(console.log, console, `%c${context}`, `font-weight:bold;color:${color};`)})()

	let solving = false
	const createNewExtrusion = () => {
		newExtrusion()
		// set that as the current feature being edited
		$featureIndex = $workbench.history.length - 1
	}
	const createNewSketch = () => {
		// log('Create new sketch')
		newSketchOnPlane()
		$featureIndex = $workbench.history.length - 1
	}
	const debugging = false

	const actions = [
		{
			alt: "new sketch",
			src: `${base}/actions/sketch_min.svg`,
			text: "New Sketch",
			handler: createNewSketch
		},
		{ alt: "extrude", src: `${base}/actions/extrude_min.svg`, handler: createNewExtrusion }
		// { alt: 'plane', src: '/actions/plane_min.svg' }
	]

	interface SketchActionType {
		alt: string
		src: string
		text: string
		// tooltip: string // TODO
		handler: () => void
	}

	const sketchActions: SketchActionType[] = Object.keys(AllTools).filter(toolName => toolName !== "Select").map((toolName: string): SketchActionType => ({
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

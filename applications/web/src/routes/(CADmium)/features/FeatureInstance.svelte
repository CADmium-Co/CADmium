<script lang="ts">
	import type { SvelteComponent } from "svelte"
	import * as AllFeatures from "./";
	import type { HistoryStep } from "shared/types"

	// prettier-ignore
	const log = (function () { const context = "[FeatureInstance.svelte]"; const color="yellow"; return Function.prototype.bind.call(console.log, console, `%c${context}`, `font-weight:bold;color:${color};`)})()

	export let feature: HistoryStep, featureIdx: number;

	const FeatureComponent: ConstructorOfATypedSvelteComponent = AllFeatures[feature.data.type];
</script>

{#if FeatureComponent}
		<!-- We're expanding the whole `feature` which will result in `<feature> was created with unknown prop '<x>'` -->
		<svelte:component this={FeatureComponent} {...feature} {featureIdx} />
{:else}
		<div>TODO: {feature.name} ({feature.data.type})</div>
{/if}

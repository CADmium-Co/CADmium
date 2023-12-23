<script>
	import ActiveSketch from './ActiveSketch.svelte'
	import PassiveSketch from './PassiveSketch.svelte'

	export let uniqueId
	export let name
	export let sketchTuple
	export let editing
	export let plane

	// $: console.log('Sketch name: ', name)
	// $: console.log('Sketch tuple: ', sketchTuple)
	// TODO: draw the extent of the plane with some lines and put text on it
</script>

{#if editing}
	<ActiveSketch {name} {uniqueId} sketch={sketchTuple[0]} plane={plane.plane} />
{:else}
	<PassiveSketch {name} {uniqueId} sketch={sketchTuple[1]} plane={plane.plane} />
{/if}

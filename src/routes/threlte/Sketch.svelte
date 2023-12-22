<script>
	import ActiveSketch from './ActiveSketch.svelte'
	import PassiveSketch from './PassiveSketch.svelte'

	export let name
	export let sketchTuple
	export let editing
	export let plane

	// TODO: draw the extent of the plane with some lines and put text on it
</script>

{#if editing}
	<ActiveSketch {name} sketch={sketchTuple[0]} plane={plane.plane} />
{:else}
	<PassiveSketch {name} sketch={sketchTuple[1]} plane={plane.plane} />
{/if}

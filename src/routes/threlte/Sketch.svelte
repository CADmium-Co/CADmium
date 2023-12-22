<script>
	import ActiveSketch from './ActiveSketch.svelte'
	import PassiveSketch from './PassiveSketch.svelte'

	export let name
	export let sketchTuple
	export let editing

	console.log('A sketch:', sketchTuple)
</script>

{#if editing}
	<ActiveSketch {name} sketch={sketchTuple[0]} />
{:else}
	<PassiveSketch {name} sketch={sketchTuple[1]} />
{/if}

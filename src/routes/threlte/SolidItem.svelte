<script>
	export let name

	let source = '/actions/part.svg'
</script>

<div class="flex items-center text-sm hover:bg-sky-200" role="button" tabindex="0">
	<img class="h-8 w-8 px-1" src={source} alt={name} />
	{name}
</div>

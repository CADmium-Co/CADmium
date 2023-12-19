<script>
	export let userName = 'mattferraro.dev'
	export let project = {}
</script>

<div class="bg-gray-200 h-[45px]">
	<div class="flex items-center gap-4 bg-gray-">
		<div class="shrink-0 select-none">
			<img class="object-cover h-10 w-10 ml-4" alt="logo" src="/cadmium_logo_min.svg" />
		</div>
		<div class="select-none">CADmium</div>
		<div class="text-xl font-medium">{project.name || ''}</div>

		<div class="flex-grow flex flex-row-reverse gap-4 mr-4">
			<div>
				<a href="https://github.com/mattferraro/cadmium"
					><img class="h-6 w-6" src="/github-mark.svg" alt="github logo" /></a
				>
			</div>
			<div>{userName}</div>
		</div>
	</div>
</div>

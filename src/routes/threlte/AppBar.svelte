<script>
	import fileDownload from 'js-file-download'
	import { projectIsStale, wasmProject } from './stores'

	import Download from 'phosphor-svelte/lib/Download'
	import Upload from 'phosphor-svelte/lib/Upload'

	export let userName = 'mattferraro.dev'
	export let project = {}

	export let newFileContent = null
</script>

<div class="bg-gray-200 h-[45px]">
	<!-- svelte-ignore a11y-click-events-have-key-events -->
	<div class="flex items-center gap-4 bg-gray-">
		<div class="shrink-0 select-none">
			<img class="object-cover h-10 w-10 ml-4" alt="logo" src="/cadmium_logo_min.svg" />
		</div>
		<div class="select-none">CADmium</div>
		<div class="text-xl font-medium">{project.name || ''}</div>
		<!-- svelte-ignore a11y-no-static-element-interactions -->
		<div
			class="hover:bg-gray-300 rounded p-1"
			on:click={() => {
				console.log('down')
				let asString = $wasmProject.to_json()
				fileDownload(asString, `${project.name}.cadmium`)
			}}
		>
			<Download class="h-6 w-6" />
		</div>

		<!-- svelte-ignore a11y-no-static-element-interactions -->
		<div
			class="hover:bg-gray-300 rounded p-1"
			on:click={() => {
				console.log('up')
			}}
		>
			<!-- <Upload class="h-6 w-6" /> -->
			<!-- <input id="file-inp" type="file" style="visibility:hidden;" onchange="readFile(event)" /> -->
			<input
				id="file-inp"
				type="file"
				on:change={(e) => {
					console.log('on change: ', e)
					var file = e.target.files[0]
					if (!file) return
					var reader = new FileReader()
					reader.onload = function (e) {
						// console.log('file contents', e.target.result)
						newFileContent = e.target.result

						// console.log('wasm project', $wasmProject)
						// let newWasmProject = $wasmProject.from_json(e.target.result)
						// wasmProject.set(newWasmProject)
						// projectIsStale.set(true)
					}
					reader.readAsText(file)
				}}
			/>
		</div>

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

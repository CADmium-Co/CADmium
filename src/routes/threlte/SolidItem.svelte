<script lang="ts">
	import fileDownload from 'js-file-download'
	import FileArrowDown from 'phosphor-svelte/lib/FileArrowDown'
	import { getObjectString } from "./projectUtils"
	import type { WithTarget } from "../../types"

	// prettier-ignore
	const log = (function () { const context = "[SolidItem.svelte]"; const color="gray"; return Function.prototype.bind.call(console.log, console, `%c${context}`, `font-weight:bold;color:${color};`)})()

	export let name: string

	const source = "/actions/part.svg"
	let contextMenuVisible = false

	// pos is cursor position when right click occur
	let pos = { x: 0, y: 0 }
	// menu is dimension (height and width) of context menu
	let menu = { h: 0, w: 0 }
	// browser/window dimension (height and width)
	let browser_size = { h: 0, w: 0 }

	function getContextMenuDimension(node: HTMLElement) {
		// This function will get context menu dimension
		// when navigation is shown => showMenu = true
		const height = node.offsetHeight
		const width = node.offsetWidth
		menu = {
			h: height,
			w: width
		}
	}

	export function rightClickContextMenu(e: WithTarget<MouseEvent, HTMLElement>) {
		contextMenuVisible = true
		browser_size = {
			w: window.innerWidth,
			h: window.innerHeight
		}
		pos = {
			x: e.clientX + 10,
			y: e.clientY
		}
		// If bottom part of context menu will be displayed
		// after right-click, then change the position of the
		// context menu. This position is controlled by `top` and `left`
		// at inline style.
		// Instead of context menu is displayed from top left of cursor position
		// when right-click occur, it will be displayed from bottom left.
		if (browser_size.h - pos.y < menu.h) pos.y = pos.y - menu.h
		if (browser_size.w - pos.x < menu.w) pos.x = pos.x - menu.w
	}

	function exportSolidOBJ() {
		contextMenuVisible = false
		const asString = getObjectString(name)
		fileDownload(asString, `${name}.obj`)
		contextMenuVisible = false
	}
	const exportSolidSTEP = () => {
		log("export solid STEP")
		contextMenuVisible = false
		// let step_string = $realization_rust.solid_to_step(solid_name)
		// log(step_string)
		// fileDownload(step_string, solid_name + '.step')
	}

	function onWindowClick() {
		contextMenuVisible = false
	}
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<div
	class="flex items-center text-sm hover:bg-sky-200"
	role="button"
	tabindex="0"
	on:contextmenu|preventDefault={(e) => {
		log("solid", e)
		rightClickContextMenu(e)
	}}
>
	<img class="h-8 w-8 px-1" src={source} alt={name} />
	{name}
</div>

{#if contextMenuVisible}
	<nav use:getContextMenuDimension style="position: absolute; top:{pos.y}px; left:{pos.x}px">
		<div
			class="navbar inline-flex border w-[210px] bg-white overflow-hidden flex-col rounded-[10px] border-[solid]"
			id="navbar"
		>
			<ul class="m-1.5">
				<li class="block list-none w-[1fr] my-1">
					<button
						class="text-base text-[#222] w-full h-[30px] text-left bg-white border-0 hover:text-black hover:text-left hover:bg-[#eee] hover:rounded-[5px] flex"
						on:click={exportSolidOBJ}
					>
						<FileArrowDown class="h-6 w-6 mx-2" /> Download as OBJ
					</button>
				</li>

				<li class="block list-none w-[1fr] my-1">
					<button
						class="text-base text-[#222] w-full h-[30px] text-left bg-white border-0 hover:text-black hover:text-left hover:bg-[#eee] hover:rounded-[5px] flex"
						on:click={exportSolidSTEP}
					>
						<FileArrowDown class="h-6 w-6 mx-2" />Download as STEP
					</button>
				</li>
			</ul>
		</div>
	</nav>
{/if}

<svelte:window on:click={onWindowClick} />

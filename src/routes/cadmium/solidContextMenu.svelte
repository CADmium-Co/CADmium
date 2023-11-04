<script>
	import fileDownload from 'js-file-download'
	import { realization, realization_rust } from './stores'

	let showMenu = false
	// pos is cursor position when right click occur
	let pos = { x: 0, y: 0 }
	// menu is dimension (height and width) of context menu
	let menu = { h: 0, y: 0 }
	// browser/window dimension (height and width)
	let browser_size = { h: 0, y: 0 }

	let solid_name

	export function hide() {
		showMenu = false
	}

	export function rightClickContextMenu(e, solid_id) {
		solid_name = solid_id
		showMenu = true
		browser_size = {
			w: window.innerWidth,
			h: window.innerHeight
		}
		pos = {
			x: e.clientX,
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

	function getContextMenuDimension(node) {
		// This function will get context menu dimension
		// when navigation is shown => showMenu = true
		let height = node.offsetHeight
		let width = node.offsetWidth
		menu = {
			h: height,
			w: width
		}
	}

	function exportSolidOBJ() {
		let obj_string = $realization_rust.solid_to_obj(solid_name, 0.001)
		fileDownload(obj_string, solid_name + '.obj')
	}
	const exportSolidSTEP = () => {
		let step_string = $realization_rust.solid_to_step(solid_name)
		console.log(step_string)
		fileDownload(step_string, solid_name + '.step')
	}

	function remove() {
		console.log('remove item')
	}

	let menuItems = [
		{
			name: 'export',
			onClick: exportSolidOBJ,
			displayText: 'Download as OBJ',
			class: 'fa-solid fa-download'
		},
		{
			name: 'export',
			onClick: exportSolidSTEP,
			displayText: 'Download as STEP',
			class: 'fa-solid fa-download'
		},
		{
			name: 'hr'
		},
		{
			name: 'trash',
			onClick: remove,
			displayText: 'Delete',
			class: 'fa-solid fa-trash-can'
		}
	]
</script>

{#if showMenu}
	<nav use:getContextMenuDimension style="position: absolute; top:{pos.y}px; left:{pos.x}px">
		<div
			class="navbar inline-flex border w-[210px] bg-white overflow-hidden flex-col rounded-[10px] border-[solid]"
			id="navbar"
		>
			<ul class="m-1.5">
				{#each menuItems as item}
					{#if item.name == 'hr'}
						<hr class="mx-0 my-[5px]" />
					{:else}
						<li class="block list-none w-[1fr]">
							<button
								class="text-base text-[#222] w-full h-[30px] text-left bg-white border-0 hover:text-black hover:text-left hover:bg-[#eee] hover:rounded-[5px]"
								on:click={item.onClick}
							>
								<i class="pl-2.5 pr-[15px] py-0 {item.class}" />{item.displayText}
							</button>
						</li>
					{/if}
				{/each}
			</ul>
		</div>
	</nav>
{/if}

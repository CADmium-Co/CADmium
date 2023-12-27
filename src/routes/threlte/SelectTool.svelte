<script>
	import { currentlyMousedOver, currentlySelected } from './stores'

	export let sketchIndex

	export function click(event, projected) {
		// just add the thing that is moused over to a new store like $currentlySelected

		// make a copy we can modify here, potentially in several ways
		let alreadySelected = [...$currentlySelected]
		for (let obj of $currentlyMousedOver) {
			let found = alreadySelected.some((e) => e.id === obj.id && e.type === obj.type) ? true : false

			if (found) {
				// unselect if it's already selected
				alreadySelected = alreadySelected.filter((e) => !(e.id === obj.id && e.type === obj.type))
			} else {
				alreadySelected.push(obj)
			}
		}

		console.log('already selected', alreadySelected)
		currentlySelected.set(alreadySelected)
	}

	// export a function to handle keyboard events
	// if the user presses the delete key, then we should delete the currently selected things
	// if the user presses the escape key, then we should deselect the currently selected things
</script>

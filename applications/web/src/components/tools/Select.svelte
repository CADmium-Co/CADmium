<script lang="ts">
  import {currentlyMousedOver, currentlySelected} from "shared/stores"
  import {deleteEntities} from "shared/projectUtils"

  const log = (function () { const context = "[SelectTool.svelte]"; const color="gray"; return Function.prototype.bind.call(console.log, console, `%c${context}`, `font-weight:bold;color:${color};`)})() // prettier-ignore

  export let sketchIndex: string, active: boolean

  export function click(_event: Event, _projected: any) {
    if ($currentlyMousedOver.length === 0) {
      // they clicked off into empty space. deselect everything
      currentlySelected.set([])
    }

    // just add the thing that is moused over to a new store like $currentlySelected
    // make a copy we can modify here, potentially in several ways
    let alreadySelected = [...$currentlySelected]
    for (let obj of $currentlyMousedOver) {
      let found = alreadySelected.some(e => e.id === obj.id && e.type === obj.type) ? true : false

      if (found) {
        // unselect if it's already selected
        alreadySelected = alreadySelected.filter(e => !(e.id === obj.id && e.type === obj.type))
      } else {
        alreadySelected.push(obj)
      }
    }

    log("already selected", alreadySelected)
    currentlySelected.set(alreadySelected)
  }

  // export a function to handle keyboard events
  // if the user presses the delete key, then we should delete the currently selected things
  // if the user presses the escape key, then we should deselect the currently selected things
  export function onKeyDown(event: KeyboardEvent) {
    if (!active) return

    // log('key press', event)
    if (event.key === "Escape") {
      currentlySelected.set([])
    } else if (event.key === "Delete" || event.key === "Backspace") {
      // delete the currently selected things
      deleteEntities(sketchIndex, $currentlySelected)
      currentlyMousedOver.set([])
      currentlySelected.set([])
    }
  }
</script>

<svelte:window on:keydown={onKeyDown} />

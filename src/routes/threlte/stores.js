import { writable } from 'svelte/store'

export const wasmProject = writable({})
export const project = writable({})
export const projectIsStale = writable(false)

export const workbenchIndex = writable(0)
export const wasmWorkbench = writable()
export const workbench = writable({})
export const workbenchIsStale = writable(false)

export const featureIndex = writable(1000)
export const wasmRealization = writable()
export const realization = writable({})
export const realizationIsStale = writable(false)

export const hiddenSketches = writable([])
export const sketchBeingEdited = writable(null)
export const sketchTool = writable(null)

// could be looking for 'face' or 'plane' or other things
export const selectingFor = writable([])
export const selectionMax = writable(1000)
export const selectionMin = writable(0)

export const currentlyMousedOver = writable([])
export const currentlySelected = writable([])
export const snapPoints = writable([])
export const previewGeometry = writable([])

export const messageHistory = writable([])

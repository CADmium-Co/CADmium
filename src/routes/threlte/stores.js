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
export const sketchMode = writable('Select')
export const sketchBeingEdited = writable(null)
export const sketchTool = writable(null)

export const tempPoints = writable([])
export const tempLines = writable([])
export const tempCircles = writable([])
export const tempArcs = writable([])

export const snapPoints = writable([])

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
// export const wasmRealization = writable({})

// stores the index of the feature being edited

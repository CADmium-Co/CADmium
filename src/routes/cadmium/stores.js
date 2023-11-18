import { writable } from 'svelte/store'

export const project_rust = writable({})
export const project = writable({})

export const realization_rust = writable({})
export const realization = writable({})

export const active_workbench_index = writable(-1)
export const workbench = writable({ history: [] })

export const outlined_solids = writable([])

export const step_being_edited = writable(-1)
export const new_realization_needed = writable(false)
export const sketch_being_edited = writable(null)

// could be ["plane"] or ["point", "line"], that kind of thing
export const looking_for = writable([])
export const selected = writable([])
export const found = writable([])

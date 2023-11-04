import { writable } from 'svelte/store'

export const project_rust = writable({})
export const project = writable({})

export const realization_rust = writable({})
export const realization = writable({})

export const active_workbench_index = writable(-1)
export const workbench = writable({ history: [] })

export const outlined_solids = writable([])

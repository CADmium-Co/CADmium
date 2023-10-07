import { writable } from 'svelte/store';


export const project_rust = writable({});
export const project = writable({});
export const active_workbench_index = writable(-1);
export const workbench = writable({ history: [] });
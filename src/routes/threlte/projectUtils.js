import {
	workbenchIsStale,
	workbenchIndex,
	workbench,
	project,
	wasmProject,
	projectIsStale,
	realizationIsStale,
	wasmRealization,
	realization
} from './stores'
import { get } from 'svelte/store'

export const CIRCLE_TOLERANCE = 0.0001

export function renameStep(stepIdx, newName) {
	console.log('renaming step to: ', newName)
	let wp = get(wasmProject)

	const message_obj = {
		RenameStep: {
			workbench_id: get(workbenchIndex),
			step_id: stepIdx,
			new_name: newName
		}
	}
	let result = wp.send_message(JSON.stringify(message_obj))
	console.log(result)
}

// If the project ever becomes stale, refresh it. This should be pretty rare.
projectIsStale.subscribe((value) => {
	if (value) {
		console.log('Refreshing project')
		let wp = get(wasmProject)
		project.set(JSON.parse(wp.to_json()))
		projectIsStale.set(false)

		workbenchIsStale.set(true)
	}
})

// If the workbench ever becomes stale, refresh it. This should be very common.
// Every time you edit any part of the feature history, for example
workbenchIsStale.subscribe((value) => {
	if (value) {
		let workbenchIdx = get(workbenchIndex)
		console.log('Refreshing workbench ', workbenchIdx)

		let wasmProj = get(wasmProject)
		let workbenchJson = wasmProj.get_workbench(workbenchIdx)
		// TODO: reach inside of project and set its representation
		// of the workbench to the new one that we just got
		workbench.set(JSON.parse(workbenchJson))
		workbenchIsStale.set(false)

		realizationIsStale.set(true)
	}
})

// If the realization ever becomes stale, refresh it. This should be very common.
// Every time you edit any part of the feature history, for example

realizationIsStale.subscribe((value) => {
	if (value) {
		console.log('Refreshing realization')
		const maxStep = 1000
		let wasmProj = get(wasmProject)
		let workbenchIdx = get(workbenchIndex)
		let wasmReal = wasmProj.get_realization(workbenchIdx, maxStep)
		wasmRealization.set(wasmReal)
		realization.set(JSON.parse(wasmReal.to_json()))
		// console.log('new realization:', get(realization))

		realizationIsStale.set(false)
	}
})

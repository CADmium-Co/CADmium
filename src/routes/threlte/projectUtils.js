import {
	workbenchIsStale,
	workbenchIndex,
	workbench,
	project,
	featureIndex,
	wasmProject,
	projectIsStale,
	realizationIsStale,
	wasmRealization,
	realization
} from './stores'
import { get } from 'svelte/store'
import { Vector2, Vector3 } from 'three'

export const CIRCLE_TOLERANCE = 0.0001

export function addRectangleBetweenPoints(sketchIdx, point1, point2) {
	console.log('trying to add a rectangle between', point1, point2)

	const messageObj = {
		NewRectangleBetweenPoints: {
			workbench_id: get(workbenchIndex),
			sketch_id: sketchIdx,
			start_id: parseInt(point1),
			end_id: parseInt(point2)
		}
	}

	let wp = get(wasmProject)
	let result = wp.send_message(JSON.stringify(messageObj))
	console.log(result)

	workbenchIsStale.set(true)
}

export function addCircleBetweenPoints(sketchIdx, point1, point2) {
	console.log('trying to add a circle between', point1, point2)

	const messageObj = {
		NewCircleBetweenPoints: {
			workbench_id: get(workbenchIndex),
			sketch_id: sketchIdx,
			center_id: parseInt(point1),
			edge_id: parseInt(point2)
		}
	}

	let wp = get(wasmProject)
	let result = wp.send_message(JSON.stringify(messageObj))
	console.log(result)

	workbenchIsStale.set(true)
}

export function addLineToSketch(sketchIdx, point1, point2) {
	console.log('trying to add a line between', point1, point2)

	const messageObj = {
		NewLineOnSketch: {
			workbench_id: get(workbenchIndex),
			sketch_id: sketchIdx,
			start_point_id: parseInt(point1),
			end_point_id: parseInt(point2)
		}
	}

	let wp = get(wasmProject)
	let result = wp.send_message(JSON.stringify(messageObj))
	console.log(result)

	workbenchIsStale.set(true)
}

export function addPointToSketch(sketchIdx, point, hidden) {
	console.log('trying to add point to sketch', get(workbenchIndex), sketchIdx, point, hidden)

	const messageObj = {
		NewPointOnSketch2: {
			workbench_id: get(workbenchIndex),
			sketch_id: sketchIdx,
			x: point.x,
			y: point.y,
			hidden: hidden
		}
	}
	console.log('sending message:', messageObj)

	let wp = get(wasmProject)
	let result = wp.send_message(JSON.stringify(messageObj))
	console.log('received result:', result)
	result = JSON.parse(result)

	// TODO: could this just refresh the workbench or realization?
	workbenchIsStale.set(true)

	return result.success.id
}

export function renameStep(stepIdx, newName) {
	console.log('renaming step to: ', newName)
	let wp = get(wasmProject)

	const messageObj = {
		RenameStep: {
			workbench_id: get(workbenchIndex),
			step_id: stepIdx,
			new_name: newName
		}
	}
	let result = wp.send_message(JSON.stringify(messageObj))
	console.log(result)
}

// If the project ever becomes stale, refresh it. This should be pretty rare.
projectIsStale.subscribe((value) => {
	if (value) {
		console.log('Refreshing project')
		let wp = get(wasmProject)
		project.set(JSON.parse(wp.to_json()))

		workbenchIndex.set(0)
		workbenchIsStale.set(true)

		projectIsStale.set(false)
	}
})

// If the workbench ever becomes stale, refresh it. This should be very common.
// Every time you edit any part of the feature history, for example
workbenchIsStale.subscribe((value) => {
	if (value) {
		let workbenchIdx = get(workbenchIndex)
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

		let wasmProj = get(wasmProject)
		let workbenchIdx = get(workbenchIndex)
		let wasmReal = wasmProj.get_realization(workbenchIdx, get(featureIndex) + 1)
		wasmRealization.set(wasmReal)
		realization.set(JSON.parse(wasmReal.to_json()))
		// console.log('new realization:', get(realization))

		realizationIsStale.set(false)
	}
})

export function readFile(e) {
	var file = e.target.files[0]
	if (!file) return
	var reader = new FileReader()
	reader.onload = function (e) {
		console.log('file contents', e.target.result)
	}
	reader.readAsText(file)
}

export function arcToPoints(center, start, end, clockwise) {
	// see https://math.stackexchange.com/a/4132095/816177
	const tolerance = CIRCLE_TOLERANCE // in meters
	const radius = start.distanceTo(center)
	const k = tolerance / radius
	// more precise but slower to calculate:
	// const n = Math.ceil(Math.PI / Math.acos(1 - k))
	// faster to calculate, at most only overestimates by 1:
	let n = Math.ceil(Math.PI / Math.sqrt(2 * k))
	const segmentAngle = (2 * Math.PI) / n
	const segmentLength = radius * segmentAngle
	if (clockwise) {
		n = -n
	}

	const startAngle = Math.atan2(start.y - center.y, start.x - center.x)

	const lineVertices = []
	lineVertices.push(start.clone())
	for (let i = 1; i <= Math.abs(n); i++) {
		let theta = ((2 * Math.PI) / n) * i + startAngle
		let xComponent = radius * Math.cos(theta)
		let yComponent = radius * Math.sin(theta)
		let point = new Vector2(xComponent, yComponent).add(center)
		lineVertices.push(point)

		let distanceToEnd = point.distanceTo(end)
		if (distanceToEnd <= segmentLength) {
			lineVertices.push(end.clone())
			break
		}
	}
	return lineVertices
}

export function circleToPoints(centerPoint, radius) {
	// this is 2D function
	// centerPoint is a Vector2, radius is a float
	// returns an array of Vector2's

	// see https://math.stackexchange.com/a/4132095/816177
	const tolerance = CIRCLE_TOLERANCE // in meters
	const k = tolerance / radius
	// more precise but slower to calculate:
	// const n = Math.ceil(Math.PI / Math.acos(1 - k))
	// faster to calculate, at most only overestimates by 1:
	const n = Math.ceil(Math.PI / Math.sqrt(2 * k))

	const lineVertices = []
	for (let i = 0; i <= n; i++) {
		let theta = ((2 * Math.PI) / n) * i
		let xComponent = radius * Math.cos(theta)
		let yComponent = radius * Math.sin(theta)
		let point = new Vector2(xComponent, yComponent).add(centerPoint)
		lineVertices.push(point)
	}
	return lineVertices
}

export function promoteTo3(points) {
	// points is an array of Vector2's
	// returns an array of Vector3's
	let points3 = []
	for (let point of points) {
		points3.push(new Vector3(point.x, point.y, 0))
	}
	return points3
}

export function flatten(points) {
	// points is an array of Vector3's
	// returns a flattened array of floats
	let pointsFlat = []

	for (let point of points) {
		pointsFlat.push(point.x, point.y, point.z)
	}
	return pointsFlat
}

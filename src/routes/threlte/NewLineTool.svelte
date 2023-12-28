<script>
	import { snapPoints, sketchTool, previewGeometry, currentlyMousedOver } from './stores'
	import { addLineToSketch, addPointToSketch } from './projectUtils'
	import { Vector3 } from 'three'

	export let pointsById
	export let sketchIndex
	export let active
	export let projectToPlane

	let previousPoint

	$: if ($sketchTool !== 'line') {
		previousPoint = null
	}

	function processPoint(point) {
		if (!previousPoint) {
			// if there is no anchor point, set one
			if (point.pointId) {
				// nothing to do, the point exists!
				console.log('nothing to do the point exists!')
			} else {
				console.log('oh cool, creating point!')
				point.pointId = null
			}
		} else {
			// there WAS an anchor point, so we should create a line
			if (previousPoint.pointId === null) {
				// if the center point doesn't exist, then we should create a point
				let result = addPointToSketch(sketchIndex, previousPoint.twoD, false)
				previousPoint.pointId = result
			}

			if (point.pointId) {
				// if the point exists, then we should create a line
				addLineToSketch(sketchIndex, previousPoint.pointId, point.pointId)
				previousPoint = null
				return
			} else {
				// if the point doesn't exist, then we should create a point and a line
				let result = addPointToSketch(sketchIndex, point.twoD, false)
				point.pointId = result
				addLineToSketch(sketchIndex, previousPoint.pointId, point.pointId)
			}
		}
		previousPoint = point
	}

	export function click(event, projected) {
		if ($snapPoints.length > 0) {
			processPoint($snapPoints[0])
		} else {
			let pt = { twoD: projected.twoD, threeD: projected.threeD, pointId: null }
			processPoint(pt)
		}
	}

	export function mouseMove(event, projected) {
		// TODO: in the future, we should also snap to the midpoints of lines
		// and to the perimeters of circles and so on
		// so these snap points do not necessarily correspond to actual points in the sketch
		let snappedTo

		for (let geom of $currentlyMousedOver) {
			if (geom.type === 'point3D') {
				let twoD = projectToPlane(new Vector3(geom.x, geom.y, geom.z))
				let point = {
					twoD: { x: twoD.x, y: twoD.y },
					threeD: { x: geom.x, y: geom.y, z: geom.z },
					pointId: null
				}
				snappedTo = point
			}
			if (geom.type === 'point') {
				let point = pointsById[geom.id]
				snappedTo = { twoD: point.twoD, threeD: point.threeD, pointId: geom.id }
				break // If there is a 2D point, prefer to use it rather than the 3D point
			}
		}

		// only reset $snapPoints if something has changed
		if (snappedTo) {
			$snapPoints = [snappedTo]
		} else {
			if ($snapPoints.length > 0) {
				$snapPoints = []
			}
		}

		if (previousPoint) {
			let end = { twoD: { x: projected.x, y: projected.y } }

			if (snappedTo) {
				end = snappedTo
			}

			let previewGeoms = [
				{ type: 'line', start: previousPoint, end: end, uuid: `line-${end.twoD.x}-${end.twoD.y}` },
				{ type: 'point', x: end.twoD.x, y: end.twoD.y, uuid: `point-${end.twoD.x}-${end.twoD.y}` }
			]

			if (previousPoint.pointId === null) {
				previewGeoms.push({
					type: 'point',
					x: previousPoint.twoD.x,
					y: previousPoint.twoD.y,
					uuid: `point-null-${previousPoint.twoD.x}-${previousPoint.twoD.y}`
				})
			}

			previewGeometry.set(previewGeoms)
		} else {
			previewGeometry.set([])
		}
	}

	export function onKeyDown(event) {
		if (!active) return

		if (event.key === 'Escape') {
			previewGeometry.set([])
			previousPoint = null
			$sketchTool = 'select'
		}
	}
</script>

<svelte:window on:keydown={onKeyDown} />

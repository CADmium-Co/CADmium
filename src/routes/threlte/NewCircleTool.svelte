<script>
	import { snapPoints, sketchTool, previewGeometry } from './stores'
	import { addCircleBetweenPoints, addPointToSketch } from './projectUtils'

	export let pointsById
	export let sketchIndex
	export let active

	let centerPoint

	$: if ($sketchTool === null) {
		centerPoint = null
	}

	function processPoint(point) {
		if (!centerPoint) {
			// if there is no center point, set one
			if (point.pointId) {
				// nothing to do, the point exists!
				// console.log('nothing to do the point exists!')
			} else {
				// again, don't actually DO anything yet to the sketch
				point.pointId = null
			}
			centerPoint = point
		} else {
			// there WAS an center point, so we should create a circle!
			if (centerPoint.pointId === null) {
				// if the center point doesn't exist, then we should create a point
				let result = addPointToSketch(sketchIndex, centerPoint.twoD, false)
				centerPoint.pointId = result
			}

			if (point.pointId) {
				// if the point exists, then we should create a circle between the two existing points
				addCircleBetweenPoints(sketchIndex, centerPoint.pointId, point.pointId)
			} else {
				// if the point doesn't exist, then we should create a point and a circle
				let result = addPointToSketch(sketchIndex, point.twoD, true)
				point.pointId = result
			}
			addCircleBetweenPoints(sketchIndex, centerPoint.pointId, point.pointId)
			centerPoint = null
		}
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
		// search through the existing points to see if we're close to one
		// if we are, then we should snap to it

		// TODO: in the future, we should also snap to the midpoints of lines
		// and to the perimeters of circles and so on
		// so these snap points do not necessarily correspond to actual points in the sketch
		let snappedTo
		for (let [pointId, point] of Object.entries(pointsById)) {
			let dx = point.twoD.x - projected.x
			let dy = point.twoD.y - projected.y
			// TODO: make the snap distance depend on camera zoom level so it appears consistent
			if (Math.hypot(dx, dy) < 0.01) {
				snappedTo = { twoD: point.twoD, threeD: point.threeD, pointId }
				break
			}
		}

		// only reset $snapPoints if something has changed
		if (snappedTo) {
			if ($snapPoints.length === 0) {
				$snapPoints = [snappedTo]
			}
		} else {
			if ($snapPoints.length > 0) {
				$snapPoints = []
			}
		}

		if (centerPoint) {
			let radius
			if (snappedTo) {
				let dx = snappedTo.twoD.x - centerPoint.twoD.x
				let dy = snappedTo.twoD.y - centerPoint.twoD.y
				radius = Math.hypot(dx, dy)
			} else {
				let dx = projected.x - centerPoint.twoD.x
				let dy = projected.y - centerPoint.twoD.y
				radius = Math.hypot(dx, dy)
			}

			previewGeometry.set([
				{
					type: 'circle',
					center: centerPoint,
					radius: radius,
					uuid: `circle-${centerPoint.twoD.x}-${centerPoint.twoD.y}-${radius}`
				},
				{
					type: 'point',
					x: centerPoint.twoD.x,
					y: centerPoint.twoD.y,
					uuid: `point-${centerPoint.twoD.x}-${centerPoint.twoD.y}`
				}
			])
		} else {
			previewGeometry.set([])
		}
	}

	export function onKeyDown(event) {
		if (!active) return

		if (event.key === 'Escape') {
			previewGeometry.set([])
			centerPoint = null
			$sketchTool = 'select'
		}
	}
</script>

<svelte:window on:keydown={onKeyDown} />

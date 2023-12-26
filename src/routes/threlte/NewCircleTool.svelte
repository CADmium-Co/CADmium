<script>
	import { snapPoints, sketchTool } from './stores'
	import { addCircleBetweenPoints, addPointToSketch } from './projectUtils'

	export let pointsById
	export let sketchIndex

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
				// console.log('oh cool, creating point!')
				let result = addPointToSketch(sketchIndex, point.twoD, false)
				point.pointId = result
			}
			centerPoint = point
		} else {
			// there WAS an center point, so we should create a circle!
			if (point.pointId) {
				// if the point exists, then we should create a circle between the two existing points
				addCircleBetweenPoints(sketchIndex, centerPoint.pointId, point.pointId)
				centerPoint = null
				return
			} else {
				// if the point doesn't exist, then we should create a point and a circle
				let result = addPointToSketch(sketchIndex, point.twoD, true)
				point.pointId = result
				addCircleBetweenPoints(sketchIndex, centerPoint.pointId, point.pointId)
			}
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
	}
</script>

<script>
	import { snapPoints, sketchTool, previewGeometry } from './stores'
	import { addRectangleBetweenPoints, addPointToSketch } from './projectUtils'

	export let pointsById
	export let sketchIndex
	export let active

	let anchorPoint

	$: if ($sketchTool === null) {
		anchorPoint = null
	}

	function processPoint(point) {
		if (!anchorPoint) {
			// if there is no anchor point, set one
			if (point.pointId) {
				// nothing to do, the point exists!
				// console.log('nothing to do the point exists!')
			} else {
				// console.log('oh cool, creating point!')
				point.pointId = null
			}
			console.log('set anchor point')
			anchorPoint = point
		} else {
			// there WAS an anchor point, so we should create a rectangle!
			if (anchorPoint.pointId === null) {
				// if the anchor point doesn't exist, then we should create a point
				let result = addPointToSketch(sketchIndex, anchorPoint.twoD, false)
				anchorPoint.pointId = result
			}

			if (point.pointId) {
				// if the point exists, then we should create a circle between the two existing points
				addRectangleBetweenPoints(sketchIndex, anchorPoint.pointId, point.pointId)
			} else {
				// if the point doesn't exist, then we should create a point and a circle
				let result = addPointToSketch(sketchIndex, point.twoD, false)
				point.pointId = result
			}
			console.log('setting rectangle')
			addRectangleBetweenPoints(sketchIndex, anchorPoint.pointId, point.pointId)
			anchorPoint = null
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

		if (anchorPoint) {
			let end = { twoD: { x: projected.x, y: projected.y } }

			if (snappedTo) {
				end = snappedTo
			}

			let upperLeft = { twoD: { x: anchorPoint.twoD.x, y: end.twoD.y } }
			let lowerRight = { twoD: { x: end.twoD.x, y: anchorPoint.twoD.y } }

			let previewGeoms = [
				{
					type: 'point',
					x: upperLeft.twoD.x,
					y: upperLeft.twoD.y,
					uuid: `point-ul-${upperLeft.twoD.x}-${upperLeft.twoD.y}`
				},
				{
					type: 'point',
					x: lowerRight.twoD.x,
					y: lowerRight.twoD.y,
					uuid: `point-lr-${lowerRight.twoD.x}-${lowerRight.twoD.y}`
				},
				{
					type: 'point',
					x: end.twoD.x,
					y: end.twoD.y,
					uuid: `point-end-${end.twoD.x}-${end.twoD.y}`
				},
				{
					type: 'line',
					start: anchorPoint,
					end: upperLeft,
					uuid: `line-s-ul-${upperLeft.twoD.x}-${upperLeft.twoD.y}`
				},

				{
					type: 'line',
					start: anchorPoint,
					end: lowerRight,
					uuid: `line-s-lr-${lowerRight.twoD.x}-${lowerRight.twoD.y}`
				},
				{
					type: 'line',
					start: upperLeft,
					end: end,
					uuid: `line-ul-end-${end.twoD.x}-${end.twoD.y}`
				},
				{
					type: 'line',
					start: lowerRight,
					end: end,
					uuid: `line-lr-end-${end.twoD.x}-${end.twoD.y}`
				}
			]

			if (anchorPoint.pointId === null) {
				previewGeoms.push({
					type: 'point',
					x: anchorPoint.twoD.x,
					y: anchorPoint.twoD.y,
					uuid: `point-null-${anchorPoint.twoD.x}-${anchorPoint.twoD.y}`
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
			anchorPoint = null
			$sketchTool = 'select'
		}
	}
</script>

<svelte:window on:keydown={onKeyDown} />

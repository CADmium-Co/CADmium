<script>
	import { snapPoints } from './stores'
	import { Vector2, Vector3 } from 'three'

	export let pointsById

	let initialPoint

	export function click(event, projected) {
		console.log('clicking', event)

		console.log('projected', projected)

		// let inTwoD = projectToPlane(e.point)

		// addPointToSketch(uniqueId, inTwoD)
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

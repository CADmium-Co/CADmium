<script lang="ts">
	import { snapPoints, sketchTool, previewGeometry, currentlyMousedOver } from './stores'
	import { addCircleBetweenPoints, addPointToSketch } from './projectUtils'
	import { Vector3, type Vector2Like, type Vector3Like } from "three"
	import type { PointLikeById, Point2D, PointsLikeById, ProjectToPlane } from "../../types"

	// prettier-ignore
	const log = (function () { const context = "[NewCircleTool.svelte]"; const color="gray"; return Function.prototype.bind.call(console.log, console, `%c${context}`, `font-weight:bold;color:${color};`)})()

	export let pointsById: PointsLikeById
	export let sketchIndex: string
	export let active: boolean
	export let projectToPlane: ProjectToPlane

	// log("[props]", "pointsById:", pointsById, "sketchIndex:", sketchIndex, "active:", active)

	let centerPoint: PointLikeById | null

	$: if ($sketchTool !== "circle") centerPoint = null
	// $: centerPoint, log("[centerPoint]", centerPoint)

	function processPoint(point: PointLikeById) {
		if (!centerPoint) {
			// if there is no center point, set one
			if (point.pointId) {
				// nothing to do, the point exists!
				// log('nothing to do the point exists!')
			} else {
				// again, don't actually DO anything yet to the sketch
				point.pointId = null
			}
			centerPoint = point
		} else {
			// there WAS an center point, so we should create a circle!

			// if the center point doesn't exist, then we should create a point
			if (centerPoint.pointId === null)
				centerPoint.pointId = addPointToSketch(sketchIndex, centerPoint.twoD!, false)

			if (point.pointId && centerPoint.pointId) {
				// if the point exists, then we should create a circle between the two existing points
				addCircleBetweenPoints(sketchIndex, centerPoint.pointId, point.pointId)
			} else {
				// if the point doesn't exist, then we should create a point and a circle
				point.pointId = addPointToSketch(sketchIndex, point.twoD!, true)
			}
			if (point.pointId && centerPoint.pointId)
				addCircleBetweenPoints(sketchIndex, centerPoint.pointId, point.pointId)
			centerPoint = null
		}
	}

	export function click(_event: Event, projected: { twoD: Vector2Like; threeD: Vector3Like }) {
		if ($snapPoints.length > 0) processPoint($snapPoints[0])
		else {
			let pt: PointLikeById = { twoD: projected.twoD, threeD: projected.threeD, pointId: null }
			processPoint(pt)
		}
	}

	// $: $snapPoints, log("[$snapPoints]", $snapPoints)

	export function mouseMove(_event: Event, projected: { x: number; y: number }) {
		// search through the existing points to see if we're close to one
		// if we are, then we should snap to it

		// TODO: in the future, we should also snap to the midpoints of lines
		// and to the perimeters of circles and so on
		// so these snap points do not necessarily correspond to actual points in the sketch
		let snappedTo = null
		for (const geom of $currentlyMousedOver) {
			// log("[currentlyMousedOver geom]", geom)
			if (geom.type === 'point3D') {
				const twoD = projectToPlane(new Vector3(geom.x, geom.y, geom.z))
				// log("[projectToPlane twoD]", twoD)
				const point = {
					twoD: { x: twoD.x, y: twoD.y },
					threeD: { x: geom.x, y: geom.y, z: geom.z },
					pointId: null
				}
				snappedTo = point
			}
			if (geom.type === 'point') {
				// log("[currentlyMousedOver geom is type point]", geom)
				const point = pointsById[geom.id]
				// oops! point.twoD etc does not exist here, we have:
				// const example = {
				// 	type: "point",
				// 	id: "1"
				// }
				function querySnapPoint(id: string | null) {
					const points = $snapPoints.filter((point) => id && point.pointId === id)
					return points.length > 0 ? points[0] : false
				}
				// see if we can retrieve it? unlikely
				// log("[querySnapPoint found point:]", querySnapPoint(point?.pointId!))
				// have not seen a successful query yet! sort it out with an if:
				if (point.twoD && point.threeD && geom.id)
					snappedTo = { twoD: point.twoD, threeD: point.threeD, pointId: geom.id }
				break // If there is a 2D point, prefer to use it rather than the 3D point
			}
		}

		// if (snappedTo) log("[snappedTo]", snappedTo)

		// only reset $snapPoints if something has changed
		if (snappedTo) {
			// @ts-ignore todo rework snapping
			$snapPoints = [snappedTo] // todo all these different point representations need work!
		} else {
			if ($snapPoints.length > 0) {
				$snapPoints = []
			}
		}

		if (centerPoint) {
			function calcDeltas(
				a: Vector2Like | Point2D | { x: number; y: number },
				b: Vector2Like | undefined
			) {
				const dx = a.x - b?.x!
				const dy = a.y - b?.y!
				return Math.hypot(dx, dy)
			}
			const radius = snappedTo
				? calcDeltas(snappedTo.twoD, centerPoint.twoD)
				: calcDeltas(projected, centerPoint.twoD)

			previewGeometry.set([
				{
					type: 'circle',
					center: centerPoint,
					radius,
					uuid: `circle-${centerPoint.twoD?.x}-${centerPoint.twoD?.y}-${radius}`
				},
				{
					type: 'point',
					x: centerPoint.twoD?.x,
					y: centerPoint.twoD?.y,
					uuid: `point-${centerPoint.twoD?.x}-${centerPoint.twoD?.y}`
				}
			])
		} else {
			previewGeometry.set([])
		}
	}

	export function onKeyDown(event: KeyboardEvent) {
		if (!active) return
		if (event.key === 'Escape') {
			previewGeometry.set([])
			centerPoint = null
			$sketchTool = 'select'
		}
	}
</script>

<svelte:window on:keydown={onKeyDown} />

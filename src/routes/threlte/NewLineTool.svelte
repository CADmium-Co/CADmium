<script lang="ts">
	import { snapPoints, sketchTool, previewGeometry, currentlyMousedOver } from "./stores"
	import { addLineToSketch, addPointToSketch } from "./projectUtils"
	import { Vector2, Vector3 } from "three"
	import type { Point, PointLikeById, PointsById, ProjectToPlane } from "../../types"

	const log = (function () {
		const context = "[NewLineTool.svelte]"
		return Function.prototype.bind.call(
			console.log,
			console,
			`%c${context}`,
			"font-weight:bold;color:limegreen;"
		)
	})()

	export let pointsById: PointsById
	export let sketchIndex: string
	export let active: boolean
	export let projectToPlane: ProjectToPlane

	$: pointsById, log("[props]", pointsById, sketchIndex, active, projectToPlane)

	let previousPoint: { pointId: string | null; twoD: Vector2 } | null

	$: if ($sketchTool !== "line") {
		previousPoint = null
	}

	function processPoint(point: PointLikeById | { pointId: string | null; twoD: Vector2 } | null) {
		if (!previousPoint && point) {
			// if there is no anchor point, set one
			if (point.pointId) {
				// nothing to do, the point exists!
				log("nothing to do the point exists!")
			} else {
				log("oh cool, creating point!")
				point.pointId = null
			}
		} else {
			// there WAS an anchor point, so we should create a line
			if (previousPoint?.pointId === null) {
				// if the center point doesn't exist, then we should create a point
				let result = addPointToSketch(sketchIndex, previousPoint.twoD, false)
				previousPoint.pointId = result
			}

			if (point?.pointId) {
				// if the point exists, then we should create a line
				if (previousPoint?.pointId && point.pointId)
					addLineToSketch(sketchIndex, previousPoint.pointId, point.pointId)
				previousPoint = null
				return
			} else {
				// if the point doesn't exist, then we should create a point and a line
				let result = addPointToSketch(sketchIndex, point!.twoD, false)
				point!.pointId = result
				addLineToSketch(sketchIndex, previousPoint!.pointId!, point!.pointId!)
			}
		}
		previousPoint = point // todo rework points
	}

	export function click(_event: Event, projected: Point) {
		if ($snapPoints.length > 0) {
			processPoint($snapPoints[0])
		} else {
			let pt = { twoD: projected.twoD, threeD: projected.threeD, pointId: null }
			processPoint(pt)
		}
	}

	export function mouseMove(_event: Event, projected: { x: number; y: number }) {
		// TODO: in the future, we should also snap to the midpoints of lines
		// and to the perimeters of circles and so on
		// so these snap points do not necessarily correspond to actual points in the sketch
		let snappedTo

		for (let geom of $currentlyMousedOver) {
			log("[geom of $currentlyMousedOver]", geom)
			if (geom.type === "point3D") {
				if (geom.x && geom.y && geom.z) {
					let twoD = projectToPlane(new Vector3(geom.x, geom.y, geom.z))
					// log("[projectToPlane twoD]", twoD)
					let point: PointLikeById = {
						twoD: { x: twoD.x, y: twoD.y },
						threeD: { x: geom.x, y: geom.y, z: geom.z },
						pointId: null
					}
					log("[point:PointById]", point)
					snappedTo = point
				}
			}
			if (geom.type === "point") {
				let point = pointsById[geom.id]
				log("[pointsById]", pointsById)
				snappedTo = { twoD: point.twoD, threeD: point.threeD, pointId: geom.id }
				log("[snappedTo]", snappedTo)
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
				{ type: "line", start: previousPoint, end: end, uuid: `line-${end.twoD.x}-${end.twoD.y}` },
				{ type: "point", x: end.twoD.x, y: end.twoD.y, uuid: `point-${end.twoD.x}-${end.twoD.y}` }
			]

			if (previousPoint.pointId === null) {
				previewGeoms.push({
					type: "point",
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

	export function onKeyDown(event: KeyboardEvent) {
		if (!active) return

		if (event.key === "Escape") {
			previewGeometry.set([])
			previousPoint = null
			$sketchTool = "select"
		}
	}
</script>

<svelte:window on:keydown={onKeyDown} />

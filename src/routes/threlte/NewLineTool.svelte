<script lang="ts">
	import { snapPoints, sketchTool, previewGeometry, currentlyMousedOver } from "./stores"
	import { addLineToSketch, addPointToSketch } from "./projectUtils"
	import { Vector2, Vector3 } from "three"
	import type {
		Point,
		PointLikeById,
		PointsById,
		PreviewGeometry,
		ProjectToPlane
	} from "../../types"

	const log = (function () {
		const context = "[NewLineTool.svelte]"
		return Function.prototype.bind.call(
			console.log,
			console,
			`%c${context}`,
			"font-weight:bold;color:limegreen;"
		)
	})()

	export let pointsById: PointsById,
		sketchIndex: string,
		active: boolean,
		projectToPlane: ProjectToPlane

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

			// if the center point doesn't exist, then we should create a point
			if (previousPoint?.pointId === null)
				previousPoint.pointId = addPointToSketch(sketchIndex, previousPoint.twoD, false)

			if (point?.pointId) {
				// if the point exists, then we should create a line
				if (previousPoint?.pointId && point.pointId)
					addLineToSketch(sketchIndex, previousPoint.pointId, point.pointId)
				previousPoint = null
				return
			} else {
				// if the point doesn't exist, then we should create a point and a line
				point!.pointId = addPointToSketch(sketchIndex, point!.twoD!, false)
				addLineToSketch(sketchIndex, previousPoint!.pointId!, point!.pointId!)
			}
		}
		// @ts-ignore  todo rework points
		previousPoint = point
	}

	export function click(_event: Event, projected: Point) {
		if ($snapPoints.length > 0) processPoint($snapPoints[0])
		else processPoint({ twoD: projected.twoD, threeD: projected.threeD, pointId: null })
	}

	export function mouseMove(_event: Event, projected: { x: number; y: number }) {
		// TODO: in the future, we should also snap to the midpoints of lines
		// and to the perimeters of circles and so on
		// so these snap points do not necessarily correspond to actual points in the sketch
		let snappedTo: PointLikeById | null = null

		for (const geom of $currentlyMousedOver) {
			log("[geom of $currentlyMousedOver]", geom)
			if (geom.type === "point3D") {
				if (geom.x && geom.y && geom.z) {
					const twoD = projectToPlane(new Vector3(geom.x, geom.y, geom.z))
					const point: PointLikeById = {
						twoD: { x: twoD.x, y: twoD.y },
						threeD: { x: geom.x, y: geom.y, z: geom.z },
						pointId: null
					} satisfies PointLikeById
					log("[point:PointById]", point)
					snappedTo = point
				}
			}
			if (geom.type === "point") {
				const point = pointsById[geom.id]
				log("[pointsById]", pointsById)
				snappedTo = {
					twoD: point.twoD,
					threeD: point.threeD,
					pointId: geom.id
				} satisfies PointLikeById
				log("[snappedTo]", snappedTo)
				break // If there is a 2D point, prefer to use it rather than the 3D point
			}
		}

		// only reset $snapPoints if something has changed
		if (snappedTo) $snapPoints = [snappedTo] satisfies PointLikeById[]
		else if ($snapPoints.length > 0) $snapPoints = []

		if (previousPoint) {
			let end: PointLikeById = { twoD: { x: projected.x, y: projected.y } } satisfies PointLikeById

			if (snappedTo) end = snappedTo

			// prettier-ignore
			const previewGeoms = [
				{ type: "line", start: previousPoint, end: end, uuid: `line-${end.twoD!.x}-${end.twoD!.y}` },
				{ type: "point", x: end.twoD!.x, y: end.twoD!.y, uuid: `point-${end.twoD!.x}-${end.twoD!.y}` }
			] satisfies PreviewGeometry[]

			if (previousPoint.pointId === null) {
				const p = {
					type: "point",
					x: previousPoint.twoD.x,
					y: previousPoint.twoD.y,
					uuid: `point-null-${previousPoint.twoD.x}-${previousPoint.twoD.y}`
				} satisfies PreviewGeometry
				previewGeoms.push(p)
			}

			previewGeometry.set(previewGeoms)
		} else previewGeometry.set([])
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

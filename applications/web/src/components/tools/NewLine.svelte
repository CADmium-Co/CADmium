<script lang="ts">
  import {snapPoints, sketchTool, previewGeometry, currentlyMousedOver} from "shared/stores"
  import {addLineToSketch, addPointToSketch} from "shared/projectUtils"
  import {Vector3} from "three"
  import type {IDictionary, Point, PointLikeById, PreviewGeometry, ProjectToPlane, SketchPoint} from "shared/types"

  const log = (function () { const context = "[NewLineTool.svelte]"; const color="gray"; return Function.prototype.bind.call(console.log, console, `%c${context}`, `font-weight:bold;color:${color};`)})() // prettier-ignore

  export let pointsById: IDictionary<SketchPoint>, sketchIndex: string, active: boolean, projectToPlane: ProjectToPlane

  let previousPoint: PointLikeById | null = null

  let stack: PointLikeById[] = []

  $: if ($sketchTool !== "line") clearStack()

  function pushToStack(point: PointLikeById) {
    // point should have the following properties:
    // - twoD: an object with x and y properties representing the point in 2D space
    // - threeD: an object with x, y, and z properties representing the point in 3D space
    // - id: a string representing the id of the point in the sketch
    // If the id is nullish we call addPointToSketch to create a new point in the sketch.
    if (!point) return
    point.id = point.id ?? addPointToSketch(sketchIndex, point.twoD, false)
    stack.push(point)
  }

  function processPoint(point: PointLikeById) {
    pushToStack(point)

    switch (stack.length) {
      case 0: // nothing to do, the stack is empty
        break
      case 1: // can't create a line with only one point!
        break
      default:
        const endPoint = popFromStack()
        const startPoint = popFromStack()
        addLineToSketch(sketchIndex, +startPoint.id, +endPoint.id)

        // leave the current point on the stack in case we want to create another line from here
        pushToStack(point)
        break
    }
    previousPoint = point
  }

  export function click(_event: Event, projected: Point) {
    if ($snapPoints.length > 0) {
      log("[click] [snapPoints]", $snapPoints)
      processPoint($snapPoints[0])
    } else processPoint({twoD: projected.twoD, threeD: projected.threeD, id: null})
  }

  export function mouseMove(_event: Event, projected: {x: number; y: number}) {
    // TODO: in the future, we should also snap to the midpoints of lines
    // and to the perimeters of circles and so on
    // so these snap points do not necessarily correspond to actual points in the sketch
    let snappedTo: PointLikeById | null = null

    for (const geom of $currentlyMousedOver) {
      if (geom.type === "point3D") {
        if (geom.x && geom.y && geom.z) {
          const twoD = projectToPlane(new Vector3(geom.x, geom.y, geom.z))
          const point: PointLikeById = {
            twoD: {x: twoD.x, y: twoD.y},
            threeD: {x: geom.x, y: geom.y, z: geom.z},
            id: null,
          } satisfies PointLikeById
          snappedTo = point
        }
      }
      if (geom.type === "point") {
        const point = pointsById[geom.id]
        snappedTo = {
          twoD: point.twoD,
          threeD: point.threeD,
          id: geom.id,
        } satisfies PointLikeById
        break // If there is a 2D point, prefer to use it rather than the 3D point
      }
    }

    // only reset $snapPoints if something has changed
    if (snappedTo) $snapPoints = [snappedTo] satisfies PointLikeById[]
    else if ($snapPoints.length > 0) $snapPoints = []

    if (previousPoint) {
      let end: PointLikeById = {twoD: {x: projected.x, y: projected.y}} satisfies PointLikeById

      if (snappedTo) end = snappedTo

      const previewGeoms = [
        {type: "line", start: previousPoint, end: end, uuid: `line-${end.twoD!.x}-${end.twoD!.y}`},
        {type: "point", x: end.twoD!.x, y: end.twoD!.y, uuid: `point-${end.twoD!.x}-${end.twoD!.y}`},
      ] satisfies PreviewGeometry[]

      if (previousPoint.id === null) {
        const p = {
          type: "point",
          x: previousPoint.twoD!.x,
          y: previousPoint.twoD!.y,
          uuid: `point-null-${previousPoint.twoD!.x}-${previousPoint.twoD!.y}`,
        } satisfies PreviewGeometry
        previewGeoms.push(p)
      }

      previewGeometry.set(previewGeoms)
    } else previewGeometry.set([])
  }

  export function onKeyDown(event: KeyboardEvent) {
    if (!active) return
    if (event.key === "Escape") {
      clearStack()
      $sketchTool = "select"
    }
  }

  function clearStack() {
    previousPoint = null
    previewGeometry.set([])
    snapPoints.set([])
    stack = []
  }

  function popFromStack(): PointLikeById | undefined {
    return stack.pop()
  }
</script>

<svelte:window on:keydown={onKeyDown} />

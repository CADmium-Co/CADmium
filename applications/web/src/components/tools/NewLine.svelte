<script lang="ts">
  import {snapPoints, sketchTool, previewGeometry, currentlyMousedOver} from "shared/stores"
  import {bench} from "shared/projectUtils"
  import {Vector2, Vector3} from "three"
  import type {IDictionary, Point2WithID, PreviewGeometry, ProjectToPlane} from "shared/types"
  import type {Point2} from "cadmium"

  // @ts-ignore
  const log = (function () { const context = "[NewLineTool.svelte]"; const color="gray"; return Function.prototype.bind.call(console.log, console, `%c${context}`, `font-weight:bold;color:${color};`)})() // prettier-ignore

  export let pointsById: IDictionary<Point2>, sketchIndex: string, active: boolean, projectToPlane: ProjectToPlane

  let previousPoint: Point2WithID | null = null

  let stack: Point2WithID[] = []

  $: if ($sketchTool !== "line") clearStack()

  function pushToStack(point: Point2WithID) {
    // point should have the following properties:
    // - twoD: an object with x and y properties representing the point in 2D space
    // - threeD: an object with x, y, and z properties representing the point in 3D space
    // - id: a string representing the id of the point in the sketch
    // If the id is nullish we call addPointToSketch to create a new point in the sketch.
    if (!point) return
    point.id = point.id ?? bench.sketchAddPoint(sketchIndex, point.x, point.y).data
    stack.push(point)
  }

  function processPoint(point: Point2WithID) {
    pushToStack(point)
    previousPoint = point

    if (stack.length > 1) {
      const endPoint = popFromStack()
      const startPoint = popFromStack()
      bench.sketchAddLine(sketchIndex, startPoint!.id!, endPoint!.id!)

      // leave the current point on the stack in case we want to create another line from here
      pushToStack(point)
      // unless it's an earlier point, which means we're finished making lines, so we clear the stack
      const isEarlierPoint = pointsById[point.id!]
      if (isEarlierPoint) clearStack()
    }
  }

  export function click(_event: Event, projected: Vector2) {
    if ($snapPoints.length > 0) processPoint($snapPoints[0])
    else processPoint({x: projected.x, y: projected.y, hidden: false} as Point2WithID)
  }

  export function mouseMove(_event: Event, projected: {x: number; y: number}) {
    // TODO: in the future, we should also snap to the midpoints of lines
    // and to the perimeters of circles and so on
    // so these snap points do not necessarily correspond to actual points in the sketch
    let snappedTo: Point2WithID | null = null

    for (const geom of $currentlyMousedOver) {
      if (geom.type === "point3D") {
        if (geom.x && geom.y && geom.z) {
          const twoD = projectToPlane(new Vector3(geom.x, geom.y, geom.z))
          snappedTo = { x: twoD.x, y: twoD.y, hidden: false } as Point2WithID
        }
      }
      if (geom.type === "point") {
        snappedTo = pointsById[geom.id]
        break // If there is a 2D point, prefer to use it rather than the 3D point
      }
    }

    // only reset $snapPoints if something has changed
    if (snappedTo) $snapPoints = [snappedTo] satisfies Point2WithID[]
    else if ($snapPoints.length > 0) $snapPoints = []

    if (previousPoint) {
      let end: Point2WithID = {x: projected.x, y: projected.y, hidden: false} as Point2WithID

      if (snappedTo) end = snappedTo

      const previewGeoms = [
        {type: "line", start: previousPoint, end: end, uuid: `line-${end.x}-${end.y}`},
        {type: "point", x: end.x, y: end.y, uuid: `point-${end.x}-${end.y}`},
      ] satisfies PreviewGeometry[]

      if (previousPoint.id === null) {
        const p = {
          type: "point",
          x: previousPoint.x,
          y: previousPoint.y,
          uuid: `point-null-${previousPoint.x}-${previousPoint.y}`,
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

  function popFromStack(): Point2WithID | undefined {
    return stack.pop()
  }
</script>

<svelte:window on:keydown={onKeyDown} />

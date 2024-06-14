<script lang="ts">
  import {snapPoints, sketchTool, previewGeometry, currentlyMousedOver} from "shared/stores"
  import {bench} from "shared/projectUtils"
  import {Vector3, type Vector2Like, type Vector3Like} from "three"
  import type {PointLikeById, Point2D, ProjectToPlane, IDictionary, PointById} from "shared/types"
  import type { Point2 } from "cadmium"

  // @ts-ignore
  const log = (function () { const context = "[NewCircleTool.svelte]"; const color="gray"; return Function.prototype.bind.call(console.log, console, `%c${context}`, `font-weight:bold;color:${color};`)})() // prettier-ignore

  export let pointsById: IDictionary<Point2>
  export let sketchId: string
  export let active: boolean
  export let projectToPlane: ProjectToPlane

  let centerPoint: Point2 | null = null
  let stack: Point2[] = []

  $: if ($sketchTool !== "circle") clearStack()

  function pushToStack(point: PointById) {
    if (!point) return
    point.id = point.id ?? bench.sketchAddPoint(sketchId, point.twoD.x, point.threeD.y).data
    stack.push(point)
  }

  function processPoint(point: PointById) {
    pushToStack(point)
    centerPoint = point

    switch (stack.length) {
      case 0: // nothing to do, the stack is empty
        break
      case 1: // can't create a circle with only one point!
        break
      default:
        const circumference = popFromStack()
        const center = popFromStack()
        bench.sketchAddCircle(sketchId, center!.id!, circumference!.id!)
        clearStack()
        break
    }
  }

  export function click(_event: Event, projected: {twoD: Vector2Like; threeD: Vector3Like}) {
    if ($snapPoints.length > 0) processPoint($snapPoints[0])
    else {
      let pt: PointLikeById = {twoD: projected.twoD, threeD: projected.threeD, id: null}
      processPoint(pt)
    }
  }

  export function mouseMove(_event: Event, projected: {x: number; y: number}) {
    // search through the existing points to see if we're close to one
    // if we are, then we should snap to it

    // TODO: in the future, we should also snap to the midpoints of lines
    // and to the perimeters of circles and so on
    // so these snap points do not necessarily correspond to actual points in the sketch
    let snappedTo = null
    for (const geom of $currentlyMousedOver) {
      if (geom.type === "point3D") {
        const twoD = projectToPlane(new Vector3(geom.x, geom.y, geom.z))
        snappedTo = {
          twoD: {x: twoD.x, y: twoD.y},
          threeD: {x: geom.x, y: geom.y, z: geom.z},
          id: null,
        }
      }
      if (geom.type === "point") {
        const point = pointsById[geom.id]
        if (point && geom.id) snappedTo = {point: point, id: geom.id}
        break // If there is a 2D point, prefer to use it rather than the 3D point
      }
    }

    // @ts-ignore todo rework snapping
    if (snappedTo) $snapPoints = [snappedTo]
    else if ($snapPoints.length > 0) $snapPoints = []

    if (centerPoint) {
      function calcDeltas(a: Vector2Like | Point2D | {x: number; y: number}, b: Vector2Like | undefined) {
        const dx = a.x - b?.x!
        const dy = a.y - b?.y!
        return Math.hypot(dx, dy)
      }
      const radius = snappedTo ? calcDeltas(snappedTo.point!, centerPoint.twoD) : calcDeltas(projected, centerPoint.twoD)

      previewGeometry.set([
        {
          type: "circle",
          center: centerPoint,
          radius,
          uuid: `circle-${centerPoint.twoD?.x}-${centerPoint.twoD?.y}-${radius}`,
        },
        {
          type: "point",
          x: centerPoint.twoD?.x,
          y: centerPoint.twoD?.y,
          uuid: `point-${centerPoint.twoD?.x}-${centerPoint.twoD?.y}`,
        },
      ])
    } else {
      previewGeometry.set([])
    }
  }

  export function onKeyDown(event: KeyboardEvent) {
    if (!active) return
    if (event.key === "Escape") {
      clearStack()
      $sketchTool = "select"
    }
  }

  function clearStack() {
    centerPoint = null
    previewGeometry.set([])
    snapPoints.set([])
    stack = []
  }

  function popFromStack(): PointLikeById | undefined {
    return stack.pop()
  }
</script>

<svelte:window on:keydown={onKeyDown} />

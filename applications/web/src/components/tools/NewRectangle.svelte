<script lang="ts">
  import {snapPoints, sketchTool, previewGeometry, currentlyMousedOver} from "shared/stores"
  import {addRectangleBetweenPoints, addPointToSketch} from "shared/projectUtils"
  import {Vector3} from "three"
  import type {IDictionary, PointLikeById, ProjectToPlane, SketchPoint} from "shared/types"

  const log = (function () { const context = "[NewRectangleTool.svelte]"; const color="gray"; return Function.prototype.bind.call(console.log, console, `%c${context}`, `font-weight:bold;color:${color};`)})() // prettier-ignore

  export let pointsById: IDictionary<SketchPoint>, sketchIndex: string, active: boolean, projectToPlane: ProjectToPlane

  let anchorPoint: PointLikeById | null = null

  let stack: PointLikeById[] = []

  $: if ($sketchTool !== "rectangle") clearStack()

  function pushToStack(point: PointLikeById) {
    if (!point) return
    point.id = point.id ?? addPointToSketch(sketchIndex, point.twoD, false)
    stack.push(point)
  }

  function processPoint(point: PointLikeById) {
    pushToStack(point)
    anchorPoint = point

    switch (stack.length) {
      case 0: // nothing to do, the stack is empty
        break
      case 1: // can't create a rectangle with only one point!
        break
      default:
        const endPoint = popFromStack()
        const anchor = popFromStack()
        addRectangleBetweenPoints(sketchIndex, +anchor.id, +endPoint.id)
        clearStack()
        break
    }
  }

  export function click(_event: Event, projected: PointLikeById) {
    if ($snapPoints.length > 0) processPoint($snapPoints[0])
    else processPoint({twoD: projected.twoD, threeD: projected.threeD, id: null})
  }

  export function mouseMove(_event: Event, projected: PointLikeById) {
    // search through the existing points to see if we're close to one
    // if we are, then we should snap to it

    // TODO: in the future, we should also snap to the midpoints of lines
    // and to the perimeters of circles and so on
    // so these snap points do not necessarily correspond to actual points in the sketch
    let snappedTo
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
        // @ts-ignore  todo make point etc factory functions and tighten types - find different solution than nulling ids
        snappedTo = {twoD: point.twoD, threeD: point.threeD, id: geom.id}
        break // If there is a 2D point, prefer to use it rather than the 3D point
      }
    }

    // only reset $snapPoints if something has changed
    // @ts-ignore
    if (snappedTo) $snapPoints = [snappedTo]
    else if ($snapPoints.length > 0) $snapPoints = []

    if (anchorPoint) {
      const end = snappedTo ? snappedTo : {twoD: {x: projected.x, y: projected.y}}
      const upperLeft = {twoD: {x: anchorPoint.twoD?.x, y: end.twoD.y}}
      const lowerRight = {twoD: {x: end.twoD.x, y: anchorPoint.twoD?.y}}

      const previewGeoms = [
        {
          type: "point",
          x: upperLeft.twoD.x,
          y: upperLeft.twoD.y,
          uuid: `point-ul-${upperLeft.twoD.x}-${upperLeft.twoD.y}`,
        },
        {
          type: "point",
          x: lowerRight.twoD.x,
          y: lowerRight.twoD.y,
          uuid: `point-lr-${lowerRight.twoD.x}-${lowerRight.twoD.y}`,
        },
        {
          type: "point",
          x: end.twoD.x,
          y: end.twoD.y,
          uuid: `point-end-${end.twoD.x}-${end.twoD.y}`,
        },
        {
          type: "line",
          start: anchorPoint,
          end: upperLeft,
          uuid: `line-s-ul-${upperLeft.twoD.x}-${upperLeft.twoD.y}`,
        },

        {
          type: "line",
          start: anchorPoint,
          end: lowerRight,
          uuid: `line-s-lr-${lowerRight.twoD.x}-${lowerRight.twoD.y}`,
        },
        {
          type: "line",
          start: upperLeft,
          end: end,
          uuid: `line-ul-end-${end.twoD.x}-${end.twoD.y}`,
        },
        {
          type: "line",
          start: lowerRight,
          end: end,
          uuid: `line-lr-end-${end.twoD.x}-${end.twoD.y}`,
        },
      ]

      if (anchorPoint.id === null) {
        previewGeoms.push({
          type: "point",
          x: anchorPoint.twoD?.x,
          y: anchorPoint.twoD?.y,
          uuid: `point-null-${anchorPoint.twoD?.x}-${anchorPoint.twoD?.y}`,
        })
      }
      // @ts-ignore todo make factory functions so type is EntityType
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
    anchorPoint = null
    previewGeometry.set([])
    snapPoints.set([])
    stack = []
  }

  function popFromStack(): PointLikeById | undefined {
    return stack.pop()
  }
</script>

<svelte:window on:keydown={onKeyDown} />

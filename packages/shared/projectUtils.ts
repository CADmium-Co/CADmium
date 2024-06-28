import type {Message} from "./cadmium-api"

import {workbenchIsStale, workbenchIndex, workbench, project, featureIndex, wasmProject, projectIsStale, messageHistory, workbenchSolids} from "./stores"
import {get} from "svelte/store"
import {Vector2, Vector3, type Vector2Like} from "three"
import type {Entity, MessageHistory, WithTarget} from "./types"
import type {Workbench, MessageResult, Solid, Step, Point3, Plane, ISketch, StepHash} from "cadmium"
import {isSketchActionStep} from "./stepTypeGuards"

import * as cad from "./cadmium-api"
(window as any).cad = cad
export { cad }

// prettier-ignore
const log = (function () { const context = "[projectUtils.ts]"; const color = "aqua"; return Function.prototype.bind.call(console.log, console, `%c${context}`, `font-weight:bold;color:${color};`) })()

export const CIRCLE_TOLERANCE = 0.05

function createWrapperFunctions(originalNamespace: any, newNamespace: any) {
  Object.keys(originalNamespace).forEach(funcName => {
    const originalFunction = originalNamespace[funcName]
    if (typeof originalFunction === "function") {
      newNamespace[funcName] = (...args: any[]) => {
        const workbench_id = get(workbenchIndex)
        return originalFunction(workbench_id.toString(), ...args)
      }
    }
  })
}

export namespace bench {
  createWrapperFunctions(cad, bench)
}
(window as any).bench = bench

export function arraysEqual(a: any[], b: any[]) {
  if (a.length !== b.length) return false
  for (let i = 0; i < a.length; i++) {
    if (a[i] !== b[i]) return false
  }
  return true
}

export function sendWasmMessage(message: Message): MessageResult {
  let wp = get(wasmProject)
  log("[sendWasmMessage] sending message:", message)
  // TODO: send_message should accept the generated Message type (cadmium-api.ts), not the cadmium Message type
  let result = wp.send_message(message as any)
  log("[sendWasmMessage] reply:", result)

  messageHistory.update((history: MessageHistory[]) => {
    log("[sendWasmMessage] [messageHistory.update] update:", {message, result})
    return [...history, {message, result}]
  })

  if (!result.success) {
    throw new Error(`[sendWasmMessage] message failed: ${result.data}`)
  } else {
    workbenchIsStale.set(true)
  }

  return result
}

export function updateExtrusion(extrusionId: number, sketchId: number, length: number, faceIds: string[]) {
  const message: Message = {
    UpdateExtrusion: {
      workbench_id: get(workbenchIndex),
      sketch_id: sketchId,
      face_ids: faceIds.map(id => +id), // on browser side ids are strings - coerce face ids to numbers here to suit rust
      length,
      offset: 0.0,
      extrusion_name: "Extra",
      direction: "Normal",
      extrusion_id: extrusionId,
    },
  }

  sendWasmMessage(message)
}

export function newExtrusion() {
  const bench: Workbench = get(workbench)
  // log("[newExtrusion] workbench:", workbench)
  // log("[newExtrusion] bench:", bench)

  let faces: number[] = []
  for (let step of bench.history.reverse()) {
    console.warn("[newExtrusion] step:", step)
    if (isSketchActionStep(step) && faces.length === 0) {
      faces = Array.from({ length: step.result.faces.length }, (_value, index) => index);
      console.log("[newExtrusion] faces:", faces)
      return cad.featureExtrusionAdd(get(workbenchIndex).toString(), step.data.sketch_id, faces, 25, 0.0, "Normal", "New")
    }
  }

  throw new Error("No sketch faces found to extrude")
}

export function deleteEntities(sketchIdx: string, selection: Entity[]) {
  const workbenchIdx = get(workbenchIndex).toString()

  // TODO: Handle compounds as well
  for (const entity of selection) {
    cad.sketchDeletePrimitive(workbenchIdx, sketchIdx, parseInt(entity.id))
  }
}

// If the project ever becomes stale, refresh it. This should be pretty rare.
projectIsStale.subscribe(value => {
  if (value) {
    const wp = get(wasmProject)
    project.set(JSON.parse(wp.to_json()))

    workbenchIndex.set(0)
    workbenchIsStale.set(true)
    projectIsStale.set(false)
    // @ts-ignore
    log("[projectIsStale] Refreshing project", "value:", value, "wasmProject:", wp, "project:", project)
  }
})

// If the workbench ever becomes stale, refresh it. This should be very common.
// Every time you edit any part of the feature history, for example
workbenchIsStale.subscribe(value => {
  if (value) {
    log("[workbenchIsStale] Workbench:", get(workbench))
    const workbenchIdx = get(workbenchIndex)
    const wasmProj = get(wasmProject)
    const workbenchJson = wasmProj.get_workbench(workbenchIdx)
    workbench.set(workbenchJson)
    workbenchIsStale.set(false)
    // workbenchSolids.set(getWorkbenchSolids())
  }
})

export function getObjectString(solidId: string): string {
  const solids = get(workbenchSolids)
  // TODO: Does this work?
  const objString = solids[solidId].solid_to_obj(solidId, 0.1)
  return objString
}

export function readFile(e: WithTarget<Event, HTMLInputElement>): void {
  const target = e.target as HTMLInputElement
  const file = target.files![0]
  const reader = new FileReader()
  reader.onload = function (e) {
    // log("[readFile] file contents", e.target?.result)
  }
  reader.readAsText(file)
}

export function arcToPoints(center: Vector2, start: Vector2, end: Vector2, clockwise: boolean = false): Vector2[] {
  // log("[arcToPoints] center, start, end, clockwise", center, start, end, clockwise)
  // see https://math.stackexchange.com/a/4132095/816177
  const tolerance = CIRCLE_TOLERANCE // in meters
  const radius = start.distanceTo(center)
  const k = tolerance / radius
  // more precise but slower to calculate:
  // const n = Math.ceil(Math.PI / Math.acos(1 - k))
  // faster to calculate, at most only overestimates by 1:
  let n = Math.ceil(Math.PI / Math.sqrt(2 * k))
  const segmentAngle = (2 * Math.PI) / n
  const segmentLength = radius * segmentAngle
  if (clockwise) n = -n

  const startAngle = Math.atan2(start.y - center.y, start.x - center.x)

  const lineVertices: Vector2[] = []
  lineVertices.push(start.clone())
  for (let i = 1; i <= Math.abs(n); i++) {
    const theta = ((2 * Math.PI) / n) * i + startAngle
    const xComponent = radius * Math.cos(theta)
    const yComponent = radius * Math.sin(theta)
    const point = new Vector2(xComponent, yComponent).add(center)
    lineVertices.push(point)

    const distanceToEnd = point.distanceTo(end)
    if (distanceToEnd <= segmentLength) {
      lineVertices.push(end.clone())
      break
    }
  }
  return lineVertices
}

export function circleToPoints(centerPoint: Vector2Like, radius: number): Vector2[] {
  // this is 2D function

  // see https://math.stackexchange.com/a/4132095/816177
  const tolerance = CIRCLE_TOLERANCE // in meters
  const k = tolerance / radius
  // more precise but slower to calculate:
  // const n = Math.ceil(Math.PI / Math.acos(1 - k))
  // faster to calculate, at most only overestimates by 1:
  const n = Math.ceil(Math.PI / Math.sqrt(2 * k))

  const lineVertices: Vector2[] = []
  for (let i = 0; i <= n; i++) {
    const theta = ((2 * Math.PI) / n) * i
    const xComponent = radius * Math.cos(theta)
    const yComponent = radius * Math.sin(theta)
    const point = new Vector2(xComponent, yComponent).add(centerPoint)
    lineVertices.push(point)
  }
  return lineVertices
}

export function promoteTo3(points: Vector2[]): Vector3[] {
  const points3: Vector3[] = []
  for (const point of points) {
    points3.push(new Vector3(point.x, point.y, 0))
  }
  return points3
}

export function flatten(points: Vector3[]): number[] {
  const pointsFlat: number[] = []
  for (const point of points) {
    pointsFlat.push(point.x, point.y, point.z)
  }
  return pointsFlat
}

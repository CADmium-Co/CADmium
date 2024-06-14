import type { Arc2, Circle2, Line2, Point2, Solid, Step, SketchActionResult } from "cadmium"
import { FeatureExtrusionAdd, SketchAddArc, SketchAddCircle, SketchAddLine, SketchAddPoint, WorkbenchPlaneAdd, WorkbenchPointAdd, WorkbenchSketchAdd } from "./cadmium-api"

// --- Workbench operations ---
export type PointStep = Step & {result: { type: "Point" }, data: {type: "WorkbenchPointAdd"} & WorkbenchPointAdd}
export function isPointStep(step: Step): step is PointStep {
  return step.data.type === "WorkbenchPointAdd" && step.result.type === "Point"
}

export type PlaneStep = Step & {result: { type: "Plane" }, data: {type: "WorkbenchPlaneAdd"} & WorkbenchPlaneAdd }
export function isPlaneStep(step: Step): step is PlaneStep {
  return step.data.type === "WorkbenchPlaneAdd" && step.result.type === "Plane"
}

export type SketchStep = Step & {result: { type: "Sketch" }, data: {type: "WorkbenchSketchAdd"} & WorkbenchSketchAdd }
export function isSketchStep(step: Step): step is SketchStep {
  return step.data.type === "WorkbenchSketchAdd" && step.result.type === "Sketch"
}

// --- Sketch primitive operations ---
// Any primitive operation
export type SketchActionStep = Step & {result: { type: "SketchAction", action: SketchActionResult & {type: string}, faces: Face[]}}
export function isSketchActionStep(step: Step): step is SketchActionStep {
  return step.result.type === "SketchAction"
}

export type SketchPointStep = SketchActionStep & {result: {action: {type: "Point2"} & Point2}, data: {type: "SketchAddPoint"} & SketchAddPoint}
export function isSketchPointStep(step: Step): step is SketchPointStep {
  return isSketchActionStep(step) && step.result.action.type === "Point2" && step.data.type === "SketchAddPoint"
}

export type SketchLineStep = SketchActionStep & {result: {action: {type: "Line2"} & Line2}, data: {type: "SketchAddLine"} & SketchAddLine}
export function isSketchLineStep(step: Step): step is SketchLineStep {
  return isSketchActionStep(step) && step.result.action.type === "Line2" && step.data.type === "SketchAddLine"
}

export type SketchCircleStep = SketchActionStep & {result: {action: {type: "Circle2"} & Circle2}, data: {type: "SketchAddCircle"} & SketchAddCircle}
export function isSketchCircleStep(step: Step): step is SketchCircleStep {
  return isSketchActionStep(step) && step.result.action.type === "Circle2" && step.data.type === "SketchAddCircle"
}

export type SketchArcStep = SketchActionStep & {result: {action: {type: "Arc2"} & Arc2}, data: {type: "SketchAddArc"} & SketchAddArc}
export function isSketchArcStep(step: Step): step is SketchArcStep {
  return isSketchActionStep(step) && step.result.action.type === "Arc2" && step.data.type === "SketchAddArc"
}

// TODO: export rectangle
// export type SketchRectangleStep = SketchCompoundStep & {result: Rectangle, data: SketchAddRectangle}
// export function isSketchRectangleStep(step: Step): step is SketchRectangleStep {
//   return isSketchCompoundStep(step) && "SketchAddRectangle" in step.data && "Rectangle" in step.result.Compound
// }

// --- Solid operations ---
// Any step that produces solids is a solid step
export type SolidStep = Step & {result: {type: "Solid", solids: Solid[]}}
export function isSolidStep(step: Step): step is SolidStep {
  return step.result.type === "Solid"
}

export type ExtrusionStep = SolidStep & {data: {type: "FeatureExtrusionAdd"} & FeatureExtrusionAdd}
export function isExtrusionStep(step: Step): step is ExtrusionStep {
  return isSolidStep(step) && step.data.type === "FeatureExtrusionAdd"
}

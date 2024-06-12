import type { Arc2, Circle2, Compound, IDType, ISketch, Line2, StepResult, Plane, Point2, Point3, Primitive, Solid, Step, WrappedPrimitive } from "cadmium"
import { FeatureExtrusionAdd, SketchAddArc, SketchAddCircle, SketchAddLine, SketchAddPoint, SketchAddRectangle, WorkbenchPlaneAdd, WorkbenchPointAdd, WorkbenchSketchAdd } from "./cadmium-api"

// --- Workbench operations ---
export type PointStep = Step & {result: { Point: Point3 }, data: {WorkbenchPointAdd: WorkbenchPointAdd}}
export function isPointStep(step: Step): step is PointStep {
  return "WorkbenchPointAdd" in step.data && typeof step.result === "object" && "Point" in step.result
}

export type PlaneStep = Step & {result: { Plane: Plane }, data: {WorkbenchPlaneAdd: WorkbenchPlaneAdd} }
export function isPlaneStep(step: Step): step is PlaneStep {
  return "WorkbenchPlaneAdd" in step.data && typeof step.result === "object" && "Plane" in step.result
}

export type SketchStep = Step & {result: { Sketch: { sketch: ISketch, faces: Face[] } }, data: {WorkbenchSketchAdd: WorkbenchSketchAdd} }
export function isSketchStep(step: Step): step is SketchStep {
  return "WorkbenchSketchAdd" in step.data && typeof step.result === "object" && "Sketch" in step.result
}

// --- Sketch primitive operations ---
// Any primitive operation
export type SketchPrimitiveStep = Step & {result: {Primitive: WrappedPrimitive}}
export function isSketchPrimitiveStep(step: Step): step is SketchPrimitiveStep {
  return typeof step.result === "object" && "Primitive" in step.result
}

export type SketchPointStep = SketchPrimitiveStep & {result: {Point2: Point2}, data: {SketchAddPoint: SketchAddPoint}}
export function isSketchPointStep(step: Step): step is SketchPointStep {
  return isSketchPrimitiveStep(step) && "SketchAddPoint" in step.data && "Point2" in step.result
}

export type SketchLineStep = SketchPrimitiveStep & {result: {Line2: Line2}, data: {SketchAddLine: SketchAddLine}}
export function isSketchLineStep(step: Step): step is SketchLineStep {
  return isSketchPrimitiveStep(step) && "SketchAddLine" in step.data && "Line2" in step.result
}

export type SketchCircleStep = SketchPrimitiveStep & {result: {Circle2: Circle2}, data: {SketchAddCircle: SketchAddCircle}}
export function isSketchCircleStep(step: Step): step is SketchCircleStep {
  return isSketchPrimitiveStep(step) && "SketchAddCircle" in step.data && "Circle2" in step.result
}

export type SketchArcStep = SketchPrimitiveStep & {result: {Arc2: Arc2}, data: {SketchAddArc: SketchAddArc}}
export function isSketchArcStep(step: Step): step is SketchArcStep {
  return isSketchPrimitiveStep(step) && "SketchAddArc" in step.data && "Arc2" in step.result
}

// --- Sketch compound operations ---
export type SketchCompoundStep = Step & {result: {Compound: Compound}}
export function isSketchCompoundStep(step: Step): step is SketchCompoundStep {
  return typeof step.result === "object" && "Compound" in step.result
}

// TODO: export rectangle
// export type SketchRectangleStep = SketchCompoundStep & {result: Rectangle, data: SketchAddRectangle}
// export function isSketchRectangleStep(step: Step): step is SketchRectangleStep {
//   return isSketchCompoundStep(step) && "SketchAddRectangle" in step.data && "Rectangle" in step.result
// }

// --- Solid operations ---
// Any step that produces solids is a solid step
export type SolidStep = Step & {result: {Solid: Solid[]}}
export function isSolidStep(step: Step): step is SolidStep {
  return "Solid" in step
}

export type ExtrusionStep = SolidStep & {data: FeatureExtrusionAdd}
export function isExtrusionStep(step: Step): step is ExtrusionStep {
  return isSolidStep(step) && "FeatureExtrusionAdd" in step.data
}

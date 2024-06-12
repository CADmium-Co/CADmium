import { Arc2, Circle2, Compound, IDType, ISketch, Line2, Node, Plane, Point2, Point3, Primitive, Solid, Step } from "cadmium"
import { FeatureExtrusionAdd, SketchAddArc, SketchAddCircle, SketchAddLine, SketchAddPoint, SketchAddRectangle, WorkbenchPlaneAdd, WorkbenchPointAdd, WorkbenchSketchAdd } from "./cadmium-api"

// --- Workbench operations ---
export type PointStep = Step & {interop_node: Point3, data: WorkbenchPointAdd}
export function isPointStep(step: Step): step is PointStep {
  return "WorkbenchPointAdd" in step.data && "Point" in step.interop_node!
}

export type PlaneStep = Step & {interop_node: Plane, data: WorkbenchPlaneAdd }
export function isPlaneStep(step: Step): step is PlaneStep {
  return "WorkbenchPlaneAdd" in step.data && "Plane" in step.interop_node!
}

export type SketchStep = Step & {interop_node: ISketch, data: WorkbenchSketchAdd }
export function isSketchStep(step: Step): step is SketchStep {
  return "WorkbenchSketchAdd" in step.data && "Sketch" in step.interop_node!
}

// --- Sketch primitive operations ---
// Any primitive operation
export type SketchPrimitiveStep = Step & {interop_node: Primitive}
export function isSketchPrimitiveStep(step: Step): step is SketchPrimitiveStep {
  return "Primitive" in step.interop_node!
}

export type SketchPointStep = SketchPrimitiveStep & {interop_node: Point2, data: SketchAddPoint}
export function isSketchPointStep(step: Step): step is SketchPointStep {
  return isSketchPrimitiveStep(step) && "SketchAddPoint" in step.data && "Point2" in step.interop_node!
}

export type SketchLineStep = SketchPrimitiveStep & {interop_node: Line2, data: SketchAddLine}
export function isSketchLineStep(step: Step): step is SketchLineStep {
  return isSketchPrimitiveStep(step) && "SketchAddLine" in step.data && "Line2" in step.interop_node!
}

export type SketchCircleStep = SketchPrimitiveStep & {interop_node: Circle2, data: SketchAddCircle}
export function isSketchCircleStep(step: Step): step is SketchCircleStep {
  return isSketchPrimitiveStep(step) && "SketchAddCircle" in step.data && "Circle2" in step.interop_node!
}

export type SketchArcStep = SketchPrimitiveStep & {interop_node: Arc2, data: SketchAddArc}
export function isSketchArcStep(step: Step): step is SketchArcStep {
  return isSketchPrimitiveStep(step) && "SketchAddArc" in step.data && "Arc2" in step.interop_node!
}

// --- Sketch compound operations ---
export type SketchCompoundStep = Step & {interop_node: Compound}
export function isSketchCompoundStep(step: Step): step is SketchCompoundStep {
  return "Compound" in step.interop_node!
}

// TODO: export rectangle
// export type SketchRectangleStep = SketchCompoundStep & {interop_node: Rectangle, data: SketchAddRectangle}
// export function isSketchRectangleStep(step: Step): step is SketchRectangleStep {
//   return isSketchCompoundStep(step) && "SketchAddRectangle" in step.data && "Rectangle" in step.interop_node!
// }

// --- Solid operations ---
// Any step that produces solids is a solid step
export type SolidStep = Step & {interop_node: Solid}
export function isSolidStep(step: Step): step is SolidStep {
  return "Solid" in step
}

export type ExtrusionStep = SolidStep & {data: FeatureExtrusionAdd}
export function isExtrusionStep(step: Step): step is ExtrusionStep {
  return isSolidStep(step) && "FeatureExtrusionAdd" in step.data
}

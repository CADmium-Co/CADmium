import type {
  IDictionary,
  WithTarget,
  SetCameraFocus,
  EntityType,
  Entity,
  CircleEntity,
  ArcEntity,
  FaceEntity,
  LineEntity,
  PlaneEntity,
  PointEntity,
  Point3DEntity,
  MeshFaceEntity,
  Project,
  WorkBench,
  PreviewGeometry,
  Plane,
  Point,
  PointById,
  SketchPoint,
  PointById,
  Vector2Vector3PointById,
  PointLikeById,
  PointsById,
  PointsLikeById,
  SnapEntity,
  ProjectToPlane,
  Point3D,
  Point2D,
  LineTuple,
  CircleTuple,
  ArcTuple,
  FaceTuple,
  HistoryStep,
  HistoryStepType,
  PointHistoryStep,
  PlaneHistoryStep,
  ExtrusionHistoryStep,
  SketchHistoryStep,
  PointData,
  PlaneData,
  ExtrusionData,
  SketchRealized,
  Arc,
  SketchData,
  SegmentId,
  Circle,
  TruckNurbsPoint,
  TruckNurbsSurfaceControlPoint,
  TruckNurbsSurfaceControlPoints,
  TruckNurbsSurface,
  TruckPlane,
  TruckSurface,
  TruckFaceBoundary,
  TruckFace,
  TruckNurbsCurve,
  TruckEdgeEndpoints,
  TruckEdge,
  TruckLineVectors,
  TruckLine,
  TruckCurve,
  SketchTuple,
  Realization,
  PlaneRealized,
  TruckFaceEdgeIndex,
  TruckBoundary,
  TruckSolid,
  SolidRealized,
  ExtrusionSketchData,
  UpdateExtrusion,
  SetSketchPlane,
  NewSketchOnPlane,
  NewExtrusion,
  DeleteLines,
  DeleteArcs,
  DeleteCircles,
  NewRectangleBetweenPoints,
  NewCircleBetweenPoints,
  NewLineOnSketch,
  NewPointOnSketch2,
  RenameStep,
  Message,
  MessageHistory,
  RenameWorkbench,
  RenameProject,
} from "./types"
import {Vector2} from "three"
import {Vector3} from "three"

export function isIDictionary(obj: unknown): obj is IDictionary<unknown> {
  const typedObj = obj as IDictionary<unknown>
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
    Object.entries<any>(typedObj).every(([key, value]) => value === "TValue" && typeof key === "string")
  )
}

export function isWithTarget(obj: unknown): obj is WithTarget<Event, unknown> {
  const typedObj = obj as WithTarget<Event, unknown>
  return (
    typeof typedObj === "Event" && // new MouseEvent("mouse") instanceof MouseEvent   -> true
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
    typeof typedObj["currentTarget"] === "Target"
  )
}

export function isSetCameraFocus(obj: unknown): obj is SetCameraFocus {
  const typedObj = obj as SetCameraFocus
  return typeof typedObj === "function"
}

export function isEntityType(obj: unknown): obj is EntityType {
  const typedObj = obj as EntityType
  return (
    typedObj === "circle" ||
    typedObj === "arc" ||
    typedObj === "face" ||
    typedObj === "line" ||
    typedObj === "plane" ||
    typedObj === "point" ||
    typedObj === "point3D" ||
    typedObj === "meshFace"
  )
}

export function isEntity(obj: unknown): obj is Entity {
  const typedObj = obj as Entity
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
    typeof typedObj["id"] === "string" &&
    (isEntityType(typedObj["type"]) as boolean) &&
    Object.keys(typedObj).length === 2
  )
}

export function isCircleEntity(obj: unknown): obj is CircleEntity {
  const typedObj = obj as CircleEntity
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
    typeof typedObj["id"] === "string" &&
    typedObj["type"] === "circle"
  )
}

export function isArcEntity(obj: unknown): obj is ArcEntity {
  const typedObj = obj as ArcEntity
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") && typeof typedObj["id"] === "string" && typedObj["type"] === "arc"
  )
}

export function isFaceEntity(obj: unknown): obj is FaceEntity {
  const typedObj = obj as FaceEntity
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") && typeof typedObj["id"] === "string" && typedObj["type"] === "face"
  )
}

export function isLineEntity(obj: unknown): obj is LineEntity {
  const typedObj = obj as LineEntity
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") && typeof typedObj["id"] === "string" && typedObj["type"] === "line"
  )
}

export function isPlaneEntity(obj: unknown): obj is PlaneEntity {
  const typedObj = obj as PlaneEntity
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
    typeof typedObj["id"] === "string" &&
    typedObj["type"] === "plane"
  )
}

export function isPointEntity(obj: unknown): obj is PointEntity {
  const typedObj = obj as PointEntity
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
    typeof typedObj["id"] === "string" &&
    typedObj["type"] === "point"
  )
}

export function isPoint3DEntity(obj: unknown): obj is Point3DEntity {
  const typedObj = obj as Point3DEntity
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
    typeof typedObj["id"] === "string" &&
    typedObj["type"] === "point3D"
  )
}

export function isMeshFaceEntity(obj: unknown): obj is MeshFaceEntity {
  const typedObj = obj as MeshFaceEntity
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
    typeof typedObj["id"] === "string" &&
    typedObj["type"] === "meshFace"
  )
}

export function isProject(obj: unknown): obj is Project {
  const typedObj = obj as Project
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
    typeof typedObj["name"] === "string" &&
    Array.isArray(typedObj["assemblies"]) &&
    Array.isArray(typedObj["workbenches"]) &&
    typedObj["workbenches"].every((e: any) => isWorkBench(e) as boolean)
  )
}

export function isWorkBench(obj: unknown): obj is WorkBench {
  const typedObj = obj as WorkBench
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
    typeof typedObj["name"] === "string" &&
    Array.isArray(typedObj["history"]) &&
    typedObj["history"].every((e: any) => isHistoryStep(e) as boolean) &&
    ((typedObj["step_counters"] !== null && typeof typedObj["step_counters"] === "object") || typeof typedObj["step_counters"] === "function") &&
    typeof typedObj["step_counters"]["Extrusion"] === "number" &&
    typeof typedObj["step_counters"]["Plane"] === "number" &&
    typeof typedObj["step_counters"]["Point"] === "number" &&
    typeof typedObj["step_counters"]["Sketch"] === "number"
  )
}

export function isPreviewGeometry(obj: unknown): obj is PreviewGeometry {
  const typedObj = obj as PreviewGeometry
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
    (isEntityType(typedObj["type"]) as boolean) &&
    (typeof typedObj["start"] === "undefined" || (isPointLikeById(typedObj["start"]) as boolean)) &&
    (typeof typedObj["end"] === "undefined" || (isPointLikeById(typedObj["end"]) as boolean)) &&
    (typeof typedObj["center"] === "undefined" || (isPointLikeById(typedObj["center"]) as boolean)) &&
    (typeof typedObj["radius"] === "undefined" || typeof typedObj["radius"] === "number") &&
    (typeof typedObj["x"] === "undefined" || typeof typedObj["x"] === "number") &&
    (typeof typedObj["y"] === "undefined" || typeof typedObj["y"] === "number") &&
    typeof typedObj["uuid"] === "string"
  )
}

export function isPlane(obj: unknown): obj is Plane {
  const typedObj = obj as Plane
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
    (isPoint3D(typedObj["origin"]) as boolean) &&
    ((typedObj["primary"] !== null && typeof typedObj["primary"] === "object") || typeof typedObj["primary"] === "function") &&
    typeof typedObj["primary"]["x"] === "number" &&
    typeof typedObj["primary"]["y"] === "number" &&
    typeof typedObj["primary"]["z"] === "number" &&
    ((typedObj["secondary"] !== null && typeof typedObj["secondary"] === "object") || typeof typedObj["secondary"] === "function") &&
    typeof typedObj["secondary"]["x"] === "number" &&
    typeof typedObj["secondary"]["y"] === "number" &&
    typeof typedObj["secondary"]["z"] === "number" &&
    ((typedObj["tertiary"] !== null && typeof typedObj["tertiary"] === "object") || typeof typedObj["tertiary"] === "function") &&
    typeof typedObj["tertiary"]["x"] === "number" &&
    typeof typedObj["tertiary"]["y"] === "number" &&
    typeof typedObj["tertiary"]["z"] === "number"
  )
}

export function isPoint(obj: unknown): obj is Point {
  const typedObj = obj as Point
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
    typedObj["twoD"] instanceof Vector2 &&
    typedObj["threeD"] instanceof Vector3
  )
}

export function isPointById(obj: unknown): obj is PointById {
  const typedObj = obj as PointById
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
    (isPoint2D(typedObj["twoD"]) as boolean) &&
    (isPoint3D(typedObj["threeD"]) as boolean) &&
    typeof typedObj["id"] === "string"
  )
}

export function isSketchPoint(obj: unknown): obj is SketchPoint {
  const typedObj = obj as SketchPoint
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
    (isPoint2D(typedObj["twoD"]) as boolean) &&
    (isPoint3D(typedObj["threeD"]) as boolean)
  )
}

export function isVector2Vector3PointById(obj: unknown): obj is Vector2Vector3PointById {
  const typedObj = obj as Vector2Vector3PointById
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
    typedObj["twoD"] instanceof Vector2 &&
    typedObj["threeD"] instanceof Vector3 &&
    (typedObj["id"] === null || typeof typedObj["id"] === "string")
  )
}

export function isPointLikeById(obj: unknown): obj is PointLikeById {
  const typedObj = obj as PointLikeById
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
    (typeof typedObj["twoD"] === "undefined" ||
      (isPoint2D(typedObj["twoD"]) as boolean) ||
      typedObj["twoD"] instanceof Vector2 ||
      (((typedObj["twoD"] !== null && typeof typedObj["twoD"] === "object") || typeof typedObj["twoD"] === "function") &&
        typeof typedObj["twoD"]["x"] === "number" &&
        typeof typedObj["twoD"]["y"] === "number")) &&
    (typeof typedObj["threeD"] === "undefined" ||
      (isPoint3D(typedObj["threeD"]) as boolean) ||
      (((typedObj["threeD"] !== null && typeof typedObj["threeD"] === "object") || typeof typedObj["threeD"] === "function") &&
        typeof typedObj["threeD"]["x"] === "number" &&
        typeof typedObj["threeD"]["y"] === "number" &&
        typeof typedObj["threeD"]["z"] === "number") ||
      typedObj["threeD"] instanceof Vector3) &&
    (typeof typedObj["id"] === "undefined" ||
      typedObj["id"] === null ||
      // typeof typedObj["id"] === "number" ||
      typeof typedObj["id"] === "string") &&
    Object.entries<any>(typedObj)
      .filter(([key]) => !["twoD", "threeD", "id"].includes(key))
      .every(([key, _value]) => typeof key === "string")
  )
}

export function isPointsById(obj: unknown): obj is PointsById {
  const typedObj = obj as PointsById
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
    Object.entries<any>(typedObj).every(([key, value]) => (isPointById(value) as boolean) && typeof key === "string")
  )
}

export function isPointsLikeById(obj: unknown): obj is PointsLikeById {
  const typedObj = obj as PointsLikeById
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
    Object.entries<any>(typedObj).every(([key, value]) => (isPointLikeById(value) as boolean) && typeof key === "string")
  )
}

export function isSnapEntity(obj: unknown): obj is SnapEntity {
  const typedObj = obj as SnapEntity
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
    typeof typedObj["id"] === "string" &&
    (isEntityType(typedObj["type"]) as boolean) &&
    (typeof typedObj["x"] === "undefined" || typeof typedObj["x"] === "number") &&
    (typeof typedObj["y"] === "undefined" || typeof typedObj["y"] === "number") &&
    (typeof typedObj["z"] === "undefined" || typeof typedObj["z"] === "number")
  )
}

export function isProjectToPlane(obj: unknown): obj is ProjectToPlane {
  const typedObj = obj as ProjectToPlane
  return typeof typedObj === "function"
}

export function isPoint3D(obj: unknown): obj is Point3D {
  const typedObj = obj as Point3D
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
    typeof typedObj["x"] === "number" &&
    typeof typedObj["y"] === "number" &&
    typeof typedObj["z"] === "number" &&
    typeof typedObj["hidden"] === "boolean"
  )
}

export function isPoint2D(obj: unknown): obj is Point2D {
  const typedObj = obj as Point2D
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
    typeof typedObj["x"] === "number" &&
    typeof typedObj["y"] === "number" &&
    typeof typedObj["m"] === "number" &&
    typeof typedObj["dx"] === "number" &&
    typeof typedObj["dy"] === "number" &&
    typeof typedObj["fx"] === "number" &&
    typeof typedObj["fy"] === "number" &&
    typeof typedObj["fixed"] === "boolean" &&
    typeof typedObj["hidden"] === "boolean"
  )
}

export function isLineTuple(obj: unknown): obj is LineTuple {
  const typedObj = obj as LineTuple
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
    typeof typedObj["id"] === "string" &&
    ((typedObj["start"] !== null && typeof typedObj["start"] === "object") || typeof typedObj["start"] === "function") &&
    (isPoint2D(typedObj["start"]["twoD"]) as boolean) &&
    (isPoint3D(typedObj["start"]["threeD"]) as boolean) &&
    ((typedObj["end"] !== null && typeof typedObj["end"] === "object") || typeof typedObj["end"] === "function") &&
    (isPoint2D(typedObj["end"]["twoD"]) as boolean) &&
    (isPoint3D(typedObj["end"]["threeD"]) as boolean)
  )
}

export function isCircleTuple(obj: unknown): obj is CircleTuple {
  const typedObj = obj as CircleTuple
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
    typeof typedObj["id"] === "string" &&
    ((typedObj["center"] !== null && typeof typedObj["center"] === "object") || typeof typedObj["center"] === "function") &&
    (isPoint2D(typedObj["center"]["twoD"]) as boolean) &&
    (isPoint3D(typedObj["center"]["threeD"]) as boolean) &&
    typeof typedObj["radius"] === "number"
  )
}

export function isArcTuple(obj: unknown): obj is ArcTuple {
  const typedObj = obj as ArcTuple
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
    typeof typedObj["id"] === "string" &&
    ((typedObj["center"] !== null && typeof typedObj["center"] === "object") || typeof typedObj["center"] === "function") &&
    (isPoint2D(typedObj["center"]["twoD"]) as boolean) &&
    (isPoint3D(typedObj["center"]["threeD"]) as boolean) &&
    ((typedObj["start"] !== null && typeof typedObj["start"] === "object") || typeof typedObj["start"] === "function") &&
    (isPoint2D(typedObj["start"]["twoD"]) as boolean) &&
    (isPoint3D(typedObj["start"]["threeD"]) as boolean) &&
    ((typedObj["end"] !== null && typeof typedObj["end"] === "object") || typeof typedObj["end"] === "function") &&
    (isPoint2D(typedObj["end"]["twoD"]) as boolean) &&
    (isPoint3D(typedObj["end"]["threeD"]) as boolean)
  )
}

export function isFaceTuple(obj: unknown): obj is FaceTuple {
  const typedObj = obj as FaceTuple
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
    typeof typedObj["id"] === "string" &&
    typeof typedObj["face"] === "object"
  )
}

export function isHistoryStep(obj: unknown): obj is HistoryStep {
  const typedObj = obj as HistoryStep
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
    typeof typedObj["name"] === "string" &&
    typeof typedObj["suppressed"] === "boolean" &&
    typeof typedObj["unique_id"] === "string" &&
    ((((typedObj["data"] !== null && typeof typedObj["data"] === "object") || typeof typedObj["data"] === "function") &&
      (isHistoryStepType(typedObj["data"]["type"]) as boolean) &&
      (isPoint3D(typedObj["data"]["point"]) as boolean)) ||
      (((typedObj["data"] !== null && typeof typedObj["data"] === "object") || typeof typedObj["data"] === "function") &&
        (isHistoryStepType(typedObj["data"]["type"]) as boolean) &&
        (isPlane(typedObj["data"]["plane"]) as boolean) &&
        typeof typedObj["data"]["width"] === "number" &&
        typeof typedObj["data"]["height"] === "number") ||
      (((typedObj["data"] !== null && typeof typedObj["data"] === "object") || typeof typedObj["data"] === "function") &&
        (isHistoryStepType(typedObj["data"]["type"]) as boolean) &&
        ((typedObj["data"]["extrusion"] !== null && typeof typedObj["data"]["extrusion"] === "object") ||
          typeof typedObj["data"]["extrusion"] === "function") &&
        typeof typedObj["data"]["extrusion"]["sketch_id"] === "string" &&
        Array.isArray(typedObj["data"]["extrusion"]["face_ids"]) &&
        typedObj["data"]["extrusion"]["face_ids"].every((e: any) => typeof e === "number") &&
        typeof typedObj["data"]["extrusion"]["length"] === "number" &&
        typeof typedObj["data"]["extrusion"]["offset"] === "number" &&
        typeof typedObj["data"]["extrusion"]["direction"] === "string" &&
        typeof typedObj["data"]["extrusion"]["mode"] === "string") ||
      (((typedObj["data"] !== null && typeof typedObj["data"] === "object") || typeof typedObj["data"] === "function") &&
        (isHistoryStepType(typedObj["data"]["type"]) as boolean) &&
        ((typedObj["data"]["plane_description"] !== null && typeof typedObj["data"]["plane_description"] === "object") ||
          typeof typedObj["data"]["plane_description"] === "function") &&
        typeof typedObj["data"]["plane_description"]["PlaneId"] === "string" &&
        typeof typedObj["data"]["width"] === "number" &&
        typeof typedObj["data"]["height"] === "number" &&
        ((typedObj["data"]["sketch"] !== null && typeof typedObj["data"]["sketch"] === "object") || typeof typedObj["data"]["sketch"] === "function") &&
        ((typedObj["data"]["sketch"]["points"] !== null && typeof typedObj["data"]["sketch"]["points"] === "object") ||
          typeof typedObj["data"]["sketch"]["points"] === "function") &&
        Object.entries<any>(typedObj["data"]["sketch"]["points"]).every(([key, value]) => (isPoint2D(value) as boolean) && typeof key === "string") &&
        typeof typedObj["data"]["sketch"]["highest_point_id"] === "number" &&
        ((typedObj["data"]["sketch"]["line_segments"] !== null && typeof typedObj["data"]["sketch"]["line_segments"] === "object") ||
          typeof typedObj["data"]["sketch"]["line_segments"] === "function") &&
        Object.entries<any>(typedObj["data"]["sketch"]["line_segments"]).every(([key, value]) => (isSegmentId(value) as boolean) && typeof key === "string") &&
        typeof typedObj["data"]["sketch"]["highest_line_segment_id"] === "number" &&
        ((typedObj["data"]["sketch"]["circles"] !== null && typeof typedObj["data"]["sketch"]["circles"] === "object") ||
          typeof typedObj["data"]["sketch"]["circles"] === "function") &&
        Object.entries<any>(typedObj["data"]["sketch"]["circles"]).every(([key, value]) => (isCircle(value) as boolean) && typeof key === "string") &&
        typeof typedObj["data"]["sketch"]["highest_circle_id"] === "number" &&
        typeof typedObj["data"]["sketch"]["arcs"] === "object" &&
        typeof typedObj["data"]["sketch"]["highest_arc_id"] === "number" &&
        typeof typedObj["data"]["sketch"]["constraints"] === "object" &&
        typeof typedObj["data"]["sketch"]["highest_constraint_id"] === "number"))
  )
}

export function isHistoryStepType(obj: unknown): obj is HistoryStepType {
  const typedObj = obj as HistoryStepType
  return typedObj === "Point" || typedObj === "Plane" || typedObj === "Extrusion" || typedObj === "Sketch"
}

export function isPointHistoryStep(obj: unknown): obj is PointHistoryStep {
  const typedObj = obj as PointHistoryStep
  return (isHistoryStep(typedObj) as boolean) && (isPointData(typedObj) as boolean)
}

export function isPlaneHistoryStep(obj: unknown): obj is PlaneHistoryStep {
  const typedObj = obj as PlaneHistoryStep
  return (isHistoryStep(typedObj) as boolean) && (isPlaneData(typedObj) as boolean)
}

export function isExtrusionHistoryStep(obj: unknown): obj is ExtrusionHistoryStep {
  const typedObj = obj as ExtrusionHistoryStep
  return (isHistoryStep(typedObj) as boolean) && (isExtrusionData(typedObj) as boolean)
}

export function isSketchHistoryStep(obj: unknown): obj is SketchHistoryStep {
  const typedObj = obj as SketchHistoryStep
  return (isHistoryStep(typedObj) as boolean) && (isSketchData(typedObj) as boolean)
}

export function isPointData(obj: unknown): obj is PointData {
  const typedObj = obj as PointData
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
    ((typedObj["data"] !== null && typeof typedObj["data"] === "object") || typeof typedObj["data"] === "function") &&
    (isHistoryStepType(typedObj["data"]["type"]) as boolean) &&
    (isPoint3D(typedObj["data"]["point"]) as boolean)
  )
}

export function isPlaneData(obj: unknown): obj is PlaneData {
  const typedObj = obj as PlaneData
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
    ((typedObj["data"] !== null && typeof typedObj["data"] === "object") || typeof typedObj["data"] === "function") &&
    (isHistoryStepType(typedObj["data"]["type"]) as boolean) &&
    (isPlane(typedObj["data"]["plane"]) as boolean) &&
    typeof typedObj["data"]["width"] === "number" &&
    typeof typedObj["data"]["height"] === "number"
  )
}

export function isExtrusionData(obj: unknown): obj is ExtrusionData {
  const typedObj = obj as ExtrusionData
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
    ((typedObj["data"] !== null && typeof typedObj["data"] === "object") || typeof typedObj["data"] === "function") &&
    (isHistoryStepType(typedObj["data"]["type"]) as boolean) &&
    ((typedObj["data"]["extrusion"] !== null && typeof typedObj["data"]["extrusion"] === "object") || typeof typedObj["data"]["extrusion"] === "function") &&
    typeof typedObj["data"]["extrusion"]["sketch_id"] === "string" &&
    Array.isArray(typedObj["data"]["extrusion"]["face_ids"]) &&
    typedObj["data"]["extrusion"]["face_ids"].every((e: any) => typeof e === "number") &&
    typeof typedObj["data"]["extrusion"]["length"] === "number" &&
    typeof typedObj["data"]["extrusion"]["offset"] === "number" &&
    typeof typedObj["data"]["extrusion"]["direction"] === "string" &&
    typeof typedObj["data"]["extrusion"]["mode"] === "string"
  )
}

export function isSketchRealized(obj: unknown): obj is SketchRealized {
  const typedObj = obj as SketchRealized
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
    typeof typedObj["plane_id"] === "string" &&
    typeof typedObj["plane_name"] === "string" &&
    ((typedObj["points"] !== null && typeof typedObj["points"] === "object") || typeof typedObj["points"] === "function") &&
    Object.entries<any>(typedObj["points"]).every(([key, value]) => (isPoint3D(value) as boolean) && typeof key === "string") &&
    ((typedObj["points_2d"] !== null && typeof typedObj["points_2d"] === "object") || typeof typedObj["points_2d"] === "function") &&
    Object.entries<any>(typedObj["points_2d"]).every(([key, value]) => (isPoint2D(value) as boolean) && typeof key === "string") &&
    typeof typedObj["highest_point_id"] === "number" &&
    ((typedObj["line_segments"] !== null && typeof typedObj["line_segments"] === "object") || typeof typedObj["line_segments"] === "function") &&
    Object.entries<any>(typedObj["line_segments"]).every(([key, value]) => (isSegmentId(value) as boolean) && typeof key === "string") &&
    typeof typedObj["highest_line_segment_id"] === "number" &&
    ((typedObj["circles"] !== null && typeof typedObj["circles"] === "object") || typeof typedObj["circles"] === "function") &&
    Object.entries<any>(typedObj["circles"]).every(([key, value]) => (isCircle(value) as boolean) && typeof key === "string") &&
    typeof typedObj["highest_circle_id"] === "number" &&
    ((typedObj["arcs"] !== null && typeof typedObj["arcs"] === "object") || typeof typedObj["arcs"] === "function") &&
    Object.entries<any>(typedObj["arcs"]).every(([key, value]) => (isArc(value) as boolean) && typeof key === "string") &&
    typeof typedObj["highest_arc_id"] === "number" &&
    typeof typedObj["constraints"] === "object" &&
    typeof typedObj["highest_constraint_id"] === "number" &&
    Array.isArray(typedObj["faces"])
  )
}

export function isArc(obj: unknown): obj is Arc {
  const typedObj = obj as Arc
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
    typeof typedObj["center"] === "number" &&
    typeof typedObj["start"] === "number" &&
    typeof typedObj["end"] === "number" &&
    typeof typedObj["clockwise"] === "boolean"
  )
}

export function isSketchData(obj: unknown): obj is SketchData {
  const typedObj = obj as SketchData
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
    ((typedObj["data"] !== null && typeof typedObj["data"] === "object") || typeof typedObj["data"] === "function") &&
    (isHistoryStepType(typedObj["data"]["type"]) as boolean) &&
    ((typedObj["data"]["plane_description"] !== null && typeof typedObj["data"]["plane_description"] === "object") ||
      typeof typedObj["data"]["plane_description"] === "function") &&
    typeof typedObj["data"]["plane_description"]["PlaneId"] === "string" &&
    typeof typedObj["data"]["width"] === "number" &&
    typeof typedObj["data"]["height"] === "number" &&
    ((typedObj["data"]["sketch"] !== null && typeof typedObj["data"]["sketch"] === "object") || typeof typedObj["data"]["sketch"] === "function") &&
    ((typedObj["data"]["sketch"]["points"] !== null && typeof typedObj["data"]["sketch"]["points"] === "object") ||
      typeof typedObj["data"]["sketch"]["points"] === "function") &&
    Object.entries<any>(typedObj["data"]["sketch"]["points"]).every(([key, value]) => (isPoint2D(value) as boolean) && typeof key === "string") &&
    typeof typedObj["data"]["sketch"]["highest_point_id"] === "number" &&
    ((typedObj["data"]["sketch"]["line_segments"] !== null && typeof typedObj["data"]["sketch"]["line_segments"] === "object") ||
      typeof typedObj["data"]["sketch"]["line_segments"] === "function") &&
    Object.entries<any>(typedObj["data"]["sketch"]["line_segments"]).every(([key, value]) => (isSegmentId(value) as boolean) && typeof key === "string") &&
    typeof typedObj["data"]["sketch"]["highest_line_segment_id"] === "number" &&
    ((typedObj["data"]["sketch"]["circles"] !== null && typeof typedObj["data"]["sketch"]["circles"] === "object") ||
      typeof typedObj["data"]["sketch"]["circles"] === "function") &&
    Object.entries<any>(typedObj["data"]["sketch"]["circles"]).every(([key, value]) => (isCircle(value) as boolean) && typeof key === "string") &&
    typeof typedObj["data"]["sketch"]["highest_circle_id"] === "number" &&
    typeof typedObj["data"]["sketch"]["arcs"] === "object" &&
    typeof typedObj["data"]["sketch"]["highest_arc_id"] === "number" &&
    typeof typedObj["data"]["sketch"]["constraints"] === "object" &&
    typeof typedObj["data"]["sketch"]["highest_constraint_id"] === "number"
  )
}

export function isSegmentId(obj: unknown): obj is SegmentId {
  const typedObj = obj as SegmentId
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
    typeof typedObj["start"] === "number" &&
    typeof typedObj["end"] === "number"
  )
}

export function isCircle(obj: unknown): obj is Circle {
  const typedObj = obj as Circle
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
    typeof typedObj["center"] === "number" &&
    typeof typedObj["radius"] === "number" &&
    typeof typedObj["top"] === "number"
  )
}

export function isTruckNurbsPoint(obj: unknown): obj is TruckNurbsPoint {
  const typedObj = obj as TruckNurbsPoint
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
    typeof typedObj["x"] === "number" &&
    typeof typedObj["y"] === "number" &&
    typeof typedObj["z"] === "number" &&
    typeof typedObj["w"] === "number"
  )
}

export function isTruckNurbsSurfaceControlPoint(obj: unknown): obj is TruckNurbsSurfaceControlPoint {
  const typedObj = obj as TruckNurbsSurfaceControlPoint
  return Array.isArray(typedObj) && (isTruckNurbsPoint(typedObj[0]) as boolean) && (isTruckNurbsPoint(typedObj[1]) as boolean)
}

export function isTruckNurbsSurfaceControlPoints(obj: unknown): obj is TruckNurbsSurfaceControlPoints {
  const typedObj = obj as TruckNurbsSurfaceControlPoints
  return Array.isArray(typedObj) && typedObj.every((e: any) => isTruckNurbsSurfaceControlPoint(e) as boolean)
}

export function isTruckNurbsSurface(obj: unknown): obj is TruckNurbsSurface {
  const typedObj = obj as TruckNurbsSurface
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
    ((typedObj["NURBSSurface"] !== null && typeof typedObj["NURBSSurface"] === "object") || typeof typedObj["NURBSSurface"] === "function") &&
    Array.isArray(typedObj["NURBSSurface"]["knot_vecs"]) &&
    typedObj["NURBSSurface"]["knot_vecs"].every((e: any) => Array.isArray(e) && e.every((e: any) => typeof e === "number")) &&
    (isTruckNurbsSurfaceControlPoints(typedObj["NURBSSurface"]["control_points"]) as boolean)
  )
}

export function isTruckPlane(obj: unknown): obj is TruckPlane {
  const typedObj = obj as TruckPlane
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
    ((typedObj["Plane"] !== null && typeof typedObj["Plane"] === "object") || typeof typedObj["Plane"] === "function") &&
    ((typedObj["Plane"]["o"] !== null && typeof typedObj["Plane"]["o"] === "object") || typeof typedObj["Plane"]["o"] === "function") &&
    typeof typedObj["Plane"]["o"]["x"] === "number" &&
    typeof typedObj["Plane"]["o"]["y"] === "number" &&
    typeof typedObj["Plane"]["o"]["z"] === "number" &&
    ((typedObj["Plane"]["p"] !== null && typeof typedObj["Plane"]["p"] === "object") || typeof typedObj["Plane"]["p"] === "function") &&
    typeof typedObj["Plane"]["p"]["x"] === "number" &&
    typeof typedObj["Plane"]["p"]["y"] === "number" &&
    typeof typedObj["Plane"]["p"]["z"] === "number" &&
    ((typedObj["Plane"]["q"] !== null && typeof typedObj["Plane"]["q"] === "object") || typeof typedObj["Plane"]["q"] === "function") &&
    typeof typedObj["Plane"]["q"]["x"] === "number" &&
    typeof typedObj["Plane"]["q"]["y"] === "number" &&
    typeof typedObj["Plane"]["q"]["z"] === "number"
  )
}

export function isTruckSurface(obj: unknown): obj is TruckSurface {
  const typedObj = obj as TruckSurface
  return (isTruckNurbsSurface(typedObj) as boolean) || (isTruckPlane(typedObj) as boolean)
}

export function isTruckFaceBoundary(obj: unknown): obj is TruckFaceBoundary {
  const typedObj = obj as TruckFaceBoundary
  return Array.isArray(typedObj) && typedObj.every((e: any) => isTruckFaceEdgeIndex(e) as boolean)
}

export function isTruckFace(obj: unknown): obj is TruckFace {
  const typedObj = obj as TruckFace
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
    Array.isArray(typedObj["boundaries"]) &&
    typedObj["boundaries"].every((e: any) => isTruckFaceBoundary(e) as boolean) &&
    typeof typedObj["orientation"] === "boolean" &&
    (isTruckSurface(typedObj["surface"]) as boolean)
  )
}

export function isTruckNurbsCurve(obj: unknown): obj is TruckNurbsCurve {
  const typedObj = obj as TruckNurbsCurve
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
    ((typedObj["NURBSCurve"] !== null && typeof typedObj["NURBSCurve"] === "object") || typeof typedObj["NURBSCurve"] === "function") &&
    Array.isArray(typedObj["NURBSCurve"]["knot_vec"]) &&
    typedObj["NURBSCurve"]["knot_vec"].every((e: any) => typeof e === "number") &&
    Array.isArray(typedObj["NURBSCurve"]["control_points"]) &&
    typedObj["NURBSCurve"]["control_points"].every((e: any) => isTruckNurbsPoint(e) as boolean)
  )
}

export function isTruckEdgeEndpoints(obj: unknown): obj is TruckEdgeEndpoints {
  const typedObj = obj as TruckEdgeEndpoints
  return Array.isArray(typedObj) && typeof typedObj[0] === "number" && typeof typedObj[1] === "number"
}

export function isTruckEdge(obj: unknown): obj is TruckEdge {
  const typedObj = obj as TruckEdge
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
    (isTruckEdgeEndpoints(typedObj["vertices"]) as boolean) &&
    (isTruckCurve(typedObj["curve"]) as boolean)
  )
}

export function isTruckLineVectors(obj: unknown): obj is TruckLineVectors {
  const typedObj = obj as TruckLineVectors
  return (
    Array.isArray(typedObj) &&
    ((typedObj[0] !== null && typeof typedObj[0] === "object") || typeof typedObj[0] === "function") &&
    typeof typedObj[0]["x"] === "number" &&
    typeof typedObj[0]["y"] === "number" &&
    typeof typedObj[0]["z"] === "number" &&
    ((typedObj[1] !== null && typeof typedObj[1] === "object") || typeof typedObj[1] === "function") &&
    typeof typedObj[1]["x"] === "number" &&
    typeof typedObj[1]["y"] === "number" &&
    typeof typedObj[1]["z"] === "number"
  )
}

export function isTruckLine(obj: unknown): obj is TruckLine {
  const typedObj = obj as TruckLine
  return ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") && (isTruckLineVectors(typedObj["Line"]) as boolean)
}

export function isTruckCurve(obj: unknown): obj is TruckCurve {
  const typedObj = obj as TruckCurve
  return (isTruckNurbsCurve(typedObj) as boolean) || (isTruckLine(typedObj) as boolean)
}

export function isSketchTuple(obj: unknown): obj is SketchTuple {
  const typedObj = obj as SketchTuple
  return Array.isArray(typedObj) && (isSketchRealized(typedObj[0]) as boolean) && (isSketchRealized(typedObj[1]) as boolean) && typeof typedObj[2] === "string"
}

export function isRealization(obj: unknown): obj is Realization {
  const typedObj = obj as Realization
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
    ((typedObj["planes"] !== null && typeof typedObj["planes"] === "object") || typeof typedObj["planes"] === "function") &&
    Object.entries<any>(typedObj["planes"]).every(([key, value]) => (isPlaneRealized(value) as boolean) && typeof key === "string") &&
    ((typedObj["points"] !== null && typeof typedObj["points"] === "object") || typeof typedObj["points"] === "function") &&
    Object.entries<any>(typedObj["points"]).every(([key, value]) => (isPoint3D(value) as boolean) && typeof key === "string") &&
    ((typedObj["sketches"] !== null && typeof typedObj["sketches"] === "object") || typeof typedObj["sketches"] === "function") &&
    Object.entries<any>(typedObj["sketches"]).every(([key, value]) => (isSketchTuple(value) as boolean) && typeof key === "string") &&
    ((typedObj["solids"] !== null && typeof typedObj["solids"] === "object") || typeof typedObj["solids"] === "function") &&
    Object.entries<any>(typedObj["solids"]).every(([key, value]) => (isSolidRealized(value) as boolean) && typeof key === "string")
  )
}

export function isPlaneRealized(obj: unknown): obj is PlaneRealized {
  const typedObj = obj as PlaneRealized
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
    typeof typedObj["name"] === "string" &&
    typeof typedObj["width"] === "number" &&
    typeof typedObj["height"] === "number" &&
    (isPlane(typedObj["plane"]) as boolean)
  )
}

export function isTruckFaceEdgeIndex(obj: unknown): obj is TruckFaceEdgeIndex {
  const typedObj = obj as TruckFaceEdgeIndex
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
    typeof typedObj["index"] === "number" &&
    typeof typedObj["orientation"] === "boolean"
  )
}

export function isTruckBoundary(obj: unknown): obj is TruckBoundary {
  const typedObj = obj as TruckBoundary
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
    Array.isArray(typedObj["vertices"]) &&
    typedObj["vertices"].every(
      (e: any) =>
        ((e !== null && typeof e === "object") || typeof e === "function") &&
        typeof e["x"] === "number" &&
        typeof e["y"] === "number" &&
        typeof e["z"] === "number",
    ) &&
    Array.isArray(typedObj["edges"]) &&
    typedObj["edges"].every((e: any) => isTruckEdge(e) as boolean) &&
    Array.isArray(typedObj["faces"]) &&
    typedObj["faces"].every((e: any) => isTruckFace(e) as boolean)
  )
}

export function isTruckSolid(obj: unknown): obj is TruckSolid {
  const typedObj = obj as TruckSolid
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
    Array.isArray(typedObj["boundaries"]) &&
    typedObj["boundaries"].every((e: any) => isTruckBoundary(e) as boolean)
  )
}

export function isSolidRealized(obj: unknown): obj is SolidRealized {
  const typedObj = obj as SolidRealized
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
    typeof typedObj["name"] === "string" &&
    typeof typedObj["crc32"] === "string" &&
    Array.isArray(typedObj["vertices"]) &&
    typedObj["vertices"].every(
      (e: any) =>
        ((e !== null && typeof e === "object") || typeof e === "function") &&
        typeof e["x"] === "number" &&
        typeof e["y"] === "number" &&
        typeof e["z"] === "number",
    ) &&
    Array.isArray(typedObj["normals"]) &&
    typedObj["normals"].every(
      (e: any) =>
        ((e !== null && typeof e === "object") || typeof e === "function") &&
        typeof e["x"] === "number" &&
        typeof e["y"] === "number" &&
        typeof e["z"] === "number",
    ) &&
    Array.isArray(typedObj["uvs"]) &&
    typedObj["uvs"].every(
      (e: any) =>
        ((e !== null && typeof e === "object") || typeof e === "function") &&
        typeof e["x"] === "number" &&
        typeof e["y"] === "number" &&
        typeof e["z"] === "number",
    ) &&
    Array.isArray(typedObj["indices"]) &&
    typedObj["indices"].every((e: any) => typeof e === "number") &&
    Array.isArray(typedObj["triangles"]) &&
    (isTruckSolid(typedObj["truck_solid"]) as boolean)
  )
}

export function isExtrusionSketchData(obj: unknown): obj is ExtrusionSketchData {
  const typedObj = obj as ExtrusionSketchData
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
    typeof typedObj["sketch_id"] === "string" &&
    Array.isArray(typedObj["face_ids"]) &&
    typedObj["face_ids"].every((e: any) => typeof e === "string") &&
    typeof typedObj["length"] === "string" &&
    typeof typedObj["offset"] === "number" &&
    typeof typedObj["direction"] === "string" &&
    typeof typedObj["mode"] === "string"
  )
}

export function isUpdateExtrusion(obj: unknown): obj is UpdateExtrusion {
  const typedObj = obj as UpdateExtrusion
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
    typeof typedObj["workbench_id"] === "number" &&
    typeof typedObj["sketch_id"] === "string" &&
    Array.isArray(typedObj["face_ids"]) &&
    typedObj["face_ids"].every((e: any) => typeof e === "number") &&
    typeof typedObj["length"] === "number" &&
    typeof typedObj["offset"] === "number" &&
    typeof typedObj["extrusion_name"] === "string" &&
    typeof typedObj["direction"] === "string" &&
    typeof typedObj["extrusion_id"] === "string"
  )
}

export function isSetSketchPlane(obj: unknown): obj is SetSketchPlane {
  const typedObj = obj as SetSketchPlane
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
    typeof typedObj["workbench_id"] === "number" &&
    typeof typedObj["sketch_id"] === "string" &&
    typeof typedObj["plane_id"] === "string"
  )
}

export function isNewSketchOnPlane(obj: unknown): obj is NewSketchOnPlane {
  const typedObj = obj as NewSketchOnPlane
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
    typeof typedObj["workbench_id"] === "number" &&
    typeof typedObj["plane_id"] === "string" &&
    typeof typedObj["sketch_name"] === "string"
  )
}

export function isNewExtrusion(obj: unknown): obj is NewExtrusion {
  const typedObj = obj as NewExtrusion
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
    typeof typedObj["workbench_id"] === "number" &&
    typeof typedObj["sketch_id"] === "string" &&
    Array.isArray(typedObj["face_ids"]) &&
    typedObj["face_ids"].every((e: any) => typeof e === "number") &&
    typeof typedObj["length"] === "number" &&
    typeof typedObj["offset"] === "number" &&
    typeof typedObj["extrusion_name"] === "string" &&
    typeof typedObj["direction"] === "string"
  )
}

export function isDeleteLines(obj: unknown): obj is DeleteLines {
  const typedObj = obj as DeleteLines
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
    typeof typedObj["workbench_id"] === "number" &&
    typeof typedObj["sketch_id"] === "string" &&
    Array.isArray(typedObj["line_ids"]) &&
    typedObj["line_ids"].every((e: any) => typeof e === "number")
  )
}

export function isDeleteArcs(obj: unknown): obj is DeleteArcs {
  const typedObj = obj as DeleteArcs
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
    typeof typedObj["workbench_id"] === "number" &&
    typeof typedObj["sketch_id"] === "string" &&
    Array.isArray(typedObj["arc_ids"]) &&
    typedObj["arc_ids"].every((e: any) => typeof e === "number")
  )
}

export function isDeleteCircles(obj: unknown): obj is DeleteCircles {
  const typedObj = obj as DeleteCircles
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
    typeof typedObj["workbench_id"] === "number" &&
    typeof typedObj["sketch_id"] === "string" &&
    Array.isArray(typedObj["circle_ids"]) &&
    typedObj["circle_ids"].every((e: any) => typeof e === "number")
  )
}

export function isNewRectangleBetweenPoints(obj: unknown): obj is NewRectangleBetweenPoints {
  const typedObj = obj as NewRectangleBetweenPoints
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
    typeof typedObj["workbench_id"] === "number" &&
    typeof typedObj["sketch_id"] === "string" &&
    typeof typedObj["start_id"] === "number" &&
    typeof typedObj["end_id"] === "number"
  )
}

export function isNewCircleBetweenPoints(obj: unknown): obj is NewCircleBetweenPoints {
  const typedObj = obj as NewCircleBetweenPoints
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
    typeof typedObj["workbench_id"] === "number" &&
    typeof typedObj["sketch_id"] === "string" &&
    typeof typedObj["center_id"] === "number" &&
    typeof typedObj["edge_id"] === "number"
  )
}

export function isNewLineOnSketch(obj: unknown): obj is NewLineOnSketch {
  const typedObj = obj as NewLineOnSketch
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
    typeof typedObj["workbench_id"] === "number" &&
    typeof typedObj["sketch_id"] === "string" &&
    typeof typedObj["start_point_id"] === "number" &&
    typeof typedObj["end_point_id"] === "number"
  )
}

export function isNewPointOnSketch2(obj: unknown): obj is NewPointOnSketch2 {
  const typedObj = obj as NewPointOnSketch2
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
    typeof typedObj["workbench_id"] === "number" &&
    typeof typedObj["sketch_id"] === "string" &&
    typeof typedObj["x"] === "number" &&
    typeof typedObj["y"] === "number" &&
    typeof typedObj["hidden"] === "boolean"
  )
}

export function isRenameStep(obj: unknown): obj is RenameStep {
  const typedObj = obj as RenameStep
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
    typeof typedObj["workbench_id"] === "number" &&
    typeof typedObj["step_id"] === "number" &&
    typeof typedObj["new_name"] === "string"
  )
}

export function isRenameWorkbench(obj: unknown): obj is RenameWorkbench {
  const typedObj = obj as RenameWorkbench
  return (
    ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
    typeof typedObj["workbench_id"] === "number" &&
    typeof typedObj["new_name"] === "string"
  )
}

export function isRenameProject(obj: unknown): obj is RenameProject {
  const typedObj = obj as RenameProject
  return ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") && typeof typedObj["new_name"] === "string"
}

export function isMessage(obj: unknown): obj is Message {
  const typedObj = obj as Message
  return (
    (((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") && (isUpdateExtrusion(typedObj["UpdateExtrusion"]) as boolean)) ||
    (((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") && (isSetSketchPlane(typedObj["SetSketchPlane"]) as boolean)) ||
    (((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
      (isNewSketchOnPlane(typedObj["NewSketchOnPlane"]) as boolean)) ||
    (((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") && (isNewExtrusion(typedObj["NewExtrusion"]) as boolean)) ||
    (((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") && (isDeleteLines(typedObj["DeleteLines"]) as boolean)) ||
    (((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") && (isDeleteArcs(typedObj["DeleteArcs"]) as boolean)) ||
    (((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") && (isDeleteCircles(typedObj["DeleteCircles"]) as boolean)) ||
    (((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
      (isNewRectangleBetweenPoints(typedObj["NewRectangleBetweenPoints"]) as boolean)) ||
    (((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
      (isNewCircleBetweenPoints(typedObj["NewCircleBetweenPoints"]) as boolean)) ||
    (((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") && (isNewLineOnSketch(typedObj["NewLineOnSketch"]) as boolean)) ||
    (((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") &&
      (isNewPointOnSketch2(typedObj["NewPointOnSketch2"]) as boolean)) ||
    (((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") && (isRenameStep(typedObj["RenameStep"]) as boolean)) ||
    (((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") && (isRenameWorkbench(typedObj["RenameWorkbench"]) as boolean)) ||
    (((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") && (isRenameProject(typedObj["RenameProject"]) as boolean))
  )
}

export function isMessageHistory(obj: unknown): obj is MessageHistory {
  const typedObj = obj as MessageHistory
  return ((typedObj !== null && typeof typedObj === "object") || typeof typedObj === "function") && (isMessage(typedObj["message"]) as boolean)
}

import type Point3D from "./routes/threlte/Point3D.svelte"
import type { Vector2, Vector3, Vector2Like, Vector3Like } from "three"

interface IDictionary<TValue> {
  [id: string]: TValue
}

type WithTarget<Event, Target> = Event & { currentTarget: Target }

type SetCameraFocus = (goTo: Vector3Like, lookAt: Vector3Like, up: Vector3Like) => void

type EntityType =
  | "circle"
  | "arc"
  | "face"
  | "line"
  | "plane"
  | "point"
  | "point3D"
  | "meshFace"

interface Entity {
  id: number
  type: EntityType
}

interface Project {
  name: string
  assemblies: []
  workbenches: WorkBench[]
}

interface WorkBench {
  name: string
  history: HistoryStep[]
  step_counters: {
    Extrusion: number, Plane: number, Point: number, Sketch: number
  }
}



interface PreviewGeometry {
  type: EntityType
  start?: PointLikeById
  end?: PointLikeById
  center?: PointLikeById
  radius?: number
  x?: number
  y?: number
  uuid: string
}

interface Plane {
  origin: Vector3Hideable
  primary: Vector3Like
  secondary: Vector3Like
  tertiary: Vector3Like
}

interface Point {
  twoD: Vector2
  threeD: Vector3
}

interface PointById {
  twoD: PointWithDelta
  threeD: Vector3Hideable
  pointId: string // todo is number string - maybe change to number?
}

interface PointLikeById {
  [x: string]: any // hack todo fix
  twoD?: Vector2Like | PointWithDelta
  threeD?: Vector3Like | Vector3Hideable
  pointId?: string | null // todo is number string - maybe change to number?
}

type PointsById = IDictionary<PointById>
type PointsLikeById = IDictionary<PointLikeById>

interface SnapEntity {
  id: string
  type: EntityType
  x?: number
  y?: number
  z?: number
}

type ProjectToPlane = (point3D: Vector3) => Vector2

interface Vector3Hideable {
  x: number
  y: number
  z: number
  hidden: boolean
}

interface PointWithDelta {
  x: number
  y: number
  m: number
  dx: number
  dy: number
  fx: number
  fy: number
  fixed: boolean
  hidden: boolean
}


interface HistoryStep {
  name: string
  suppressed: boolean
  unique_id: string
  data: PointData["data"] | PlaneData["data"] | ExtrusionData["data"] | SketchData["data"]
}

type HistoryStepType =
  | "Point"
  | "Plane"
  | "Extrusion"
  | "Sketch"

type PointHistoryStep = HistoryStep & PointData
type PlaneHistoryStep = HistoryStep & PlaneData
type ExtrusionHistoryStep = HistoryStep & ExtrusionData
type SketchHistoryStep = HistoryStep & SketchData

interface PointData {
  data: {
    type: HistoryStepType = "Point"
    point: Vector3Hideable
  }
}

interface PlaneData {
  data: {
    type: HistoryStepType = "Plane"
    plane: Plane
    width: number  // %
    height: number // %
  }
}

interface ExtrusionData {
  data: {
    type: HistoryStepType = "Extrusion"
    extrusion: {
      sketch_id: string
      face_ids: number[]
      length: number
      offset: number
      direction: string // e.g "Normal"  todo enums
      mode: string // e,g "New"  todo enums
    }
  }
}

interface SketchRealized {
  plane_id: string
  plane_name: string
  points: IDictionary<Vector3Hideable>
  points_2d: IDictionary<PointWithDelta>
  highest_point_id: number
  line_segments: IDictionary<SegmentId>
  highest_line_segment_id: number
  circles: IDictionary<Circle>
  highest_circle_id: number
  arcs: object // todo
  highest_arc_id: number
  constraints: object // todo
  highest_constraint_id: number
  faces: array // todo
}

interface SketchData {
  data: {
    type: HistoryStepType = "Sketch"
    plane_description: {
      PlaneId: string
    }
    width: number
    height: number
    sketch: {
      points: IDictionary<PointWithDelta>
      highest_point_id: number
      line_segments: IDictionary<SegmentId>
      highest_line_segment_id: number
      circles: IDictionary<Circle>
      highest_circle_id: number
      arcs: object // todo
      highest_arc_id: number
      constraints: object // todo
      highest_constraint_id: number
    }
  }
}

interface SegmentId {
  start: number
  end: number
}

interface Circle {
  center: number
  radius: number
  top: number
}

interface TruckFaceEdgeIndex {
  index: number
  orientation: boolean
}

type TruckFaceBoundary = TruckFaceEdgeIndex[]
type TruckFaceBoundaries = TruckFaceBoundary[]

interface TruckNurbsPoint {
  x: number
  y: number
  z: number
  w: number
}

type TruckNurbsSurfaceControlPoint = [TruckNurbsPoint, TruckNurbsPoint] // [start, end] ?
type TruckNurbsSurfaceControlPoints = TruckNurbsSurfaceControlPoint[]

interface TruckNurbsSurface {
  NURBSSurface: {
    knot_vecs: number[][]
    // knot_vecs: [                              // todo type strongly
    //   [0, 0, 0, 0.25, 0.5, 0.75, 1, 1, 1],    // Vector3Like[] ?
    //   [0, 0, 1, 1]                            // similar to NurbsPoint? [x,y,z,w] ?
    // ],
    control_points: TruckNurbsSurfaceControlPoints
  }
}

interface TruckPlane {
  Plane: {
    o: Vector3Like
    p: Vector3Like
    q: Vector3Like
  }
}

type TruckSurface = TruckNurbsSurface | TruckPlane

interface TruckFace {
  boundaries: TruckFaceBoundaries
  orientation: boolean
  surface: TruckSurface
}

interface TruckNurbsCurve {
  NURBSCurve: {
    knot_vec: number[]
    // "knot_vec": [0, 0, 0, 0.25, 0.5, 0.75, 1, 1, 1], // todo type strongly
    control_points: TruckNurbsPoint[]
  }
}

type TruckEdgeEndpoints = [number, number] // [startIndex, EndIndex] ?

interface TruckEdge {
  vertices: TruckEdgeEndpoints
  curve: TruckCurve
}

type TruckLineVectors = [Vector3Like, Vector3Like] // [{y,y,z}, {y,y,z}] // [start, end]

interface TruckLine {
  Line: TruckLineVectors
}

type TruckCurve = TruckNurbsCurve | TruckLine

interface Realization {
  planes: IDictionary<PlaneRealized>
  points: IDictionary<Vector3Hideable>
  sketches: IDictionary<[SketchRealized, SketchRealized, string]>
  solids: IDictionary<SolidRealized>
}

interface PlaneRealized {
  name: string
  width: number
  height: number
  plane: Plane
}

interface TruckFaceEdgeIndex {
  index: number
  orientation: boolean
}

interface TruckBoundary {
  vertices: Vector3Like[]
  edges: TruckEdge[]
  faces: TruckFace[]
}

interface TruckSolid {
  boundaries: TruckBoundary[]
}
interface SolidRealized {
  name: string
  crc32: string
  vertices: Vector3Like[]
  normals: Vector3Like[]
  uvs: Vector3Like[]
  indices: number[]
  triangles: array // todo
  truck_solid: TruckSolid
}


interface ExtrusionSketchData {
  sketch_id: string
  face_ids: string[]
  length: string // todo change to number
  offset: number
  direction: string
  mode: string
}

// rust expects:
// `RenameWorkbench`
// `RenameStep`
// `RenameProject`
// `DeleteLines`
// `DeleteArcs`
// `DeleteCircles`
// `NewPointOnSketch`
// `NewPointOnSketch2`
// `NewCircleBetweenPoints`
// `NewRectangleBetweenPoints`
// `NewLineOnSketch`
// `DeleteLineSegment`
// `StepSketch`
// `SolveSketch`
// `NewSketchOnPlane`
// `SetSketchPlane`
// `DeleteSketch`
// `NewExtrusion`
// `UpdateExtrusion`
// `UpdateExtrusionLength`

// todo these interfaces should be exported from rust with wasm bindgen ?

// ========= io messages sent to rust =========
interface UpdateExtrusion {
  workbench_id: number
  sketch_id: string
  face_ids: number[]
  length: number
  offset: 0.0
  extrusion_name: "Extra"
  direction: "Normal"
  extrusion_id: number
}

interface SetSketchPlane {
  workbench_id: number
  sketch_id: string
  plane_id: string
}

interface NewSketchOnPlane {
  workbench_id: number
  plane_id: string
  sketch_name: string
}

interface NewExtrusion {
  workbench_id: number
  sketch_id: string
  face_ids: number[]
  length: number
  offset: number
  extrusion_name: string
  direction: string
}

interface DeleteLines {
  workbench_id: number
  sketch_id: string
  line_ids: number[]
}

interface DeleteArcs {
  workbench_id: number
  sketch_id: string
  arc_ids: number[]
}

interface DeleteCircles {
  workbench_id: number
  sketch_id: string
  circle_ids: number[]
}

interface NewRectangleBetweenPoints {
  workbench_id: number
  sketch_id: string
  start_id: number
  end_id: number
}

interface NewCircleBetweenPoints {
  workbench_id: number
  sketch_id: string
  center_id: number
  edge_id: number
}

interface NewLineOnSketch {
  workbench_id: number
  sketch_id: string
  start_point_id: number
  end_point_id: number
}

interface NewPointOnSketch2 {
  workbench_id: number
  sketch_id: string
  x: Vector2["x"]
  y: Vector2["y"]
  hidden: boolean
}

interface RenameStep {
  workbench_id: number
  step_id: number
  new_name: string
}

type Message =
  | { UpdateExtrusion: UpdateExtrusion }
  | { SetSketchPlane: SetSketchPlane }
  | { NewSketchOnPlane: NewSketchOnPlane }
  | { NewExtrusion: NewExtrusion }
  | { DeleteLines: DeleteLines }
  | { DeleteArcs: DeleteArcs }
  | { DeleteCircles: DeleteCircles }
  | { NewRectangleBetweenPoints: NewRectangleBetweenPoints }
  | { NewCircleBetweenPoints: NewCircleBetweenPoints }
  | { NewLineOnSketch: NewLineOnSketch }
  | { NewPointOnSketch2: NewPointOnSketch2 }
  | { RenameStep: RenameStep }

interface MessageHistory {
  message: Message
  result: any
}

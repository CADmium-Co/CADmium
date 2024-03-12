declare module "nurbs"

export type WithTarget<Event, Target> = Event & { currentTarget: Target }

export type EntityType =
  "circle" |
  "arc" |
  "face" |
  "line" |
  "plane" |
  "point" |
  "point3D" |
  "meshFace"

export interface MousedOver {
  type: EntityType, id: string
}

const example = {
  "name": "First Project",
  "assemblies": [],
  "workbenches": [
    {
      "name": "Workbench 1",
      "history": [
        {
          "name": "Origin",
          "unique_id": "Point-0",
          "suppressed": false,
          "data": {
            "type": "Point",
            "point": {
              "x": 0,
              "y": 0,
              "z": 0,
              "hidden": false
            }
          }
        },
        {
          "name": "Top",
          "unique_id": "Plane-0",
          "suppressed": false,
          "data": {
            "type": "Plane",
            "plane": {
              "origin": {
                "x": 0,
                "y": 0,
                "z": 0,
                "hidden": false
              },
              "primary": {
                "x": 1,
                "y": 0,
                "z": 0
              },
              "secondary": {
                "x": 0,
                "y": 1,
                "z": 0
              },
              "tertiary": {
                "x": 0,
                "y": 0,
                "z": 1
              }
            },
            "width": 100,
            "height": 100
          }
        },
        {
          "name": "Front",
          "unique_id": "Plane-1",
          "suppressed": false,
          "data": {
            "type": "Plane",
            "plane": {
              "origin": {
                "x": 0,
                "y": 0,
                "z": 0,
                "hidden": false
              },
              "primary": {
                "x": 1,
                "y": 0,
                "z": 0
              },
              "secondary": {
                "x": 0,
                "y": 0,
                "z": 1
              },
              "tertiary": {
                "x": 0,
                "y": -1,
                "z": 0
              }
            },
            "width": 100,
            "height": 100
          }
        },
        {
          "name": "Right",
          "unique_id": "Plane-2",
          "suppressed": false,
          "data": {
            "type": "Plane",
            "plane": {
              "origin": {
                "x": 0,
                "y": 0,
                "z": 0,
                "hidden": false
              },
              "primary": {
                "x": 0,
                "y": 1,
                "z": 0
              },
              "secondary": {
                "x": 0,
                "y": 0,
                "z": 1
              },
              "tertiary": {
                "x": 1,
                "y": 0,
                "z": 0
              }
            },
            "width": 100,
            "height": 100
          }
        }
      ],
      "step_counters": {
        "Extrusion": 0,
        "Sketch": 0,
        "Point": 1,
        "Plane": 3
      }
    }
  ]
}

export interface Project {
  name: string
  assemblies: []
  workbenches: WorkBench[]
}

export interface WorkBench {
  name: string
  history: HistoryStep[]
  step_counters: {
    Extrusion: number, Plane: number, Point: number, Sketch: number
  }
}

export interface HistoryStep {
  name: string
  suppressed: boolean
  unique_id: string
  data: any // todo tighten up
}

export interface Entity {
  id: string
  type: EntityType
}

export interface Realization {
  planes: object
  points: object
  sketches: object
  solids: object
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

// io messages sent to rust
interface UpdateExtrusion {
  workbench_id: number
  sketch_id: string
  face_ids: number[]
  length: number
  offset: 0.0
  extrusion_name: "Extra"
  direction: "Normal"
  extrusion_id: string
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

export type Message =
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

export interface MessageHistory {
  message: Message
  result: any
}

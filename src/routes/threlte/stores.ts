import { Project as WasmProject, Realization as WasmRealization } from 'cadmium'
import { writable } from 'svelte/store'
import type { WorkBench, MessageHistory, Project, Realization, Entity, EntityType, SnapEntity, PointLikeById, PreviewGeometry } from "../../types"

// @ts-ignore
export const wasmProject = writable<WasmProject>({})
export const project = writable<Project>(emptyProject())
export const projectIsStale = writable(false)

export const workbenchIndex = writable(0)
export const workbench = writable<WorkBench>(emptyWorkBench())
export const workbenchIsStale = writable(false)

export const featureIndex = writable<number>(1000)
export const extrusionFeatures = writable<Entity[]>([])
export const wasmRealization = writable<WasmRealization>()
export const realization = writable<Realization>(emptyRealization())
export const realizationIsStale = writable(false)

export const hiddenSketches = writable<string[]>([])
export const sketchBeingEdited = writable("")
export const sketchTool = writable("")

// could be looking for 'face' or 'plane' or other things
export const selectingFor = writable<EntityType[]>([])
export const selectionMax = writable(1000)
export const selectionMin = writable(0)

export const currentlyMousedOver = writable<SnapEntity[]>([])
export const currentlySelected = writable<Entity[]>([])
export const snapPoints = writable<PointLikeById[]>([])
export const previewGeometry = writable<PreviewGeometry[]>([])

export const messageHistory = writable<MessageHistory[]>([])

function emptyWorkBench(): WorkBench {
  return {
    name: "",
    history: [],
    step_counters: {
      Extrusion: 0,
      Plane: 0,
      Point: 0,
      Sketch: 0,
    }
  }
}
function emptyProject(): Project {
  return {
    name: "",
    assemblies: [],
    workbenches: [],
  }
}
function emptyRealization(): Realization {
  return {
    planes: {},
    points: {},
    sketches: {},
    solids: {},
  }
}



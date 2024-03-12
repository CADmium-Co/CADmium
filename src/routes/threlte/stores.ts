import { Project as WasmProject, Realization as WasmRealization } from 'cadmium'
import { writable } from 'svelte/store'
import type { WorkBench, MessageHistory, Project, Realization } from "../../types"
import type { MousedOver } from "../../types"

// @ts-ignore
export const wasmProject = writable<WasmProject>({})
export const project = writable<Project>(emptyProject())
export const projectIsStale = writable(false)

export const workbenchIndex = writable(0)
export const wasmWorkbench = writable()
export const workbench = writable<WorkBench>(emptyBench())
export const workbenchIsStale = writable(false)

export const featureIndex = writable(1000)
export const wasmRealization = writable<WasmRealization>()
export const realization = writable<Realization>(emptyRealization())
export const realizationIsStale = writable(false)

export const hiddenSketches = writable([])
export const sketchBeingEdited = writable(null)
export const sketchTool = writable(null)

// could be looking for 'face' or 'plane' or other things
export const selectingFor = writable([])
export const selectionMax = writable(1000)
export const selectionMin = writable(0)

export const currentlyMousedOver = writable<MousedOver[]>([])
export const currentlySelected = writable([])
export const snapPoints = writable([])
export const previewGeometry = writable([])

export const messageHistory = writable<MessageHistory[]>([])

function emptyBench(): WorkBench {
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



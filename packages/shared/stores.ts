import {Solid, Project as WasmProject, Workbench} from "cadmium"
import {writable} from "svelte/store"
import type {MessageHistory, Project, Entity, EntityType, SnapEntity, PointLikeById, PreviewGeometry} from "./types"
import {isArcEntity, isCircleEntity, isEntity, isFaceEntity, isLineEntity, isMeshFaceEntity, isPlaneEntity, isPoint3DEntity, isPointEntity} from "./typeGuards"

// prettier-ignore
const log = (function () { const context = "[stores.ts]"; const color = "hotpink"; return Function.prototype.bind.call(console.log, console, `%c${context}`, `font-weight:bold;color:${color};`) })()

export const wasmProject = writable<WasmProject>({} as WasmProject)
export const project = writable<Project>(emptyProject())
export const projectIsStale = writable(false)

export const workbenchIndex = writable(0)
export const workbench = writable<Workbench>(emptyWorkBench())
export const workbenchSolids = writable<Solid[]>([])
export const workbenchIsStale = writable(false)

export const featureIndex = writable<number>(1000)
export const extrusionFeatures = writable<Entity[]>([])

export const hiddenSketches = writable<number[]>([])
export const sketchBeingEdited = writable(-1)
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

project.subscribe(store => log("[project]", store))
workbenchIndex.subscribe(store => log("[workbenchIndex]", store))
workbench.subscribe(store => log("[workbench]", store))
workbenchIsStale.subscribe(store => log("[workbenchIsStale]", store))
featureIndex.subscribe(store => log("[featureIndex]", store))
extrusionFeatures.subscribe(store => log("[extrusionFeatures]", store))
sketchBeingEdited.subscribe(store => log("[sketchBeingEdited]", store))
messageHistory.subscribe(store => log("[messageHistory]", store))

currentlySelected.subscribe(store => {
  log("[currentlySelected]", store)
  const allValid = store.every(entity => isEntity(entity))
  const error = "[stores.ts] [currentlySelected] has invalid entities"
  if (!allValid) {
    console.error(error, store)
    // throw new Error(error)
  }

  const types = ["circle", "arc", "face", "line", "plane", "point", "point3D", "meshFace"] as EntityType[]
  types.forEach(type => {
    const [isType, entity] = latestIsEntity(store, type)
    if (isType) log(`[currentlySelected] entity is ${type === "arc" ? "an" : "a"} ${type}:`, entity)
  })
})

function latestIsEntity(store: Entity[], type: EntityType) {
  if (store.length === 0) return [false, null]
  const entity = store[store.length - 1]
  switch (type) {
    case "circle":
      return [isCircleEntity(entity), entity]
    case "arc":
      return [isArcEntity(entity), entity]
    case "face":
      return [isFaceEntity(entity), entity]
    case "line":
      return [isLineEntity(entity), entity]
    case "plane":
      return [isPlaneEntity(entity), entity]
    case "point":
      return [isPointEntity(entity), entity]
    case "point3D":
      return [isPoint3DEntity(entity), entity]
    case "meshFace":
      return [isMeshFaceEntity(entity), entity]
    default:
      break
  }
  log("[latestIsEntity] has incorrect switch statement implemented")
  return [false, null]
}

function emptyWorkBench(): Workbench {
  return {
    name: "",
    history: [],
    points: [],
    points_next_id: 0,
    sketches: [],
    sketches_next_id: 0,
    planes: [],
    planes_next_id: 0,
    features: [],
    features_next_id: 0,
  }
}
function emptyProject(): Project {
  return {
    name: "",
    assemblies: [],
    workbenches: [],
  }
}

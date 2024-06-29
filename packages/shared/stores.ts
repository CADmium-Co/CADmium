import {EvTree, Project, StepHash, Workbench, get_project} from "cadmium"
import {derived, writable} from "svelte/store"
import type {MessageHistory, Entity, EntityType, SnapEntity, PointLikeById, PreviewGeometry, Point2WithID} from "./types"
import {isArcEntity, isCircleEntity, isEntity, isFaceEntity, isLineEntity, isMeshFaceEntity, isPlaneEntity, isPoint3DEntity, isPointEntity} from "./typeGuards"

// prettier-ignore
const log = (function () { const context = "[stores.ts]"; const color = "hotpink"; return Function.prototype.bind.call(console.log, console, `%c${context}`, `font-weight:bold;color:${color};`) })()

export const projectIndex = writable(0)
export const project = derived<typeof projectIndex, Project>(projectIndex, $projectIndex => { get_project($projectIndex) })

export const workbenchIndex = writable(0)
export const workbench = derived<[typeof project, typeof workbenchIndex], Workbench>([project, workbenchIndex], ([$project, $workbenchIndex]) => { $project.workbenches[$workbenchIndex] })

export const featureIndex = writable<number>(1000)
export const extrusionFeatures = writable<Entity[]>([])

export const hiddenSketches = writable<StepHash[]>([])
export const sketchBeingEdited = writable<StepHash | null>(null)
export const sketchTool = writable("")

// could be looking for 'face' or 'plane' or other things
export const selectingFor = writable<EntityType[]>([])
export const selectionMax = writable(1000)
export const selectionMin = writable(0)

export const currentlyMousedOver = writable<SnapEntity[]>([])
export const currentlySelected = writable<Entity[]>([])
export const snapPoints = writable<Point2WithID[]>([])
export const previewGeometry = writable<PreviewGeometry[]>([])

export const messageHistory = writable<MessageHistory[]>([])

project.subscribe(store => log("[project]", store))
workbench.subscribe(store => log("[workbench]", store))
featureIndex.subscribe(store => log("[featureIndex]", store))
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
    evtree: {} as EvTree,
  }
}
function emptyProject(): Project {
  return {
    name: "",
    assemblies: [],
    workbenches: [],
  }
}

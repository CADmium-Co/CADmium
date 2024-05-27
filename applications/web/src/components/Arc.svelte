<script lang="ts">
  import {LineGeometry} from "three/addons/lines/LineGeometry.js"
  import {LineMaterial} from "three/addons/lines/LineMaterial.js"
  import {Vector2} from "three"
  import {T} from "@threlte/core"
  import {flatten, arcToPoints, promoteTo3} from "shared/projectUtils"
  import {currentlySelected, currentlyMousedOver, sketchTool} from "shared/stores"
  import type {EntityType, SketchPoint} from "shared/types"
  import {isEntity} from "shared/typeGuards"

  const log = (function () { const context = "[Arc.svelte]"; const color="gray"; return Function.prototype.bind.call(console.log, console, `%c${context}`, `font-weight:bold;color:${color};`)})() // prettier-ignore

  const type: EntityType = "arc"

  export let id: string, center: SketchPoint, start: SketchPoint, end: SketchPoint

  export let dashedLineMaterial: LineMaterial,
    dashedHoveredMaterial: LineMaterial,
    solidLineMaterial: LineMaterial,
    solidHoveredMaterial: LineMaterial,
    solidSelectedMaterial: LineMaterial,
    collisionLineMaterial: LineMaterial

  let hovered = false

  $: selected = $currentlySelected.some(e => isEntity(e) && e.id === id && e.type === type) ? true : false

  const center2 = new Vector2(center.twoD.x, center.twoD.y)
  const start2 = new Vector2(start.twoD.x, start.twoD.y)
  const end2 = new Vector2(end.twoD.x, end.twoD.y)

  const points = flatten(promoteTo3(arcToPoints(center2, start2, end2 /** implicit false */))) // defaulted false in function todo ask Matt

  const lineGeometry = new LineGeometry()
  lineGeometry.setPositions(points)
</script>

<T.Group>
  <T.Line2
    geometry={lineGeometry}
    material={hovered ? dashedHoveredMaterial : dashedLineMaterial}
    on:create={({ref}) => {
      ref.computeLineDistances()
    }}
  />
  <T.Line2
    geometry={lineGeometry}
    material={hovered ? solidHoveredMaterial : selected ? solidSelectedMaterial : solidLineMaterial}
    on:create={({ref}) => {
      ref.computeLineDistances()
    }}
  />
  <T.Line2
    geometry={lineGeometry}
    material={collisionLineMaterial}
    on:create={({ref}) => {
      ref.computeLineDistances()
    }}
    on:pointerover={() => {
      if ($sketchTool === "select") {
        hovered = true
        $currentlyMousedOver = [...$currentlyMousedOver, {type, id}]
      }
    }}
    on:pointerout={() => {
      if ($sketchTool === "select") {
        hovered = false
        $currentlyMousedOver = $currentlyMousedOver.filter(item => !(item.id === id && item.type === type))
      }
    }}
  />
</T.Group>

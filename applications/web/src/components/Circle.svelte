<script lang="ts">
  import {LineGeometry} from "three/addons/lines/LineGeometry.js"
  import {LineMaterial} from "three/addons/lines/LineMaterial.js"
  import {T} from "@threlte/core"
  import {flatten, circleToPoints, promoteTo3} from "shared/projectUtils"
  import {currentlySelected, currentlyMousedOver, sketchTool} from "shared/stores"
  import type {CircleTuple, EntityType} from "shared/types"

  const log = (function () { const context = "[Circle.svelte]"; const color="gray"; return Function.prototype.bind.call(console.log, console, `%c${context}`, `font-weight:bold;color:${color};`)})() // prettier-ignore

  const type: EntityType = "circle"

  export let id: string, center: CircleTuple["center"], radius: number

  // log("[props]", "id:", id, "center:", center, "radius:", radius)

  export let dashedLineMaterial: LineMaterial,
    dashedHoveredMaterial: LineMaterial,
    solidLineMaterial: LineMaterial,
    solidHoveredMaterial: LineMaterial,
    solidSelectedMaterial: LineMaterial,
    collisionLineMaterial: LineMaterial

  let hovered = false
  $: selected = $currentlySelected.some(e => e.id === id && e.type === type) ? true : false

  // array of x,y,z points
  const points = flatten(promoteTo3(circleToPoints(center.twoD, radius)))

  const lineGeometry = new LineGeometry()
  lineGeometry.setPositions(points)
</script>

<T.Group>
  <T.Line2 geometry={lineGeometry} material={hovered ? dashedHoveredMaterial : dashedLineMaterial} on:create={({ref}) => ref.computeLineDistances()} />
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
        // log("$currentlyMousedOver", $currentlyMousedOver)
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

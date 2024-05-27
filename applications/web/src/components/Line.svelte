<script lang="ts">
  import {LineGeometry} from "three/addons/lines/LineGeometry.js"
  import type {LineMaterial} from "three/examples/jsm/lines/LineMaterial.js"
  import {Vector2} from "three"
  import {T} from "@threlte/core"
  import {flatten, promoteTo3} from "shared/projectUtils"
  import {currentlySelected, currentlyMousedOver, sketchTool} from "shared/stores"
  import type {EntityType, PointById} from "shared/types"
  import {isEntity} from "shared/typeGuards"

  const log = (function () { const context = "[Line.svelte]"; const color="gray"; return Function.prototype.bind.call(console.log, console, `%c${context}`, `font-weight:bold;color:${color};`)})() // prettier-ignore

  export let id: string, start: PointById, end: PointById
  // log("[props]", "id:", id, "start:", start, "end:", end)

  export let dashedLineMaterial: LineMaterial,
    dashedHoveredMaterial: LineMaterial,
    solidLineMaterial: LineMaterial,
    solidHoveredMaterial: LineMaterial,
    solidSelectedMaterial: LineMaterial,
    collisionLineMaterial: LineMaterial

  const type: EntityType = "line"

  let hovered = false
  $: selected = $currentlySelected.some(e => checkIsEntity(e) && e.id === id && e.type === type) ? true : false

  function checkIsEntity(e: unknown) {
    // log("[checkIsEntity]", isEntity(e), e)
    return isEntity(e)
  }

  // $: selected, log("[selected] an entity has been selected:", selected, "Line.id:", id)
  // prettier-ignore
  // $: $currentlySelected, ()=> {if (selected && !$currentlySelected.every((e) => isEntity(e))) log("ERROR [currentlySelected] are not all isEntity", $currentlySelected)}

  const points = flatten(
		promoteTo3([new Vector2(start.twoD.x, start.twoD.y), new Vector2(end.twoD.x, end.twoD.y)])
	)

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
        $currentlyMousedOver = [...$currentlyMousedOver, {type, id: id}]
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

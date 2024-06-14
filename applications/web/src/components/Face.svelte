<script lang="ts">
  import {T} from "@threlte/core"
  import {Path, Vector2, Shape, MeshStandardMaterial, DoubleSide, ShapeGeometry} from "three"
  import {circleToPoints, arcToPoints} from "shared/projectUtils"
  import {currentlySelected, currentlyMousedOver, selectingFor} from "shared/stores"
  import type {EntityType, IDictionary, Point2WithID} from "shared/types"

  // @ts-ignore
  const log = (function () { const context = "[Face.svelte]"; const color="gray"; return Function.prototype.bind.call(console.log, console, `%c${context}`, `font-weight:bold;color:${color};`)})() // prettier-ignore

  // todo see docs below
  // interface Face {
  // 	 exterior: wire
  //   holes: wires[]
  // }
  export let face: any, id: string, pointsById: IDictionary<Point2WithID>

  const type: EntityType = "face"

  let hovered = false
  $: selected = $currentlySelected.some(e => e.id === id && e.type === type) ? true : false

  // a face has an exterior and holes.
  // exterior is a wire, and holes is an array of wires.
  // a wire contains either .Segments or .Circle
  // If a wire has .Segments, it is an array of segments
  // each segment is an object with a field called 'type'
  // if 'type' is 'Line' then there is also .start and .end which are point IDs
  // if 'type' is 'Arc' then there is also .center, .start, and .end which are point IDs and .clockwise which is boolean
  // If a wire has .Circle is an object with:
  // .center which is a point ID, .radius which is a float, and .top which is a point ID
  // holes is an array of wires

  // todo type wire properly
  function writeWireToShape(wire: {Circle: any; Segments: any}, shape: Path) {
    if (wire.Circle) {
      let circle = wire.Circle
      let center = pointsById[circle.center]
      let radius = circle.radius
      let points = circleToPoints(new Vector2(center.x, center.y), radius)
      shape.setFromPoints(points)
    } else if (wire.Segments) {
      let points = []
      for (let segment of wire.Segments) {
        console.log("segment", segment)
        if ("Line" in segment) {
          let start = segment.Line.start
          let end = segment.Line.end

          if (points.length === 0) {
            points.push(new Vector2(start.data[0], start.data[1]))
          }
          points.push(new Vector2(end.data[0], end.data[1]))
        } else if ("Arc" in segment) {
          // TODO: We use a center + angles, not 3 points
          let center = segment.Arc.center
          let start = pointsById[segment.Arc.start]
          let end = pointsById[segment.Arc.end]

          let arcPoints = arcToPoints(
            new Vector2(center.x, center.y),
            new Vector2(start.x, start.y),
            new Vector2(end.x, end.y),
            segment.Arc.clockwise,
          )

          if (points.length !== 0) {
            arcPoints.shift()
          }
          points.push(...arcPoints)
        }
      }
      if (points.length > 0) {
        shape.setFromPoints(points)
      }
    }
  }

  const shape = new Shape()
  let exterior = face.exterior
  writeWireToShape(exterior, shape)

  for (let interior of face.holes) {
    const path = new Path()
    writeWireToShape(interior, path)
    shape.holes.push(path)
  }

  const geometry = new ShapeGeometry(shape)
  // const edges = new EdgesGeometry(geometry, 15)
  // const edgeMaterial = new LineBasicMaterial({ color: 0xff0000 })

  const standardMaterial = new MeshStandardMaterial({
    color: "#525252",
    side: DoubleSide,
    metalness: 0.0,
    transparent: true,
    opacity: 0.1,
    depthWrite: false,
    depthTest: true,
    wireframe: false,
    polygonOffset: true,
    polygonOffsetFactor: -4,
  })

  const hoverMaterial = new MeshStandardMaterial({
    color: "#525252",
    side: DoubleSide,
    metalness: 0.0,
    transparent: true,
    opacity: 0.25,
    depthWrite: false,
    depthTest: true,
    wireframe: false,
    polygonOffset: true,
    polygonOffsetFactor: -4,
  })

  const selectedMaterial = new MeshStandardMaterial({
    color: "#525252",
    side: DoubleSide,
    metalness: 0.0,
    transparent: true,
    opacity: 0.4,
    depthWrite: false,
    depthTest: true,
    wireframe: false,
    polygonOffset: true,
    polygonOffsetFactor: -4,
  })
</script>

<T.Group>
  <T.Mesh
    {geometry}
    material={selected ? selectedMaterial : hovered ? hoverMaterial : standardMaterial}
    on:pointerenter={() => {
      if ($selectingFor.includes(type)) {
        hovered = true
        $currentlyMousedOver = [...$currentlyMousedOver, {type, id}]
      }
    }}
    on:pointerleave={() => {
      if ($selectingFor.includes(type)) {
        hovered = false
        $currentlyMousedOver = $currentlyMousedOver.filter(item => !(item.id === id && item.type === type))
      }
    }}
    on:click={() => {
      if ($selectingFor.includes(type)) {
        if ($currentlySelected.some(e => e.id === id && e.type === type)) {
          // this face was already selected, so unselect it
          $currentlySelected = $currentlySelected.filter(item => !(item.id === id && item.type === type))
        } else {
          // @ts-ignore todo make all numeric ids number type.
          $currentlySelected = [...$currentlySelected, {type, id}]
        }
      }
    }}
  />
  <!-- <T.LineSegments geometry={edges} material={edgeMaterial} /> -->
</T.Group>

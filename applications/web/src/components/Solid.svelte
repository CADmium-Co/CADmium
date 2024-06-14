<script lang="ts">
  import * as THREE from "three"
  import {T} from "@threlte/core"
  import SelectableSurface from "./SelectableSurface.svelte"
  import type {LineMaterial} from "three/examples/jsm/lines/LineMaterial.js"
  import type {TruckBoundary, TruckEdge, TruckFace, TruckSolid} from "shared/types"
  import type {Vector3Like} from "three"

  const log = (function () { const context = "[Solid.svelte]"; const color="gray"; return Function.prototype.bind.call(console.log, console, `%c${context}`, `font-weight:bold;color:${color};`)})() // prettier-ignore

  export let name: string, indices, vertices, normals, truckSolid: TruckSolid

  export let dashedLineMaterial: LineMaterial,
    dashedHoveredMaterial: LineMaterial,
    solidLineMaterial: LineMaterial,
    solidHoveredMaterial: LineMaterial,
    solidSelectedMaterial: LineMaterial,
    collisionLineMaterial: LineMaterial

  let truck_vertices: TruckBoundary["vertices"], truck_edges: TruckEdge[], truck_faces: TruckFace[]

  $: {
    const boundaries = truckSolid.boundaries[0]
    truck_vertices = boundaries.vertices
    truck_edges = boundaries.edges
    truck_faces = boundaries.faces
    // log("truckSolid.boundaries[0]", "boundaries:", boundaries)
  }

  const geometry = new THREE.BufferGeometry()

  const normalsArray = new Float32Array(normals.flatMap((v: Vector3Like) => [v.x, v.y, v.z]))
  const verticesArray = new Float32Array(vertices.flatMap((v: Vector3Like) => [v.x, v.y, v.z]))

  // log("Vertices: ", vertices.length)

  geometry.setIndex(indices)
  geometry.setAttribute("position", new THREE.Float32BufferAttribute(verticesArray, 3))
  geometry.setAttribute("normal", new THREE.Float32BufferAttribute(normalsArray, 3))

  const material = new THREE.MeshStandardMaterial({
    color: "#999999",
    side: THREE.DoubleSide,
    wireframe: false,
    metalness: 1.0,
    roughness: 0.6,
  })

  const edges = new THREE.EdgesGeometry(geometry, 15)
  const mat = new THREE.LineBasicMaterial({color: 0x000000})
</script>

<T.Group>
  <T.Mesh {geometry} {material} />
  <T.LineSegments geometry={edges} material={mat} />

  {#each truck_faces as truck_face, i (i)}
    <SelectableSurface
      id={i.toString()}
      {truck_face}
      {truck_vertices}
      {truck_edges}
      {solidLineMaterial}
      {solidHoveredMaterial}
      {solidSelectedMaterial}
      {dashedHoveredMaterial}
      {dashedLineMaterial}
      {collisionLineMaterial}
    />
  {/each}
</T.Group>

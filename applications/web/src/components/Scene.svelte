<script lang="ts">
  import {T, useThrelte} from "@threlte/core"
  import {Environment} from "@threlte/extras"
  import {Vector2, Vector3, type Vector3Like} from "three"
  import {interactivity} from "@threlte/extras"
  import {LineMaterial} from "three/addons/lines/LineMaterial.js"
  import {realization, workbench, sketchBeingEdited} from "shared/stores"
  import Point3D from "./Point3D.svelte"
  import Plane from "./Plane.svelte"
  import Solid from "./Solid.svelte"
  import Sketch from "./Sketch.svelte"
  import CubeGizmo from "./controls/CubeGizmo/CubeGizmo.svelte"
  import {base} from "../base"
  import CadControls from "./controls/CadControls/CadControls.svelte"

  const log = (function () { const context = "[Scene.svelte]"; const color="gray"; return Function.prototype.bind.call(console.log, console, `%c${context}`, `font-weight:bold;color:${color};`)})() // prettier-ignore

  interactivity()

  const {size, dpr, camera} = useThrelte()

  $: points = $realization.points ? Object.entries($realization.points) : []
  $: planes = $realization.planes ? Object.entries($realization.planes) : []
  $: planesById = planes ? Object.fromEntries(planes) : {}
  $: solids = $realization.solids ? Object.entries($realization.solids) : []
  $: sketches = $realization.sketches ? Object.entries($realization.sketches) : []

  // $: $workbench, log("[$workbench]", $workbench)
  // $: points, log("[realization.points]", points)
  // $: planes, log("[realization.planes]", planes)
  // $: planesById, log("[planesById]", planesById)
  // $: solids, log("[realization.solids]", solids)
  // $: sketches, log("[realization.sketches]", sketches)

  // put it on window for debugging. todo remove
  if (!(globalThis as any).realization) (globalThis as any).realization = []
  $: $realization, (() => ((globalThis as any).realization = [...(globalThis as any).realization, $realization]))()

  export function setCameraFocus(goTo: Vector3Like, lookAt: Vector3Like, up: Vector3Like): void {
    // TODO: make this tween nicely
    const positionMultiple = 1000
    const vector = new Vector3(goTo.x, goTo.y, goTo.z)
    vector.multiplyScalar(positionMultiple)
    const look = new Vector3(lookAt.x, lookAt.y, lookAt.z)
    const lookup = new Vector3(up.x, up.y, up.z)
    camera.current.position.set(vector.x, vector.y, vector.z)
    camera.current.lookAt(look.x, look.y, look.z)
    camera.current.up = lookup
  }

  $: dashedLineMaterial = new LineMaterial({
    color: "#000000",
    linewidth: 1.0 * $dpr,
    depthTest: false,
    transparent: true,
    dashed: true,
    dashSize: 2,
    gapSize: 2,
    dashScale: 1,
    resolution: new Vector2($size.width * $dpr, $size.height * $dpr),
  })

  $: dashedHoveredMaterial = new LineMaterial({
    color: "#ffaa00",
    linewidth: 1.0 * $dpr,
    depthTest: false,
    transparent: true,
    dashed: true,
    dashSize: 2,
    gapSize: 2,
    dashScale: 1,
    resolution: new Vector2($size.width * $dpr, $size.height * $dpr),
  })

  $: solidLineMaterial = new LineMaterial({
    color: "#000000",
    linewidth: 1.5 * $dpr,
    depthTest: true,
    transparent: true,
    dashed: false,
    resolution: new Vector2($size.width * $dpr, $size.height * $dpr),
  })

  $: solidHoveredMaterial = new LineMaterial({
    color: "#88aa00",
    linewidth: 5.5 * $dpr,
    depthTest: true,
    transparent: true,
    dashed: false,
    resolution: new Vector2($size.width * $dpr, $size.height * $dpr),
  })

  $: solidSelectedMaterial = new LineMaterial({
    color: "#ffaa00",
    linewidth: 5.5 * $dpr,
    depthTest: true,
    transparent: true,
    dashed: false,
    resolution: new Vector2($size.width * $dpr, $size.height * $dpr),
  })

  $: collisionLineMaterial = new LineMaterial({
    color: "#FFFFFF",
    linewidth: 12.0 * $dpr,
    depthTest: false,
    depthWrite: false,
    transparent: true,
    opacity: 0,
    dashed: false,
    resolution: new Vector2($size.width * $dpr, $size.height * $dpr),
  })

  // mouseButtons={{ LEFT: 0, MIDDLE: 1, RIGHT: 2 }} 0 // standard
  // mouseButtons={{ LEFT: 0, MIDDLE: 2, RIGHT: 1 }} 1 // no
  // mouseButtons={{ LEFT: 1, MIDDLE: 0, RIGHT: 2 }} 2 // close!
  // mouseButtons={{ LEFT: 1, MIDDLE: 2, RIGHT: 0 }} 3 // close?
  // mouseButtons={{ LEFT: 2, MIDDLE: 1, RIGHT: 0 }} 5 // no
  // mouseButtons={{ LEFT: 2, MIDDLE: 0, RIGHT: 1 }} 4 // seems to meet most people's expectations
  // mouseButtons={{ LEFT: 2, MIDDLE: 50, RIGHT: 1 }} 4 // disable left click entirely--free it up for interaction

  // camera position: [160.8, -250.8, 200.55] looks good and angular
</script>

<T.OrthographicCamera makeDefault position={[160.8, -250.8, 200.55]} zoom={5} up={[0, 0, 1]}>
  <CadControls rotateSpeed={1.8} panSpeed={0.5} on:create={({ref}) => {}} mouseButtons={{LEFT: 2, MIDDLE: 50, RIGHT: 1}} />
</T.OrthographicCamera>

<!-- <T.DirectionalLight args={['#ff8888', 50.0]} position.x={-10} position.y={0} position.z={0} />
<T.DirectionalLight args={['#88ff88', 50.0]} position.x={10} position.y={0} position.z={0} />
<T.DirectionalLight args={['#8888ff', 50.0]} position.x={0} position.y={0} position.z={10} /> -->
<!-- <T.DirectionalLight position.x={10} position.y={0} position.z={0} /> -->

<!-- <T.PointLight args={['#ffffff', 5000.0]} position.x={3} position.y={3} position.z={15} />
<T.PointLight args={['#ffffff', 3000.0]} position.x={3} position.y={-3} position.z={-15} />
<T.PointLight args={['#ffffff', 3000.0]} position.x={10} position.y={-13} position.z={1.1} />
<T.PointLight args={['#ffffff', 3000.0]} position.x={-10.5} position.y={11} position.z={0.86} /> -->

<!-- <T.AmbientLight intensity={0.6} /> -->

<Environment path="{base}/envmap/hdr/" files="kloofendal_28d_misty_puresky_1k.hdr" isBackground={false} format="hdr" />

{#each points as [pointName, point] (`${$workbench.name}-${pointName}`)}
  <Point3D id={pointName} x={point.x} y={point.y} z={point.z} hidden={point.hidden} {collisionLineMaterial} />
{/each}

{#each planes as [planeName, plane] (`${$workbench.name}-${planeName}`)}
  <Plane
    name={plane.name}
    id={planeName}
    height={plane.height}
    width={plane.width}
    origin={plane.plane.origin}
    primary={plane.plane.primary}
    secondary={plane.plane.secondary}
    tertiary={plane.plane.tertiary}
  />
{/each}

{#each sketches as [sketchId, sketchTuple] (`${$workbench.name}-${sketchId}`)}
  <Sketch
    uniqueId={sketchId}
    name={sketchTuple[2]}
    {sketchTuple}
    editing={$sketchBeingEdited === sketchId}
    plane={planesById[sketchTuple[0].plane_id]}
    {solidLineMaterial}
    {solidHoveredMaterial}
    {solidSelectedMaterial}
    {dashedHoveredMaterial}
    {dashedLineMaterial}
    {collisionLineMaterial}
  />
{/each}

{#each solids as [solidName, solid] (`${$workbench.name}-${solidName}-${solid.crc32}`)}
  <Solid
    name={solidName}
    indices={solid.indices}
    vertices={solid.vertices}
    normals={solid.normals}
    truckSolid={solid.truck_solid}
    {solidLineMaterial}
    {solidHoveredMaterial}
    {solidSelectedMaterial}
    {dashedHoveredMaterial}
    {dashedLineMaterial}
    {collisionLineMaterial}
  />
{/each}

<CubeGizmo verticalPlacement={"top"} size={140} paddingX={20} paddingY={20} {setCameraFocus} />

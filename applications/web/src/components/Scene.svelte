<script lang="ts">
  import {T, useThrelte} from "@threlte/core"
  import {Environment} from "@threlte/extras"
  import {Vector2, Vector3, type Vector3Like} from "three"
  import {interactivity} from "@threlte/extras"
  import {LineMaterial} from "three/addons/lines/LineMaterial.js"
  import {workbench, sketchBeingEdited} from "shared/stores"
  import Point3D from "./Point3D.svelte"
  import Plane from "./Plane.svelte"
  import Solid from "./Solid.svelte"
  import Sketch from "./Sketch.svelte"
  import CubeGizmo from "./controls/CubeGizmo/CubeGizmo.svelte"
  import {base} from "../base"
  import CadControls from "./controls/CadControls/CadControls.svelte"
  import {isPlaneStep, isPointStep, isSketchStep, isSolidStep} from "shared/stepTypeGuards"

  // @ts-ignore
  const log = (function () { const context = "[Scene.svelte]"; const color="gray"; return Function.prototype.bind.call(console.log, console, `%c${context}`, `font-weight:bold;color:${color};`)})() // prettier-ignore

  interactivity()

  const {size, dpr, camera} = useThrelte()

  $: history = $workbench.history ?? []

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
  <CadControls rotateSpeed={1.8} panSpeed={0.5} mouseButtons={{LEFT: 2, MIDDLE: 50, RIGHT: 1}} />
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
{#each history as step}
  {#if isPointStep(step)}
    <Point3D
      id={`${step.hash}`}
      x={step.result.Point.x}
      y={step.result.Point.y}
      z={step.result.Point.z}
      hidden={step.result.Point.hidden}
      {collisionLineMaterial}
    />
  {:else if isPlaneStep(step)}
    <Plane
      name={step.name}
      id={`${step.hash}`}
      height={100}
      width={100}
      origin={step.result.Plane.origin}
      primary={step.result.Plane.primary}
      secondary={step.result.Plane.secondary}
      tertiary={step.result.Plane.tertiary}
    />
  {:else if isSketchStep(step)}
    <Sketch
      uniqueId={`${step.hash}`}
      name={step.name}
      sketch={step.result.Sketch.sketch}
      editing={$sketchBeingEdited === step.hash}
      {solidLineMaterial}
      {solidHoveredMaterial}
      {solidSelectedMaterial}
      {dashedHoveredMaterial}
      {dashedLineMaterial}
      {collisionLineMaterial}
    />
  {:else if isSolidStep(step)}
    {#each step.result.Solid as solid}
      <Solid
        name={step.name}
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
  {/if}
{/each}

<CubeGizmo verticalPlacement={"top"} size={140} paddingX={20} paddingY={20} {setCameraFocus} />

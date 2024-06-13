<script lang="ts">
  import {Matrix4, Euler, MeshStandardMaterial, Vector2, Vector3} from "three"
  import {T, useThrelte} from "@threlte/core"
  import {Text, Suspense} from "@threlte/extras"
  import {hiddenSketches, previewGeometry, sketchTool, workbench} from "shared/stores"
  import Point2D from "./Point2D.svelte"
  import Line from "./Line.svelte"
  import Circle from "./Circle.svelte"
  import Arc from "./Arc.svelte"
  import Face from "./Face.svelte"
  import {LineMaterial} from "three/addons/lines/LineMaterial.js"
  import {LineGeometry} from "three/addons/lines/LineGeometry.js"
  import NewLineTool from "./tools/NewLine.svelte"
  import NewCircleTool from "./tools/NewCircle.svelte"
  import NewRectangleTool from "./tools/NewRectangle.svelte"
  import SelectTool from "./tools/Select.svelte"
  import type {FaceTuple, IDictionary, PreviewGeometry, SketchPoint} from "shared/types"
  import debounce from "just-debounce-it"
  import type {ISketch, Plane} from "cadmium"
  import {isSketchArcStep, isSketchCircleStep, isSketchLineStep, isSketchPointStep} from "shared/stepTypeGuards"

  // @ts-ignore
  const log = (function () { const context = "[PassiveSketch.svelte]"; const color="gray"; return Function.prototype.bind.call(console.log, console, `%c${context}`, `font-weight:bold;color:${color};`)})() // prettier-ignore

  export let name: string,
    sketch: ISketch,
    plane: Plane,
    uniqueId: string,
    editing = false

  const {size, dpr} = useThrelte()

  $: history = $workbench.history ?? []

  export let dashedLineMaterial: LineMaterial,
    dashedHoveredMaterial: LineMaterial,
    solidLineMaterial: LineMaterial,
    solidHoveredMaterial: LineMaterial,
    solidSelectedMaterial: LineMaterial,
    collisionLineMaterial: LineMaterial

  let newLineTool: NewLineTool, newCircleTool: NewCircleTool, newRectangleTool: NewRectangleTool, selectTool: SelectTool

  let faceTuples: FaceTuple[] = []
  let pointsById: IDictionary<SketchPoint> = {}

  // Build some Three.js vectors from the props
  const origin_point = new Vector3(plane.origin.x, plane.origin.y, plane.origin.z)
  const primary = new Vector3(plane.primary.x, plane.primary.y, plane.primary.z)
  const secondary = new Vector3(plane.secondary.x, plane.secondary.y, plane.secondary.z)
  const tertiary = new Vector3(plane.tertiary.x, plane.tertiary.y, plane.tertiary.z)

  // Use those to make the rotation matrix and euler angles
  const rotationMatrix = new Matrix4()
  rotationMatrix.makeBasis(primary, secondary, tertiary)
  const eulerAngles = new Euler(0, 0, 0, "XYZ")
  eulerAngles.setFromRotationMatrix(rotationMatrix, "XYZ")

  // Lastly, make the Plane Material
  const planeMaterial = new MeshStandardMaterial({
    color: "#525292",
    metalness: 0.0,
    transparent: true,
    opacity: 0.0,
    depthWrite: false,
    depthTest: false,
    wireframe: false,
    polygonOffset: true,
    polygonOffsetFactor: -4,
  })

  const width = 200.0
  const height = 150.0

  // this is x, y, z for each of five points, making a closed square
  const points = [-width / 2, -height / 2, 0, width / 2, -height / 2, 0, width / 2, height / 2, 0, -width / 2, height / 2, 0, -width / 2, -height / 2, 0]

  $: boundaryMaterial = new LineMaterial({
    color: "#42a7eb",
    linewidth: 1.0 * $dpr,
    depthTest: true,
    transparent: true,
    dashed: false,
    resolution: new Vector2($size.width * $dpr, $size.height * $dpr),
  })

  const lineGeometry = new LineGeometry()
  lineGeometry.setPositions(points)

  $: hidden = $hiddenSketches.includes(uniqueId) && !editing
  // $: $hiddenSketches, log("[$hiddenSketches]", $hiddenSketches)

  $: if (editing) $sketchTool = "select"

  function projectToPlane(point3D: Vector3): Vector2 {
    const xComponent = point3D.clone().sub(plane.origin).dot(primary)
    const yComponent = point3D.clone().sub(plane.origin).dot(secondary)
    return new Vector2(xComponent, yComponent)
  }

  function isGeomType(geom: PreviewGeometry, type: string) {
    return geom.type === type
  }
</script>

{#if !hidden}
  <T.Group
    rotation.x={eulerAngles.x}
    rotation.y={eulerAngles.y}
    rotation.z={eulerAngles.z}
    position.x={origin_point.x}
    position.y={origin_point.y}
    position.z={origin_point.z}
  >
    <T.Mesh
      material={planeMaterial}
      on:click={e => {
        if (editing) {
          if ($sketchTool === "line") {
            newLineTool.click(e, {twoD: projectToPlane(e.point), threeD: e.point})
          } else if ($sketchTool === "circle") {
            newCircleTool.click(e, {twoD: projectToPlane(e.point), threeD: e.point})
          } else if ($sketchTool === "rectangle") {
            newRectangleTool.click(e, {twoD: projectToPlane(e.point), threeD: e.point})
          } else if ($sketchTool === "select") {
            selectTool.click(e, projectToPlane(e.point))
          }
        }
      }}
      on:pointermove={debounce((e: any) => {
        if (editing) {
          if ($sketchTool === "line") {
            newLineTool.mouseMove(e, projectToPlane(e.point))
          } else if ($sketchTool === "circle") {
            newCircleTool.mouseMove(e, projectToPlane(e.point))
          } else if ($sketchTool === "rectangle") {
            newRectangleTool.mouseMove(e, projectToPlane(e.point))
          }
        }
      }, 10)}
    >
      <T.PlaneGeometry args={[width * 100, height * 100]} />
    </T.Mesh>

    <SelectTool bind:this={selectTool} sketchIndex={uniqueId} active={$sketchTool === "select"} />
    <NewLineTool bind:this={newLineTool} {pointsById} sketchIndex={uniqueId} active={$sketchTool === "line"} {projectToPlane} />
    <NewCircleTool bind:this={newCircleTool} {pointsById} sketchId={uniqueId} active={$sketchTool === "circle"} {projectToPlane} />
    <NewRectangleTool bind:this={newRectangleTool} {pointsById} sketchIndex={uniqueId} active={$sketchTool === "rectangle"} {projectToPlane} />

    <T.Line2
      geometry={lineGeometry}
      material={boundaryMaterial}
      on:create={({ref}) => {
        ref.computeLineDistances()
      }}
    />

    <T.Group position.x={(-width / 2) * 0.99} position.y={(height / 2) * 0.99}>
      <Suspense>
        <Text text={name} color="#42a7eb" fontSize={5} anchorX="0%" anchorY="0%" />
      </Suspense>
    </T.Group>

    {#each history as step}
      {#if isSketchPointStep(step)}
        <Point2D x={step.result.Point2.x} y={step.result.Point2.y} hidden={step.result.Point2.hidden} id={step.hash} {collisionLineMaterial} />
      {:else if isSketchCircleStep(step)}
        <Circle
          center={pointsById[step.result.Circle2.center]}
          radius={step.result.Circle2.radius}
          id={step.hash}
          {solidLineMaterial}
          {solidHoveredMaterial}
          {solidSelectedMaterial}
          {dashedHoveredMaterial}
          {dashedLineMaterial}
          {collisionLineMaterial}
        />
      {:else if isSketchArcStep(step)}
        <!-- TODO: Use start & end angle instead of points -->
        <Arc
          center={pointsById[step.result.Arc2.center]}
          start={pointsById[0]}
          end={pointsById[1]}
          id={step.hash}
          {solidLineMaterial}
          {solidHoveredMaterial}
          {solidSelectedMaterial}
          {dashedHoveredMaterial}
          {dashedLineMaterial}
          {collisionLineMaterial}
        />
      {:else if isSketchLineStep(step)}
        <Line
          start={pointsById[step.result.Line2.start]}
          end={pointsById[step.result.Line2.end]}
          id={step.hash}
          {solidLineMaterial}
          {solidHoveredMaterial}
          {solidSelectedMaterial}
          {dashedHoveredMaterial}
          {dashedLineMaterial}
          {collisionLineMaterial}
        />
      {/if}
    {/each}

    {#each $previewGeometry as geom (geom.uuid)}
      {#if isGeomType(geom, "line")}
        <Line
          start={geom.start}
          end={geom.end}
          id={geom.uuid}
          {solidLineMaterial}
          {solidHoveredMaterial}
          {solidSelectedMaterial}
          {dashedHoveredMaterial}
          {dashedLineMaterial}
          {collisionLineMaterial}
        />
      {:else if isGeomType(geom, "circle")}
        <Circle
          center={geom.center}
          radius={geom.radius}
          id={geom.uuid}
          {solidLineMaterial}
          {solidHoveredMaterial}
          {solidSelectedMaterial}
          {dashedHoveredMaterial}
          {dashedLineMaterial}
          {collisionLineMaterial}
        />
      {:else if isGeomType(geom, "point")}
        <Point2D x={geom.x} y={geom.y} hidden={false} id={geom.uuid} isPreview {collisionLineMaterial} />
      {/if}
    {/each}

    {#each faceTuples as face (`${faceTuples.length}-${face.id}`)}
      <Face face={face.face} id={face.id} {pointsById} />
    {/each}
  </T.Group>
{/if}

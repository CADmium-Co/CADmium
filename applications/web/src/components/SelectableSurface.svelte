<script lang="ts">
  import {LineGeometry} from "three/addons/lines/LineGeometry.js"
  import type {LineMaterial} from "three/examples/jsm/lines/LineMaterial.js"
  import {Shape, ShapeGeometry, Vector3, Vector2, Path, MeshStandardMaterial, DoubleSide, Euler, Matrix4, type Vector3Like} from "three"
  import {T} from "@threlte/core"
  import {flatten} from "shared/projectUtils"
  import {currentlySelected, currentlyMousedOver, selectingFor, selectionMax, selectionMin} from "shared/stores"
  import type {EntityType, TruckEdge, TruckFace, TruckFaceBoundary} from "shared/types"
  import nurbs from "nurbs"

  const log = (function () { const context = "[SelectableSurface.svelte]"; const color="gray"; return Function.prototype.bind.call(console.log, console, `%c${context}`, `font-weight:bold;color:${color};`)})() // prettier-ignore

  export let truck_face: TruckFace, truck_edges: TruckEdge[], id: string
  // log("[props]", "truck_face:", truck_face, "truck_edges:", truck_edges, "id:", id)

  // svelte-ignore unused-export-let hmmm why does it not ignore?
  export let dashedLineMaterial: LineMaterial,
    dashedHoveredMaterial: LineMaterial,
    solidLineMaterial: LineMaterial,
    solidHoveredMaterial: LineMaterial,
    solidSelectedMaterial: LineMaterial,
    collisionLineMaterial: LineMaterial,
    truck_vertices

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

  const hoveredMaterial = new MeshStandardMaterial({
    color: "#ff0000",
    side: DoubleSide,
    metalness: 0.0,
    transparent: true,
    opacity: 0.5,
    depthWrite: false,
    depthTest: true,
    wireframe: false,
    polygonOffset: true,
    polygonOffsetFactor: -4,
  })

  const surface = truck_face.surface
  const interiors: LineGeometry[] = []
  const eulerAngles: Euler = new Euler(0, 0, 0, "XYZ")
  let plane
  let exterior: LineGeometry
  let origin = new Vector3(0, 0, 0)

  const shape = new Shape()

  if ("Plane" in surface) {
    // cool, this surface is planar. let's extract its boundaries
    // boundaries is an array like [0, 1] where the indices point to the truck_edges array

    plane = surface.Plane
    const o = new Vector3(plane.o.x, plane.o.y, plane.o.z)
    origin = o
    const p = new Vector3(plane.p.x, plane.p.y, plane.p.z)
    const q = new Vector3(plane.q.x, plane.q.y, plane.q.z)
    const u = p.clone().sub(o).normalize()
    const v = q.clone().sub(o).normalize()

    // Build some Three.js vectors from the props
    const primary = u
    const secondary = v
    const tertiary = u.clone().cross(v)

    // Use those to make the rotation matrix and euler angles
    const rotationMatrix = new Matrix4()
    rotationMatrix.makeBasis(primary, secondary, tertiary)
    // eulerAngles = new Euler(0, 0, 0, "XYZ")
    eulerAngles.setFromRotationMatrix(rotationMatrix, "XYZ")

    const boundaries = truck_face.boundaries
    const exterior_bounds = boundaries[0]
    // log('Boundaries: ', boundaries)
    const points = curveToPoints(exterior_bounds)
    exterior = new LineGeometry()
    exterior.setPositions(points)

    const projectedPoints = project(points, u, v, o)
    shape.setFromPoints(projectedPoints)

    // log('Projected points', projectedPoints)

    // shape.setFromPoints(points)

    /*
		shape lives in 2D and needs points which look like {x: 3 y: 2.3}

		So I need to extract the origin, x and y axis that define the Plane that
		this surface lives on, then project each xyz point onto that 2d plane
		then save each of those as the new points, then shape.setFromPoints(those_points)

		THEN I need to understand the rotation and translation required to go from the TOP plane
		to this new plane, and apply that rotation to a T.Group object which contains the face

		Q: what do I do when the face is not planar? does a 2d nurbs surface provide me with a way
		to make triangles easily?
		Q: Can I completely replace the mesh visualization with this b-rep visualization? That would
		solve my issue with huge numbers of triangles AND my inefficient triangle encoding
		Q: how do I pick how many points to make for a NURBS curve? Is it sufficient to
		assume it is a circular arc and do the radius thing, with some minimum so even small holes
		look good? Or does it provide a single NURBS curve for where the circle hits the straight lines?
		If so, can I determine from the knot vector where I need to sample densely?
		*/

    boundaries.slice(1).forEach(element => {
      let points = curveToPoints(element)
      let ring = new LineGeometry()
      ring.setPositions(points)
      interiors.push(ring)
      // log("[interiors]", interiors)

      let projectedPoints = project(points, u, v, o)
      const path = new Path()
      path.setFromPoints(projectedPoints)
      shape.holes.push(path)
    })
  }

  const geometry = new ShapeGeometry(shape)

  function project(points: number[], u: Vector3Like, v: Vector3Like, o: Vector3Like) {
    // log("[project]", "Points to project:", "points:", points, "u:", u, "v:", v, "o:", o)
    const vectors = []
    for (let i = 0; i < points.length; i += 3) {
      const point3D = new Vector3(points[i], points[i + 1], points[i + 2])
      point3D.x = point3D.x - o.x
      point3D.y = point3D.y - o.y
      point3D.z = point3D.z - o.z
      const xComponent = point3D.dot(u)
      const yComponent = point3D.dot(v)
      vectors.push(new Vector2(xComponent, yComponent))
    }
    return vectors
  }

  function curveToPoints(exterior: TruckFaceBoundary) {
    const points = []
    for (let {index, orientation} of exterior) {
      // log('grabbing edge: ', index, orientation)
      const edge = truck_edges[index]
      const curve = edge.curve

      // https://github.com/MattFerraro/CADmium/pull/2#discussion_r1536905388
      // There are two common representations for NURBS control points, one where x, y, z, w can be used as is,
      // and one where you need to normalize by w first. The NURBS engine that is included in truck uses one kind and the
      // NURBS javascript library that we're using here uses the other kind, so it is necessary to normalize by w here.
      if ("NURBSCurve" in curve) {
        const {NURBSCurve} = curve
        const weights = NURBSCurve.control_points.map(point => point.w)
        const controlPoints = NURBSCurve.control_points.map(point => [point.x / point.w, point.y / point.w, point.z / point.w])

        const nurbsCurve = nurbs({
          points: controlPoints,
          weights: weights,
          knots: NURBSCurve.knot_vec,
          degree: 2,
        })

        const domain = nurbsCurve.domain[0]
        // log('Spline Dimension:', curve.splineDimension)
        // log('Dimension:', curve.dimension)

        // todo find out how the nurbs library works. meantime ignore its wizardry
        // @ts-ignore
        const a = []
        const b = []
        for (let t = domain[0]; t <= domain[1]; t += 0.02) {
          // @ts-ignore
          nurbsCurve.evaluate(a, t)
          // @ts-ignore
          b.push(new Vector3(a[0], a[1], a[2]))
        }

        const flattened = flatten(b)
        for (let p of flattened) points.push(p)
      } else if ("Line" in curve) {
        const line = curve.Line
        const startPoint = orientation === true ? line[0] : line[1]
        const endPoint = orientation === true ? line[1] : line[0]

        points.push(startPoint.x)
        points.push(startPoint.y)
        points.push(startPoint.z)

        points.push(endPoint.x)
        points.push(endPoint.y)
        points.push(endPoint.z)
      }
    }

    points.push(points[0])
    points.push(points[1])
    points.push(points[2])

    return points
  }

  let hovered = false
  let selected = false
  $: selected = $currentlySelected.some(e => e.id === id && e.type === type) ? true : false

  const type: EntityType = "meshFace"
</script>

<T.Group>
  {#if exterior}
    <T.Line2
      geometry={exterior}
      material={solidLineMaterial}
      on:create={({ref}) => {
        ref.computeLineDistances()
      }}
    />

    {#each interiors as interior}
      <T.Line2
        geometry={interior}
        material={solidLineMaterial}
        on:create={({ref}) => {
          ref.computeLineDistances()
        }}
      />
    {/each}

    <T.Group rotation.x={eulerAngles.x} rotation.y={eulerAngles.y} rotation.z={eulerAngles.z} position.x={origin.x} position.y={origin.y} position.z={origin.z}>
      <T.Mesh
        {geometry}
        material={hovered ? hoveredMaterial : standardMaterial}
        on:pointerenter={e => {
          if ($selectingFor.includes(type)) {
            // log("On Pointer Enter and includes type")
            e.stopPropagation()
            hovered = true
            $currentlyMousedOver = [...$currentlyMousedOver, {type, id}]
          }
        }}
        on:pointerleave={() => {
          // log("On Pointer Leave!"")
          if ($selectingFor.includes(type)) {
            hovered = false
            $currentlyMousedOver = $currentlyMousedOver.filter(item => !(+item.id === +id && item.type === type))
          } else hovered = false
        }}
        on:click={e => {
          if ($selectingFor.includes(type)) {
            e.stopPropagation()
            if ($currentlySelected.some(e => e.id === id && e.type === type)) {
              if ($currentlySelected.length - 1 < $selectionMin) {
                // we can't deselect if doing so puts us below the minimum
                // number of selected entities
                return
              }

              $currentlySelected = $currentlySelected.filter(item => !(item.id === id && item.type === type))
            } else {
              if ($currentlySelected.length + 1 > $selectionMax) {
                // if selecting this entity puts us above the maximum
                // number of selected entities, boot the oldest one
                $currentlySelected.shift()
              }

              $currentlySelected = [...$currentlySelected, {type, id: id}]
            }
          }
        }}
      />
    </T.Group>
  {/if}
</T.Group>

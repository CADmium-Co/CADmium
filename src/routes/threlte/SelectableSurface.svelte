<script lang="ts">
	import { LineGeometry } from "three/addons/lines/LineGeometry.js"
	import type { LineMaterial } from "three/examples/jsm/lines/LineMaterial.js"
	import {
		Shape,
		ShapeGeometry,
		Vector3,
		Vector2,
		Path,
		MeshStandardMaterial,
		DoubleSide,
		Euler,
		Matrix4
	} from "three"
	import { T } from "@threlte/core"
	import { flatten } from "./projectUtils"
	import {
		currentlySelected,
		currentlyMousedOver,
		selectingFor,
		selectionMax,
		selectionMin
	} from "./stores"
	import type { EntityType, TruckEdges, TruckFace, TruckFaceBoundary } from "../../types"
	import nurbs from "nurbs"

	const log = (function () {
		const context = "[SelectableSurface.svelte]"
		return Function.prototype.bind.call(
			console.log,
			console,
			`%c${context}`,
			"font-weight:bold;color:cyan;"
		)
	})()

	export let truck_face: TruckFace, /** truck_vertices */ truck_edges: TruckEdges, id: number
	log("[props]", "truck_face:", truck_face, "truck_edges:", truck_edges, "id:", id)

	export let solidLineMaterial: LineMaterial

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
		polygonOffsetFactor: -4
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
		polygonOffsetFactor: -4
	})

	let surface = truck_face.surface
	let plane
	let exterior: LineGeometry
	let interiors: LineGeometry[] = []
	let eulerAngles: Euler = new Euler(0, 0, 0, "XYZ")

	let origin = new Vector3(0, 0, 0)

	const shape = new Shape()

	if ("Plane" in surface) {
		// cool, this surface is planar. let's extract its boundaries
		// boundaries is an array like [0, 1] where the indices point to the truck_edges array

		plane = surface.Plane
		let o = new Vector3(plane.o.x, plane.o.y, plane.o.z)
		origin = o
		let p = new Vector3(plane.p.x, plane.p.y, plane.p.z)
		let q = new Vector3(plane.q.x, plane.q.y, plane.q.z)
		let u = p.clone().sub(o).normalize()
		let v = q.clone().sub(o).normalize()

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
		let points = curveToPoints(exterior_bounds)
		exterior = new LineGeometry()
		exterior.setPositions(points)

		let projectedPoints = project(points, u, v, o)
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

		boundaries.slice(1).forEach((element) => {
			let points = curveToPoints(element)
			let ring = new LineGeometry()
			ring.setPositions(points)
			interiors.push(ring)
			log("[interiors]", interiors)

			let projectedPoints = project(points, u, v, o)
			const path = new Path()
			path.setFromPoints(projectedPoints)
			shape.holes.push(path)
		})
	}

	const geometry = new ShapeGeometry(shape)

	function project(points, u, v, o) {
		let retval = []
		// log('Points to project:', points)
		for (let i = 0; i < points.length; i += 3) {
			let point3D = new Vector3(points[i], points[i + 1], points[i + 2])
			point3D.x = point3D.x - o.x
			point3D.y = point3D.y - o.y
			point3D.z = point3D.z - o.z
			let xComponent = point3D.dot(u)
			let yComponent = point3D.dot(v)
			retval.push(new Vector2(xComponent, yComponent))
		}
		return retval
	}

	function curveToPoints(exterior: TruckFaceBoundary) {
		let points = []
		for (let { index, orientation } of exterior) {
			// log('grabbing edge: ', index, orientation)
			const edge = truck_edges[index]
			const curve = edge.curve

			if ("NURBSCurve" in curve) {
				const NURBSCurve = curve.NURBSCurve
				const knot = NURBSCurve.knot_vec
				let controlPoints = NURBSCurve.control_points
				let weights = controlPoints.map((point) => point.w)

				controlPoints = controlPoints.map((point) => [
					point.x / point.w,
					point.y / point.w,
					point.z / point.w
				])

				let nurbsCurve = nurbs({
					points: controlPoints,
					weights: weights,
					knots: knot,
					degree: 2
				})

				let domain = nurbsCurve.domain[0]
				// log('Spline Dimension:', curve.splineDimension)
				// log('Dimension:', curve.dimension)
				let a = []
				let b = []

				for (let t = domain[0]; t <= domain[1]; t += 0.02) {
					nurbsCurve.evaluate(a, t)
					let as_three = new Vector3(a[0], a[1], a[2])
					b.push(as_three)
				}

				let flattened = flatten(b)
				for (let p of flattened) {
					points.push(p)
				}
			} else if ("Line" in curve) {
				const line = curve.Line
				// log('Line:', line)

				let startPoint = line[0]
				let endPoint = line[1]

				if (orientation === false) {
					startPoint = line[1]
					endPoint = line[0]
				}

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
	$: selected = $currentlySelected.some((e) => e.id === id && e.type === type) ? true : false

	const type: EntityType = "meshFace"
</script>

<T.Group>
	{#if exterior}
		<T.Line2
			geometry={exterior}
			material={solidLineMaterial}
			on:create={({ ref }) => {
				ref.computeLineDistances()
			}}
		/>

		{#each interiors as interior}
			<T.Line2
				geometry={interior}
				material={solidLineMaterial}
				on:create={({ ref }) => {
					ref.computeLineDistances()
				}}
			/>
		{/each}

		<T.Group
			rotation.x={eulerAngles.x}
			rotation.y={eulerAngles.y}
			rotation.z={eulerAngles.z}
			position.x={origin.x}
			position.y={origin.y}
			position.z={origin.z}
		>
			<T.Mesh
				{geometry}
				material={hovered ? hoveredMaterial : standardMaterial}
				on:pointerenter={(e) => {
					// log('On Pointer Enter!')
					if ($selectingFor.includes(type)) {
						log("On enter and includes type")
						e.stopPropagation()
						hovered = true
						$currentlyMousedOver = [...$currentlyMousedOver, { type: type, id: id }]
					}
				}}
				on:pointerleave={() => {
					// log('On Pointer Leave!')
					if ($selectingFor.includes(type)) {
						hovered = false
						$currentlyMousedOver = $currentlyMousedOver.filter(
							(item) => !(item.id === id && item.type === type)
						)
					} else {
						hovered = false
					}
				}}
				on:click={(e) => {
					if ($selectingFor.includes(type)) {
						e.stopPropagation()
						if ($currentlySelected.some((e) => e.id === id && e.type === type)) {
							if ($currentlySelected.length - 1 < $selectionMin) {
								// we can't deselect if doing so puts us below the minimum
								// number of selected entities
								return
							}

							$currentlySelected = $currentlySelected.filter(
								(item) => !(item.id === id && item.type === type)
							)
						} else {
							if ($currentlySelected.length + 1 > $selectionMax) {
								// if selecting this entity puts us above the maximum
								// number of selected entities, boot the oldest one
								$currentlySelected.shift()
							}

							$currentlySelected = [...$currentlySelected, { type: type, id: id }]
						}
					}
				}}
			/>
		</T.Group>
	{/if}
</T.Group>

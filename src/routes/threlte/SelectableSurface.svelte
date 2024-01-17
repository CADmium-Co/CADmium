<script>
	import { LineGeometry } from 'three/addons/lines/LineGeometry.js'
	import { Shape, ShapeGeometry, Vector3, MeshStandardMaterial, DoubleSide } from 'three'
	import { T } from '@threlte/core'
	import { flatten, promoteTo3 } from './projectUtils'
	import { currentlySelected, currentlyMousedOver, sketchTool } from './stores'

	import nurbs from 'nurbs'

	export let truck_face, truck_vertices, truck_edges
	export let dashedLineMaterial,
		dashedHoveredMaterial,
		solidLineMaterial,
		solidHoveredMaterial,
		solidSelectedMaterial,
		collisionLineMaterial

	const standardMaterial = new MeshStandardMaterial({
		color: '#525252',
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

	let surface = truck_face.surface
	let plane
	let exterior
	let interiors = []

	const shape = new Shape()

	if ('Plane' in surface) {
		// cool, this surface is planar. let's extract its boundaries
		// boundaries is an array like [0, 1] where the indices point to the truck_edges array
		const boundaries = truck_face.boundaries
		const exterior_bounds = boundaries[0]
		console.log('Boundaries: ', boundaries)
		let points = curveToPoints(exterior_bounds)
		exterior = new LineGeometry()
		exterior.setPositions(points)
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
		})

		plane = surface.Plane
	}

	const geometry = new ShapeGeometry(shape)

	function curveToPoints(exterior) {
		let points = []
		for (let { index, orientation } of exterior) {
			console.log('grabbing edge: ', index, orientation)
			const edge = truck_edges[index]
			const curve = edge.curve

			if ('NURBSCurve' in curve) {
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
				// console.log('Spline Dimension:', curve.splineDimension)
				// console.log('Dimension:', curve.dimension)
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
			} else if ('Line' in curve) {
				const line = curve.Line
				// console.log('Line:', line)

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

		<T.Mesh {geometry} material={standardMaterial} />
	{/if}
</T.Group>

<script>
	import Point2D from './Point2D.svelte'
	import Line from './Line.svelte'
	import Circle from './Circle.svelte'
	import Arc from './Arc.svelte'
	import Face from './Face.svelte'

	import { hiddenSketches } from './stores.js'
	import { Text, Suspense } from '@threlte/extras'
	import { Matrix4, Euler, MeshStandardMaterial, Vector2, Vector3 } from 'three'
	import { T, useThrelte } from '@threlte/core'
	import { LineMaterial } from 'three/addons/lines/LineMaterial.js'
	import { LineGeometry } from 'three/addons/lines/LineGeometry.js'

	const { size, dpr } = useThrelte()

	export let editing = false
	export let uniqueId
	export let name
	export let sketch
	export let plane

	const pointIds = Object.keys(sketch.points)
	const pointTuples = []
	const pointsById = {}
	for (let pointId of pointIds) {
		const point3D = sketch.points[pointId]
		const point2D = sketch.points_2d[pointId]
		pointTuples.push({ id: pointId, twoD: point2D, threeD: point3D })
		pointsById[pointId] = { twoD: point2D, threeD: point3D }
	}

	const lineTuples = []
	for (let lineId of Object.keys(sketch.line_segments)) {
		const line = sketch.line_segments[lineId]
		const start = pointsById[line.start]
		const end = pointsById[line.end]
		lineTuples.push({ id: lineId, start, end })
	}

	const circleTuples = []
	for (let circleId of Object.keys(sketch.circles)) {
		const circle = sketch.circles[circleId]
		const center = pointsById[circle.center]
		const radius = circle.radius
		circleTuples.push({ id: circleId, center, radius })
	}

	const arcTuples = []
	for (let arcId of Object.keys(sketch.arcs)) {
		const arc = sketch.arcs[arcId]
		const center = pointsById[arc.center]
		const start = pointsById[arc.start]
		const end = pointsById[arc.end]
		arcTuples.push({ id: arcId, center, start, end })
	}

	const faceTuples = []
	for (let faceId of Object.keys(sketch.faces)) {
		const face = sketch.faces[faceId]
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
		// console.log('face:', faceId, face)
		faceTuples.push({ id: faceId, face })
	}

	// $: console.log('passive sketch plane', plane)
	// Build some Three.js vectors from the props
	const primary = new Vector3(plane.primary.x, plane.primary.y, plane.primary.z)
	const secondary = new Vector3(plane.secondary.x, plane.secondary.y, plane.secondary.z)
	const tertiary = new Vector3(plane.tertiary.x, plane.tertiary.y, plane.tertiary.z)

	// Use those to make the rotation matrix and euler angles
	const rotationMatrix = new Matrix4()
	rotationMatrix.makeBasis(primary, secondary, tertiary)
	const eulerAngles = new Euler(0, 0, 0, 'XYZ')
	eulerAngles.setFromRotationMatrix(rotationMatrix, 'XYZ')

	// Lastly, make the Plane Material
	const material = new MeshStandardMaterial({
		color: '#525292',
		metalness: 0.0,
		transparent: true,
		opacity: 0.0,
		depthWrite: false,
		depthTest: false,
		wireframe: false,
		polygonOffset: true,
		polygonOffsetFactor: -4
	})

	const width = 2.0
	const height = 1.5

	// this is x, y, z for each of five points, making a closed square
	const points = [
		-width / 2,
		-height / 2,
		0,
		width / 2,
		-height / 2,
		0,
		width / 2,
		height / 2,
		0,
		-width / 2,
		height / 2,
		0,
		-width / 2,
		-height / 2,
		0
	]

	$: lineMaterial = new LineMaterial({
		color: '#42a7eb',
		linewidth: 1.0 * $dpr,
		depthTest: true,
		transparent: true,
		dashed: false,
		resolution: new Vector2($size.width * $dpr, $size.height * $dpr)
	})

	const lineGeometry = new LineGeometry()
	lineGeometry.setPositions(points)

	$: hidden = $hiddenSketches.includes(uniqueId)
</script>

{#if !hidden}
	<T.Group rotation.x={eulerAngles.x} rotation.y={eulerAngles.y} rotation.z={eulerAngles.z}>
		<T.Mesh
			{material}
			on:click={(e) => {
				if (editing) {
					// how should we handle this event?
					// console.log(e.point)
				}
			}}
		>
			<T.PlaneGeometry args={[width * 10, height * 10]} />
		</T.Mesh>

		<T.Line2
			geometry={lineGeometry}
			material={lineMaterial}
			on:create={({ ref }) => {
				ref.computeLineDistances()
			}}
		/>

		<T.Group position.x={-width / 2 + 0.01} position.y={height / 2 - 0.01}>
			<Suspense>
				<Text text={name} color="#42a7eb" fontSize={0.05} anchorX="0%" anchorY="0%" />
			</Suspense>
		</T.Group>

		{#each circleTuples as circle (circle.id)}
			<Circle center={circle.center} radius={circle.radius} id={circle.id} />
		{/each}
	</T.Group>

	{#each pointTuples as { id, twoD, threeD } (id)}
		<Point2D {name} x={threeD.x} y={threeD.y} z={threeD.z} hidden={threeD.hidden} />
	{/each}

	{#each lineTuples as line (line.id)}
		<Line start={line.start} end={line.end} />
	{/each}

	{#each arcTuples as arc (arc.id)}
		<Arc center={arc.center} start={arc.start} end={arc.end} {plane} />
	{/each}

	<!-- {#each faceTuples as face (face.id)}
		<Face {plane} face={face.face} id={face.id} {pointsById} />
	{/each} -->
{/if}

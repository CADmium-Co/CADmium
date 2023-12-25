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
	import { addPointToSketch } from './projectUtils'

	const { size, dpr } = useThrelte()

	export let editing = false
	export let uniqueId
	export let name
	export let sketch
	export let plane

	let pointTuples = []
	let lineTuples = []
	let circleTuples = []
	let arcTuples = []
	let faceTuples = []
	let pointsById = {}

	$: {
		const pointIds = Object.keys(sketch.points)
		pointTuples = []
		pointsById = {}
		for (let pointId of pointIds) {
			const point3D = sketch.points[pointId]
			const point2D = sketch.points_2d[pointId]
			pointTuples.push({ id: pointId, twoD: point2D, threeD: point3D })
			pointsById[pointId] = { twoD: point2D, threeD: point3D }
		}

		lineTuples = []
		for (let lineId of Object.keys(sketch.line_segments)) {
			const line = sketch.line_segments[lineId]
			const start = pointsById[line.start]
			const end = pointsById[line.end]
			lineTuples.push({ id: lineId, start, end })
		}

		circleTuples = []
		for (let circleId of Object.keys(sketch.circles)) {
			const circle = sketch.circles[circleId]
			const center = pointsById[circle.center]
			const radius = circle.radius
			circleTuples.push({ id: circleId, center, radius })
		}

		arcTuples = []
		for (let arcId of Object.keys(sketch.arcs)) {
			const arc = sketch.arcs[arcId]
			const center = pointsById[arc.center]
			const start = pointsById[arc.start]
			const end = pointsById[arc.end]
			arcTuples.push({ id: arcId, center, start, end })
		}

		faceTuples = []
		for (let faceId of Object.keys(sketch.faces)) {
			const face = sketch.faces[faceId]
			faceTuples.push({ id: faceId, face })
		}
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

	function projectToPlane(point3D) {
		// point3D is a Vector3
		let xComponent = point3D.clone().sub(plane.origin).dot(primary)
		let yComponent = point3D.clone().sub(plane.origin).dot(secondary)
		return new Vector2(xComponent, yComponent)
	}
</script>

{#if !hidden}
	<T.Group rotation.x={eulerAngles.x} rotation.y={eulerAngles.y} rotation.z={eulerAngles.z}>
		<T.Mesh
			{material}
			on:click={(e) => {
				if (editing) {
					// how should we handle this event?
					let inTwoD = projectToPlane(e.point)

					addPointToSketch(uniqueId, inTwoD)
					// console.log(inTwoD)
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

		{#each arcTuples as arc (arc.id)}
			<Arc center={arc.center} start={arc.start} end={arc.end} />
		{/each}

		{#each lineTuples as line (line.id)}
			<Line start={line.start} end={line.end} />
		{/each}

		{#each pointTuples as { id, twoD, threeD } (id)}
			<Point2D {name} x={twoD.x} y={twoD.y} hidden={threeD.hidden} />
		{/each}

		{#each faceTuples as face (face.id)}
			<Face face={face.face} id={face.id} {pointsById} />
		{/each}
	</T.Group>
{/if}

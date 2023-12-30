<script>
	import { Matrix4, Euler, MeshStandardMaterial, DoubleSide, Vector2, Vector3 } from 'three'
	import { T, extend, useThrelte } from '@threlte/core'
	import { Text, Suspense } from '@threlte/extras'

	import { Line2 } from 'three/addons/lines/Line2.js'
	import { LineMaterial } from 'three/addons/lines/LineMaterial.js'
	import { LineGeometry } from 'three/addons/lines/LineGeometry.js'

	import { currentlySelected, currentlyMousedOver, selectingFor } from './stores'

	export let name, id
	export let width, height
	export let origin, primary, secondary, tertiary

	extend({ Line2 })

	const { size, dpr } = useThrelte()

	// Build some Three.js vectors from the props
	origin = new Vector3(origin.x, origin.y, origin.z)
	primary = new Vector3(primary.x, primary.y, primary.z)
	secondary = new Vector3(secondary.x, secondary.y, secondary.z)
	tertiary = new Vector3(tertiary.x, tertiary.y, tertiary.z)

	// Use those to make the rotation matrix and euler angles
	const rotationMatrix = new Matrix4()
	rotationMatrix.makeBasis(primary, secondary, tertiary)
	const eulerAngles = new Euler(0, 0, 0, 'XYZ')
	eulerAngles.setFromRotationMatrix(rotationMatrix, 'XYZ')

	// Lastly, make the Plane Material
	const material = new MeshStandardMaterial({
		color: '#525292',
		side: DoubleSide,
		metalness: 0.0,
		transparent: true,
		opacity: 0.05,
		depthWrite: false,
		depthTest: true,
		wireframe: false,
		polygonOffset: true,
		polygonOffsetFactor: -4
	})

	// this is x, y, z for each of five points, making a closed rectangle
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
		linewidth: 2.0 * $dpr,
		depthTest: true,
		transparent: true,
		dashed: false,
		resolution: new Vector2($size.width * $dpr, $size.height * $dpr)
	})

	const lineGeometry = new LineGeometry()
	lineGeometry.setPositions(points)

	const type = 'plane'

	let hovered = false
	$: selected = $currentlySelected.some((e) => e.id === id && e.type === type) ? true : false
</script>

<T.Group rotation.x={eulerAngles.x} rotation.y={eulerAngles.y} rotation.z={eulerAngles.z}>
	<T.Mesh
		{material}
		on:pointerenter={() => {
			if ($selectingFor.includes(type)) {
				hovered = true
				$currentlyMousedOver = [...$currentlyMousedOver, { type: type, id: id }]
			}
		}}
		on:pointerleave={() => {
			if ($selectingFor.includes(type)) {
				hovered = false
				$currentlyMousedOver = $currentlyMousedOver.filter(
					(item) => !(item.id === id && item.type === type)
				)
			}
		}}
	>
		<T.PlaneGeometry args={[width, height]} />
	</T.Mesh>

	<T.Line2
		geometry={lineGeometry}
		material={lineMaterial}
		on:create={({ ref }) => {
			ref.computeLineDistances()
		}}
	/>

	<T.Group position.x={(-width / 2) * 0.99} position.y={(height / 2) * 0.99}>
		<Suspense>
			<Text text={name} color="#42a7eb" fontSize={5} anchorX="0%" anchorY="0%" />
		</Suspense>
	</T.Group>
</T.Group>

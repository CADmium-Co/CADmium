<script>
	import { Matrix4, Euler, MeshStandardMaterial, DoubleSide, Vector3 } from 'three'
	import { T } from '@threlte/core'
	import { Text, Suspense } from '@threlte/extras'

	export let name
	export let width
	export let height
	export let origin
	export let primary
	export let secondary
	export let tertiary

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
		opacity: 0.1,
		depthWrite: false,
		depthTest: true,
		wireframe: false
	})
</script>

<T.Group rotation.x={eulerAngles.x} rotation.y={eulerAngles.y} rotation.z={eulerAngles.z}>
	<T.Mesh {material}>
		<T.PlaneGeometry args={[width, height]} />
	</T.Mesh>

	<T.Group position.x={-0.5} position.y={0.5} position.z={0}>
		<Suspense>
			<Text text={name} color="black" fontSize={0.1} anchorX="0%" anchorY="0%" />
		</Suspense>
	</T.Group>
</T.Group>

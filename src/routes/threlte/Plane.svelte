<script>
	import { MeshStandardMaterial, DoubleSide, BufferGeometry, BufferAttribute, Vector3 } from 'three'
	import { T } from '@threlte/core'

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
	const half_width = width / 2
	const half_height = height / 2

	// Use them to compute the four corners of the plane
	const upper_right = origin
		.clone()
		.addScaledVector(primary, half_width)
		.addScaledVector(secondary, half_height)
	const upper_left = origin
		.clone()
		.addScaledVector(primary, -half_width)
		.addScaledVector(secondary, half_height)
	const lower_right = origin
		.clone()
		.addScaledVector(primary, half_width)
		.addScaledVector(secondary, -half_height)
	const lower_left = origin
		.clone()
		.addScaledVector(primary, -half_width)
		.addScaledVector(secondary, -half_height)
	const label_position = upper_left.clone().addScaledVector(tertiary, 0.001)

	// Now build the BufferGeometry
	const geometry = new BufferGeometry()
	const vertices = new Float32Array([
		lower_left.x,
		lower_left.y,
		lower_left.z,
		lower_right.x,
		lower_right.y,
		lower_right.z,
		upper_right.x,
		upper_right.y,
		upper_right.z,
		upper_right.x,
		upper_right.y,
		upper_right.z,
		upper_left.x,
		upper_left.y,
		upper_left.z,
		lower_left.x,
		lower_left.y,
		lower_left.z
	])
	geometry.setAttribute('position', new BufferAttribute(vertices, 3))
	const normals = new Float32Array([
		tertiary.x,
		tertiary.y,
		tertiary.z,
		tertiary.x,
		tertiary.y,
		tertiary.z,
		tertiary.x,
		tertiary.y,
		tertiary.z,
		tertiary.x,
		tertiary.y,
		tertiary.z,
		tertiary.x,
		tertiary.y,
		tertiary.z,
		tertiary.x,
		tertiary.y,
		tertiary.z
	])
	geometry.setAttribute('normal', new BufferAttribute(normals, 3))

	// And lastly the Material
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

	// const mesh = new Mesh(geometry, material)
</script>

<T.Mesh {geometry} {material} />

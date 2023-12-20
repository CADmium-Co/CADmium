<script>
	import * as THREE from 'three'
	import { T } from '@threlte/core'

	export let name
	export let indices
	export let vertices
	export let normals

	const geometry = new THREE.BufferGeometry()

	const normalsArray = new Float32Array(normals.flatMap((v) => [v.x, v.y, v.z]))
	const verticesArray = new Float32Array(vertices.flatMap((v) => [v.x, v.y, v.z]))

	geometry.setIndex(indices)
	geometry.setAttribute('position', new THREE.Float32BufferAttribute(verticesArray, 3))
	geometry.setAttribute('normal', new THREE.Float32BufferAttribute(normalsArray, 3))

	const material = new THREE.MeshStandardMaterial({
		color: '#999999',
		side: THREE.DoubleSide,
		wireframe: false,
		metalness: 1.0,
		roughness: 0.6
	})

	let edges = new THREE.EdgesGeometry(geometry, 15)
	let mat = new THREE.LineBasicMaterial({ color: 0x000000 })
</script>

<T.Group>
	<T.Mesh {geometry} {material} />
	<T.LineSegments geometry={edges} material={mat} />
</T.Group>

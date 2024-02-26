<script>
	import * as THREE from 'three'
	import { T } from '@threlte/core'
	import SelectableSurface from './SelectableSurface.svelte'

	export let name
	export let indices
	export let vertices
	export let normals
	export let truckSolid
	export let dashedLineMaterial,
		dashedHoveredMaterial,
		solidLineMaterial,
		solidHoveredMaterial,
		solidSelectedMaterial,
		collisionLineMaterial

	let truck_vertices, truck_edges, truck_faces

	$: {
		let boundaries = truckSolid.boundaries[0]
		truck_vertices = boundaries.vertices
		truck_edges = boundaries.edges
		truck_faces = boundaries.faces

		// console.log('vertices', truck_vertices)
		// console.log('edges', truck_edges)
		// console.log('faces', truck_faces)
	}

	const geometry = new THREE.BufferGeometry()

	const normalsArray = new Float32Array(normals.flatMap((v) => [v.x, v.y, v.z]))
	const verticesArray = new Float32Array(vertices.flatMap((v) => [v.x, v.y, v.z]))

	console.log('Vertices: ', vertices.length)

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

	{#each truck_faces as truck_face, i (i)}
		<SelectableSurface
			id={i}
			{truck_face}
			{truck_vertices}
			{truck_edges}
			{solidLineMaterial}
			{solidHoveredMaterial}
			{solidSelectedMaterial}
			{dashedHoveredMaterial}
			{dashedLineMaterial}
			{collisionLineMaterial}
		/>
	{/each}
</T.Group>

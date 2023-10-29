import * as THREE from 'three'

class Solid {
	constructor(name, original_solid, element) {
		this.original_solid = original_solid
		this.name = name
		this.element = element

		// console.log('Original solid', original_solid.vertices.length)
		const geometry = new THREE.BufferGeometry()

		const normals = new Float32Array(original_solid.normals.flatMap((v) => [v.x, v.y, v.z]))
		const vertices = new Float32Array(original_solid.vertices.flatMap((v) => [v.x, v.y, v.z]))

		geometry.setIndex(original_solid.indices)
		geometry.setAttribute('position', new THREE.Float32BufferAttribute(vertices, 3))
		geometry.setAttribute('normal', new THREE.Float32BufferAttribute(normals, 3))

		const material = new THREE.MeshStandardMaterial({ color: 0xaaaaaa })
		this.mesh = new THREE.Mesh(geometry, material)

		this.group = new THREE.Group()
		this.group.add(this.mesh)

		// add edges!
		let edges = new THREE.EdgesGeometry(geometry, 15)
		let mat = new THREE.LineBasicMaterial({ color: 0x000000, linewidth: 2 })
		let wireframe = new THREE.LineSegments(edges, mat)
		wireframe.renderOrder = 1 // make sure wireframes are rendered 2nd

		this.group.add(wireframe)
	}

	addTo(object) {
		object.add(this.group)
	}

	removeFrom(object) {
		object.remove(this.group)
	}
}

export { Solid }

import * as THREE from 'three'

class Solid {
	constructor(name, original_solid, element) {
		this.original_solid = original_solid
		this.name = name
		this.element = element

		console.log('Original solid', original_solid)
		const geometry = new THREE.BufferGeometry()

		const normals = new Float32Array(original_solid.normals.flatMap((v) => [v.x, v.y, v.z]))
		const vertices = new Float32Array(original_solid.vertices.flatMap((v) => [v.x, v.y, v.z]))

		geometry.setIndex(original_solid.indices)
		geometry.setAttribute('position', new THREE.Float32BufferAttribute(vertices, 3))
		geometry.setAttribute('normal', new THREE.Float32BufferAttribute(normals, 3))

		const material = new THREE.MeshStandardMaterial({ color: 0xff0000 })
		this.mesh = new THREE.Mesh(geometry, material)

		this.group = new THREE.Group()
		this.group.add(this.mesh)
	}

	addTo(object) {
		object.add(this.group)
	}

	removeFrom(object) {
		object.remove(this.group)
	}
}

export { Solid }

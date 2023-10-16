import * as THREE from 'three'

class Point {
	constructor(name, { x, y, z }, point_2d, parent = null) {
		this.name = name
		this.x = x
		this.y = y
		this.z = z
		this.x_2d = point_2d.x
		this.y_2d = point_2d.y
		this.parent = parent

		let image = '/actions/point_min.svg'
		if (parent) {
			image = '/actions/simple_point_min.svg'
		}

		let tex = new THREE.TextureLoader().load(image)
		const geom = new THREE.BufferGeometry()
		const vertices = new Float32Array([x, y, z])
		geom.setAttribute('position', new THREE.Float32BufferAttribute(vertices, 3))
		const material = new THREE.PointsMaterial({
			size: parent ? 6.0 : 12.0,
			map: tex,
			transparent: true,
			sizeAttenuation: false
		})
		material.depthTest = parent ? true : false
		const mesh = new THREE.Points(geom, material)
		this.mesh = mesh
		this.mesh.renderOrder = 2
	}

	addTo(object) {
		object.add(this.mesh)
	}

	removeFrom(object) {
		object.remove(this.mesh)
	}
}

export { Point }

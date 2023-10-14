import * as THREE from 'three'

import { circleToPoints, arcToPoints } from './utils'

class Face {
	constructor(face, real_plane, parent, points) {
		this.real_plane = real_plane

		const shape = new THREE.Shape()

		let exterior = face.exterior
		// console.log('ext', exterior)
		let shape_points = []
		if (exterior.Circle) {
			let center_point = points[`${parent}:${exterior.Circle.center}`]
			let center_2d = new THREE.Vector2(center_point.x_2d, center_point.y_2d)

			let as_points = circleToPoints(center_2d, exterior.Circle.radius)

			for (let point of as_points) {
				shape_points.push([point.x, point.y])
			}
		} else {
			for (let segment of exterior.Segments) {
				if (segment.type === 'Line') {
					let start_point = points[`${parent}:${segment.start}`]
					let end_point = points[`${parent}:${segment.end}`]

					let start_point_2d = [start_point.x_2d, start_point.y_2d]
					let end_point_2d = [end_point.x_2d, end_point.y_2d]

					if (shape_points.length === 0) {
						shape_points.push(start_point_2d)
					}
					shape_points.push(end_point_2d)
				} else if (segment.type === 'Arc') {
					let center_point = points[`${parent}:${segment.center}`]
					let start_point = points[`${parent}:${segment.start}`]
					let end_point = points[`${parent}:${segment.end}`]

					let center_point_2d = new THREE.Vector2(center_point.x_2d, center_point.y_2d)
					let start_point_2d = new THREE.Vector2(start_point.x_2d, start_point.y_2d)
					let end_point_2d = new THREE.Vector2(end_point.x_2d, end_point.y_2d)

					let as_points = arcToPoints(
						center_point_2d,
						start_point_2d,
						end_point_2d,
						segment.clockwise
					)

					if (shape_points.length !== 0) {
						as_points.shift()
					}

					for (let point of as_points) {
						shape_points.push([point.x, point.y])
					}
				}
			}
		}

		if (shape_points.length > 0) {
			shape.moveTo(shape_points[0][0], shape_points[0][1])
			for (let i = 1; i < shape_points.length; i++) {
				shape.lineTo(shape_points[i][0], shape_points[i][1])
			}
		}

		const geometry = new THREE.ShapeGeometry(shape)
		const material = new THREE.MeshStandardMaterial({
			color: 0xc0c0c0,
			side: THREE.DoubleSide,
			transparent: true,
			opacity: 0.3,
			depthWrite: false,
			depthTest: false
			// polygonOffset: true,
			// polygonOffsetFactor: 2,
			// polygonOffsetUnits: 1
		})

		let { origin, primary, secondary, tertiary } = this.real_plane.plane
		origin = new THREE.Vector3(origin.x, origin.y, origin.z)
		primary = new THREE.Vector3(primary.x, primary.y, primary.z)
		secondary = new THREE.Vector3(secondary.x, secondary.y, secondary.z)
		tertiary = new THREE.Vector3(tertiary.x, tertiary.y, tertiary.z)

		// we need to rotate properly
		const m = new THREE.Matrix4()
		m.makeBasis(primary, secondary, tertiary)
		const ea = new THREE.Euler(0, 0, 0, 'XYZ')
		ea.setFromRotationMatrix(m, 'XYZ')
		this.mesh = new THREE.Mesh(geometry, material)
		this.mesh.rotation.x = ea.x
		this.mesh.rotation.y = ea.y
		this.mesh.rotation.z = ea.z
		this.mesh.position.x = origin.x
		this.mesh.position.y = origin.y
		this.mesh.position.z = origin.z
	}
	addTo(object) {
		object.add(this.mesh)
	}
}

export { Face }

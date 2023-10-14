import * as THREE from 'three'

import { Line2 } from 'three/addons/lines/Line2.js'
import { LineMaterial } from 'three/addons/lines/LineMaterial.js'
import { LineGeometry } from 'three/addons/lines/LineGeometry.js'

import { ARC_TOLERANCE, SKETCH_LINE_WIDTH } from './utils'

class Arc {
	constructor(name, { center, start, end, clockwise }, real_plane, parent, points, element) {
		this.name = name
		this.real_plane = real_plane
		let plane = real_plane.plane

		let o = new THREE.Vector3(plane.origin.x, plane.origin.y, plane.origin.z)
		let x = new THREE.Vector3(plane.primary.x, plane.primary.y, plane.primary.z)
		let y = new THREE.Vector3(plane.secondary.x, plane.secondary.y, plane.secondary.z)
		let z = new THREE.Vector3(plane.tertiary.x, plane.tertiary.y, plane.tertiary.z)

		let center_point = points[`${parent}:${center}`]
		let center_2d = new THREE.Vector2(center_point.x_2d, center_point.y_2d)
		center_point = new THREE.Vector3(center_point.x, center_point.y, center_point.z)

		let start_point = points[`${parent}:${start}`]
		let start_2d = new THREE.Vector2(start_point.x_2d, start_point.y_2d)
		start_point = new THREE.Vector3(start_point.x, start_point.y, start_point.z)

		let end_point = points[`${parent}:${end}`]
		end_point = new THREE.Vector3(end_point.x, end_point.y, end_point.z)

		let start_angle = Math.atan2(start_2d.y - center_2d.y, start_2d.x - center_2d.x)

		// see https://math.stackexchange.com/a/4132095/816177
		const tolerance = ARC_TOLERANCE // in meters
		const radius = center_point.distanceTo(start_point)
		const k = tolerance / radius
		const n = Math.ceil(Math.PI / Math.sqrt(2 * k))
		const segment_angle = (2 * Math.PI) / n
		const segment_length = radius * segment_angle

		const line_vertices = []
		line_vertices.push(start_point.x, start_point.y, start_point.z)
		for (let i = 1; i <= n; i++) {
			let theta = ((2 * Math.PI) / n) * i + start_angle
			let x_component = x.clone().multiplyScalar(radius * Math.cos(theta))
			let y_component = y.clone().multiplyScalar(radius * Math.sin(theta))
			let point = o.clone().add(x_component).add(y_component)
			point.add(center_point)
			line_vertices.push(point.x, point.y, point.z)

			let distance_to_end = point.distanceTo(end_point)
			if (distance_to_end <= segment_length) {
				line_vertices.push(end_point.x, end_point.y, end_point.z)
				break
			}
		}
		const line_geometry = new LineGeometry()
		line_geometry.setPositions(line_vertices)

		this.defaultMaterial = new LineMaterial({
			color: '#000000',
			linewidth: (this.lineWidth =
				SKETCH_LINE_WIDTH * window.devicePixelRatio * window.devicePixelRatio),
			depthTest: false,
			transparent: true,
			dashed: false,
			resolution: new THREE.Vector2(
				element.width * window.devicePixelRatio,
				element.height * window.devicePixelRatio
			)
		})

		const fat_line = new Line2(line_geometry, this.defaultMaterial)
		fat_line.computeLineDistances()
		this.mesh = fat_line
	}

	addTo(object) {
		object.add(this.mesh)
	}
}

export { Arc }

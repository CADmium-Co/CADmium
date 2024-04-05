import * as THREE from "three"

import { LineMaterial } from "three/addons/lines/LineMaterial.js"
import { LineGeometry } from "three/addons/lines/LineGeometry.js"
import { Line2 } from "three/addons/lines/Line2.js"

import { CIRCLE_TOLERANCE, SKETCH_LINE_WIDTH } from "./utils"

class Circle {
	constructor(name, original_circle, real_plane, parent, points, element) {
		this.original_circle = original_circle
		let { radius, center } = original_circle
		this.name = name
		this.real_plane = real_plane
		let plane = real_plane.plane

		let o = new THREE.Vector3(plane.origin.x, plane.origin.y, plane.origin.z)
		let x = new THREE.Vector3(plane.primary.x, plane.primary.y, plane.primary.z)
		let y = new THREE.Vector3(plane.secondary.x, plane.secondary.y, plane.secondary.z)
		let center_point = points[`${parent}:${center}`]

		// see https://math.stackexchange.com/a/4132095/816177
		const tolerance = CIRCLE_TOLERANCE // in meters
		const k = tolerance / radius
		// more precise but slower to calculate:
		// const n = Math.ceil(Math.PI / Math.acos(1 - k))
		// faster to calculate, at most only overestimates by 1:
		const n = Math.ceil(Math.PI / Math.sqrt(2 * k))

		const line_vertices = []
		for (let i = 0; i <= n; i++) {
			let theta = ((2 * Math.PI) / n) * i
			let x_component = x.clone().multiplyScalar(radius * Math.cos(theta))
			let y_component = y.clone().multiplyScalar(radius * Math.sin(theta))
			let point = o.clone().add(x_component).add(y_component)
			point.add(center_point)
			line_vertices.push(point.x, point.y, point.z)
		}
		const line_geometry = new LineGeometry()
		line_geometry.setPositions(line_vertices)

		this.defaultMaterial = new LineMaterial({
			color: "#000000",
			linewidth: (this.lineWidth = SKETCH_LINE_WIDTH * window.devicePixelRatio * window.devicePixelRatio),
			depthTest: true,
			transparent: true,
			dashed: false,
			resolution: new THREE.Vector2(element.width * window.devicePixelRatio, element.height * window.devicePixelRatio)
		})

		const fat_line = new Line2(line_geometry, this.defaultMaterial)
		fat_line.computeLineDistances()
		this.mesh = fat_line
	}

	addTo(object) {
		object.add(this.mesh)
	}
}
export { Circle }

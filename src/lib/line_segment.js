import * as THREE from 'three'

import { Line2 } from 'three/addons/lines/Line2.js'
import { LineMaterial } from 'three/addons/lines/LineMaterial.js'
import { LineGeometry } from 'three/addons/lines/LineGeometry.js'

import { SKETCH_LINE_WIDTH } from './utils'

class LineSegment {
	constructor(name, { start, end }, real_plane, parent, points, element) {
		this.name = name
		this.start = start
		this.end = end
		this.parent = parent
		this.real_plane = real_plane

		let start_point = points[`${parent}:${start}`]
		let end_point = points[`${parent}:${end}`]

		const line_vertices = [
			start_point.x,
			start_point.y,
			start_point.z,
			end_point.x,
			end_point.y,
			end_point.z
		]
		const line_geometry = new LineGeometry()
		line_geometry.setPositions(line_vertices)

		let resolution_vector = new THREE.Vector2(
			element.width * window.devicePixelRatio,
			element.height * window.devicePixelRatio
		)
		let line_w = (this.lineWidth =
			SKETCH_LINE_WIDTH * window.devicePixelRatio * window.devicePixelRatio)

		this.selectionStatus = 'unselected' // could also be 'mouseOver' or 'selected'

		this.defaultMaterial = new LineMaterial({
			color: '#000000',
			linewidth: line_w,
			resolution: resolution_vector
		})

		this.mouseOverMaterial = new LineMaterial({
			color: '#ffa500',
			linewidth: line_w,
			resolution: resolution_vector
		})

		this.selectedMaterial = new LineMaterial({
			color: '#ffa500',
			linewidth: line_w,
			resolution: resolution_vector
		})

		const fat_line = new Line2(line_geometry, this.defaultMaterial)
		fat_line.computeLineDistances()
		this.mesh = fat_line
		this.mesh.name = name
	}

	addTo(object) {
		object.add(this.mesh)
	}

	setSelectionStatus(status) {
		console.log('setting selection status to', status)
		if (status === 'unselected') {
			this.mesh.material = this.defaultMaterial
		} else if (status === 'mouseOver') {
			this.mesh.material = this.mouseOverMaterial
		} else if (status === 'selected') {
			this.mesh.material = this.selectedMaterial
		} else {
			throw new Error('Invalid selection status: ', status)
		}
		this.selectionStatus = status
	}
}

export { LineSegment }

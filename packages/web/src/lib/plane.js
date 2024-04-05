import * as THREE from "three"

import { Line2 } from "three/addons/lines/Line2.js"
import { LineMaterial } from "three/addons/lines/LineMaterial.js"
import { LineGeometry } from "three/addons/lines/LineGeometry.js"

import { Text } from "troika-three-text"

class Plane {
	constructor(name, { plane, width, height }, element) {
		let { origin, primary, secondary, tertiary } = plane
		this.origin = origin
		this.primary = primary
		this.secondary = secondary
		this.tertiary = tertiary
		this.width = width
		this.height = height
		this.name = name

		this.fillColor = "#525292"
		this.strokeColor = "#42a7eb"
		this.lineWidth = 2.0 * window.devicePixelRatio * window.devicePixelRatio
		this.material = new THREE.MeshStandardMaterial({
			color: this.fillColor,
			// color: '#ffff00',
			side: THREE.DoubleSide,
			metalness: 0.0,
			transparent: true,
			opacity: 0.1,
			depthWrite: false,
			depthTest: true,
			wireframe: false
			// polygonOffset: true,
			// polygonOffsetFactor: 4.0,
			// polygonOffsetUnits: 4.0
		})
		this.lineMaterial = new LineMaterial({
			color: this.strokeColor,
			linewidth: this.lineWidth,
			depthTest: true,
			transparent: true,
			dashed: false,
			resolution: new THREE.Vector2(element.width * window.devicePixelRatio, element.height * window.devicePixelRatio)
		})

		this.mouseOverFillColor = "#525292"
		this.mouseOverStrokeColor = "#ffa500"
		this.mouseOverLineWidth = 2.0
		this.mouseOverMaterial = new THREE.MeshStandardMaterial({
			color: this.mouseOverFillColor,
			side: THREE.DoubleSide,
			metalness: 0.0,
			transparent: true,
			opacity: 0.05,
			depthWrite: false
		})
		this.mouseOverLineMaterial = new LineMaterial({
			color: this.mouseOverStrokeColor,
			linewidth: this.lineWidth,
			depthTest: true,
			transparent: true,
			dashed: false,
			resolution: new THREE.Vector2(element.width * window.devicePixelRatio, element.height * window.devicePixelRatio)
		})

		this.selectedFillColor = "#525292"
		this.selectedStrokeColor = "#ff0000"
		this.selectedLineWidth = 2.0
		this.selectedMaterial = new THREE.MeshStandardMaterial({
			color: this.selectedFillColor,
			side: THREE.DoubleSide,
			metalness: 0.0,
			transparent: true,
			opacity: 0.05,
			depthWrite: false
		})
		this.selectedLineMaterial = new LineMaterial({
			color: this.selectedStrokeColor,
			linewidth: this.lineWidth,
			depthTest: true,
			transparent: true,
			dashed: false,
			resolution: new THREE.Vector2(element.width * window.devicePixelRatio, element.height * window.devicePixelRatio)
		})

		this.selectionStatus = "unselected" // could also be 'mouseOver' or 'selected'

		origin = new THREE.Vector3(origin.x, origin.y, origin.z)
		primary = new THREE.Vector3(primary.x, primary.y, primary.z)
		secondary = new THREE.Vector3(secondary.x, secondary.y, secondary.z)
		tertiary = new THREE.Vector3(tertiary.x, tertiary.y, tertiary.z)

		let half_width = width / 2
		let half_height = height / 2

		const upper_right = origin.clone().addScaledVector(primary, half_width).addScaledVector(secondary, half_height)
		const upper_left = origin.clone().addScaledVector(primary, -half_width).addScaledVector(secondary, half_height)
		const lower_right = origin.clone().addScaledVector(primary, half_width).addScaledVector(secondary, -half_height)
		const lower_left = origin.clone().addScaledVector(primary, -half_width).addScaledVector(secondary, -half_height)
		const label_position = upper_left.clone().addScaledVector(tertiary, 0.001)

		const geometry = new THREE.BufferGeometry()
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

		geometry.setAttribute("position", new THREE.BufferAttribute(vertices, 3))
		geometry.setAttribute("normal", new THREE.BufferAttribute(normals, 3))

		const mesh = new THREE.Mesh(geometry, this.material)

		const line_vertices = [
			lower_left.x,
			lower_left.y,
			lower_left.z,
			lower_right.x,
			lower_right.y,
			lower_right.z,
			upper_right.x,
			upper_right.y,
			upper_right.z,
			upper_left.x,
			upper_left.y,
			upper_left.z,
			lower_left.x,
			lower_left.y,
			lower_left.z
		]
		const line_geometry = new LineGeometry()
		line_geometry.setPositions(line_vertices)

		const fat_line = new Line2(line_geometry, this.lineMaterial)
		fat_line.computeLineDistances()

		const label = new Text()

		// Set properties to configure:
		label.text = " " + name
		label.fontSize = 0.05
		label.position.x = label_position.x
		label.position.y = label_position.y
		label.position.z = label_position.z
		label.color = 0x42a7eb
		label.depthOffset = -1

		// Update the rendering:
		label.sync()

		// we need to rotate the text properly
		const m = new THREE.Matrix4()
		m.makeBasis(primary, secondary, tertiary)
		const ea = new THREE.Euler(0, 0, 0, "XYZ")
		ea.setFromRotationMatrix(m, "XYZ")
		this.ea = ea
		label.rotation.x = ea.x
		label.rotation.y = ea.y
		label.rotation.z = ea.z

		label.renderOrder = 1

		this.mesh = mesh
		this.line = fat_line
		this.label = label

		this.mesh.name = name
	}

	addTo(object) {
		object.add(this.mesh)
		object.add(this.line)
		object.add(this.label)
	}

	removeFrom(object) {
		object.remove(this.mesh)
		object.remove(this.line)
		object.remove(this.label)
	}

	setSelectionStatus(status) {
		if (status === "unselected") {
			this.mesh.material.color.set(this.fillColor)
			this.line.material = this.lineMaterial
		} else if (status === "mouseOver") {
			this.mesh.material.color.set(this.mouseOverFillColor)
			this.line.material = this.mouseOverLineMaterial
		} else if (status === "selected") {
			this.mesh.material.color.set(this.selectedFillColor)
			this.line.material = this.selectedLineMaterial
		} else {
			throw new Error("Invalid selection status: ", status)
		}
		this.selectionStatus = status
	}
}

export { Plane }

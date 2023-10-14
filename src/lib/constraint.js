import * as THREE from 'three'
import { Text } from 'troika-three-text'

class Constraint {
	constructor(name, original_constraint, parent, points, circles) {
		this.parent = parent
		this.original_constraint = original_constraint
		this.name = name

		if (original_constraint.type === 'CircleDiameter') {
			// console.log('Circ diam', original_constraint)

			// we need an arrow that points to the center of the circle
			// with a text label that says the diameter
			// colored so that bigger error = more red

			let extendedKey = `${parent}:${original_constraint.circle_id}`
			let circle = circles[extendedKey]
			let plane = circle.real_plane.plane
			// console.log('circle', circle)
			// console.log('plane', plane)

			let center_point = points[`${parent}:${circle.original_circle.center}`]
			// console.log(center_point)

			let x_component =
				Math.cos(original_constraint.angle_offset) *
				(circle.original_circle.radius + original_constraint.r_offset)
			let y_component =
				Math.sin(original_constraint.angle_offset) *
				(circle.original_circle.radius + original_constraint.r_offset)

			let end_point = new THREE.Vector3(center_point.x, center_point.y, center_point.z)
			let primary = new THREE.Vector3(plane.primary.x, plane.primary.y, plane.primary.z)
			let secondary = new THREE.Vector3(plane.secondary.x, plane.secondary.y, plane.secondary.z)
			let tertiary = new THREE.Vector3(plane.tertiary.x, plane.tertiary.y, plane.tertiary.z)

			end_point.addScaledVector(primary, x_component)
			end_point.addScaledVector(secondary, y_component)
			// console.log('end_point', end_point, original_constraint.diameter)

			const label = new Text()
			label.text = original_constraint.diameter.toFixed(2)
			label.fontSize = 0.05
			label.position.x = end_point.x
			label.position.y = end_point.y
			label.position.z = end_point.z

			let r_val = Math.min(1, Math.abs(original_constraint.error) * 4)
			console.log('r val', r_val)
			label.color = new THREE.Color(r_val, 0, 0)

			label.anchorX = 'center'
			label.anchorY = 'middle'
			label.depthOffset = -1
			label.sync()
			this.label = label

			// we need to rotate the text properly
			const m = new THREE.Matrix4()
			m.makeBasis(primary, secondary, tertiary)
			const ea = new THREE.Euler(0, 0, 0, 'XYZ')
			ea.setFromRotationMatrix(m, 'XYZ')
			this.label.rotation.x = ea.x
			this.label.rotation.y = ea.y
			this.label.rotation.z = ea.z
		}
	}
	addTo(object) {
		if (this.label) {
			object.add(this.label)
		}
	}
}

export { Constraint }

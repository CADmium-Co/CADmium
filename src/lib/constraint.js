import * as THREE from 'three'
import { Text } from 'troika-three-text'

class Constraint {
	constructor(name, original_constraint, real_plane, parent, points, lines, circles) {
		this.parent = parent
		this.real_plane = real_plane
		this.compute_plane(real_plane.plane)
		this.original_constraint = original_constraint
		this.name = name

		if (original_constraint.type === 'CircleDiameter') {
			// we need an arrow that points to the center of the circle
			// with a text label that says the diameter
			// colored so that bigger error = more red
			const extendedKey = `${parent}:${original_constraint.circle_id}`
			const circle = circles[extendedKey]

			const center_point = points[`${parent}:${circle.original_circle.center}`]
			const end_point = new THREE.Vector3(center_point.x, center_point.y, center_point.z)
			const start_point = end_point.clone()
			const text_point = end_point.clone()

			const cos = Math.cos(original_constraint.angle_offset)
			const sin = Math.sin(original_constraint.angle_offset)

			end_point.addScaledVector(
				this.primary,
				cos * (circle.original_circle.radius + original_constraint.r_offset - 0.05)
			)
			end_point.addScaledVector(
				this.secondary,
				sin * (circle.original_circle.radius + original_constraint.r_offset - 0.05)
			)

			text_point.addScaledVector(
				this.primary,
				cos * (circle.original_circle.radius + original_constraint.r_offset)
			)
			text_point.addScaledVector(
				this.secondary,
				sin * (circle.original_circle.radius + original_constraint.r_offset)
			)

			start_point.addScaledVector(this.primary, cos * circle.original_circle.radius)
			start_point.addScaledVector(this.secondary, sin * circle.original_circle.radius)

			const arrow_dir = start_point.clone().sub(end_point).normalize()
			const arrow_len = end_point.distanceTo(start_point)
			const arrow = new THREE.ArrowHelper(arrow_dir, end_point, arrow_len, 0x000000, 0.05, 0.025)

			this.label.text = original_constraint.diameter.toFixed(2)
			this.label.position.x = text_point.x
			this.label.position.y = text_point.y
			this.label.position.z = text_point.z

			let r_val = Math.min(1, Math.abs(original_constraint.error) * 4)
			this.label.color = new THREE.Color(r_val, 0, 0)
			this.label.sync()

			this.group.add(this.label)
			this.group.add(arrow)
		} else if (original_constraint.type === 'SegmentLength') {
			// console.log('og', original_constraint)
			const extendedKey = `${name}:${original_constraint.segment_id}`
			const line = lines[extendedKey]
			const start_point = points[`${name}:${line.start}`]
			const end_point = points[`${name}:${line.end}`]

			const start = new THREE.Vector3(start_point.x, start_point.y, start_point.z)
			const end = new THREE.Vector3(end_point.x, end_point.y, end_point.z)
			const difference = end.clone().sub(start)
			const midpoint = start.clone().addScaledVector(difference, 0.5)

			const dir = difference.clone().normalize()
			const normal = dir.clone().cross(this.tertiary)

			const edge_buffer = 0.01

			const offset_midpoint = midpoint
				.clone()
				.addScaledVector(normal, original_constraint.normal_offset)

			const edge_0_end = start
				.clone()
				.addScaledVector(normal, original_constraint.normal_offset + edge_buffer)
			const geometry_edge_0 = new THREE.BufferGeometry().setFromPoints([start, edge_0_end])
			const edge_0 = new THREE.Line(
				geometry_edge_0,
				new THREE.LineBasicMaterial({ color: 0x000000, linewidth: 1 })
			)
			this.group.add(edge_0)

			const edge_1_end = end
				.clone()
				.addScaledVector(normal, original_constraint.normal_offset + edge_buffer)
			const geometry_edge_1 = new THREE.BufferGeometry().setFromPoints([end, edge_1_end])
			const edge_1 = new THREE.Line(
				geometry_edge_1,
				new THREE.LineBasicMaterial({ color: 0x000000, linewidth: 1 })
			)
			this.group.add(edge_1)

			const text_point = offset_midpoint.clone()

			this.label.text = original_constraint.length.toFixed(2)
			this.label.position.x = text_point.x
			this.label.position.y = text_point.y
			this.label.position.z = text_point.z

			let r_val = Math.min(1, Math.abs(original_constraint.error) * 4)
			this.label.color = new THREE.Color(r_val, 0, 0)
			this.label.sync()

			this.group.add(this.label)

			// const arrow_len = end_point.distanceTo(start_point)
			let tiny_delta_size = 0.05
			let tiny_delta = dir.clone().multiplyScalar(tiny_delta_size)
			const arrow_0 = new THREE.ArrowHelper(
				dir.clone().negate(),
				offset_midpoint.clone().sub(tiny_delta),
				difference.length() / 2 - tiny_delta_size,
				0x000000,
				0.05,
				0.025
			)
			this.group.add(arrow_0)

			const arrow_1 = new THREE.ArrowHelper(
				dir,
				offset_midpoint.clone().add(tiny_delta),
				difference.length() / 2 - tiny_delta_size,
				0x000000,
				0.05,
				0.025
			)
			this.group.add(arrow_1)
		} else if (original_constraint.type === 'SegmentAngle') {
			console.log('SA', original_constraint)
			const extendedKey = `${name}:${original_constraint.segment_id}`
			const line = lines[extendedKey]
			const start_point = points[`${name}:${line.start}`]
			const end_point = points[`${name}:${line.end}`]

			const start = new THREE.Vector3(start_point.x, start_point.y, start_point.z)
			const end = new THREE.Vector3(end_point.x, end_point.y, end_point.z)
			const difference = end.clone().sub(start)
			const midpoint = start.clone().addScaledVector(difference, 0.5)

			let image = '/actions/horizontal.svg'

			let tex = new THREE.TextureLoader().load(image)
			tex.center = new THREE.Vector2(0.5, 0.5)
			tex.rotation = original_constraint.angle

			const geom = new THREE.BufferGeometry()
			const vertices = new Float32Array([midpoint.x, midpoint.y, midpoint.z])
			geom.setAttribute('position', new THREE.Float32BufferAttribute(vertices, 3))
			const material = new THREE.PointsMaterial({
				size: 40.0,
				map: tex,
				color: 0xffffff,
				// opacity: 1.0,
				transparent: false,
				alphaTest: 0.5
			})
			// material.depthTest = true
			const mesh = new THREE.Points(geom, material)
			mesh.renderOrder = 0
			// mesh.setRotationFromAxisAngle(this.tertiary, original_constraint.angle_offset)

			this.group.add(mesh)
		}

		// console.log(original_constraint.type)
	}
	addTo(object) {
		if (this.label) {
			object.add(this.group)
		}
	}

	compute_plane(plane) {
		this.primary = new THREE.Vector3(plane.primary.x, plane.primary.y, plane.primary.z)
		this.secondary = new THREE.Vector3(plane.secondary.x, plane.secondary.y, plane.secondary.z)
		this.tertiary = new THREE.Vector3(plane.tertiary.x, plane.tertiary.y, plane.tertiary.z)

		// we need to rotate the text properly
		const m = new THREE.Matrix4()
		m.makeBasis(this.primary, this.secondary, this.tertiary)
		this.ea = new THREE.Euler(0, 0, 0, 'XYZ')
		this.ea.setFromRotationMatrix(m, 'XYZ')

		this.label = new Text()
		this.label.fontSize = 0.05
		this.label.anchorX = 'center'
		this.label.anchorY = 'middle'
		this.label.depthOffset = -1
		this.label.rotation.x = this.ea.x
		this.label.rotation.y = this.ea.y
		this.label.rotation.z = this.ea.z

		this.group = new THREE.Group()
	}
}

export { Constraint }

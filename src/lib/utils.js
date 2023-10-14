import * as THREE from 'three'

const ARC_TOLERANCE = 0.0001
const CIRCLE_TOLERANCE = 0.0001
const SKETCH_LINE_WIDTH = 1.0

const circleToPoints = (center_point, radius) => {
	const tolerance = ARC_TOLERANCE // in meters
	const k = tolerance / radius
	const n = Math.ceil(Math.PI / Math.sqrt(2 * k))
	const segment_angle = (2 * Math.PI) / n

	const shape_points = []

	for (let i = 1; i <= n; i++) {
		let theta = ((2 * Math.PI) / n) * i
		let x_component = radius * Math.cos(theta)
		let y_component = radius * Math.sin(theta)
		let point = new THREE.Vector2(x_component, y_component).add(center_point)
		shape_points.push(point)
	}
	return shape_points
}

const arcToPoints = (center_point, start_point, end_point, clockwise) => {
	// these points are THREE.Vector2's
	const tolerance = ARC_TOLERANCE // in meters
	const radius = center_point.distanceTo(start_point)
	const k = tolerance / radius
	let n = Math.ceil(Math.PI / Math.sqrt(2 * k))
	const segment_angle = (2 * Math.PI) / n
	const segment_length = radius * segment_angle
	if (clockwise) {
		n = -n
	}

	let start_angle = Math.atan2(start_point.y - center_point.y, start_point.x - center_point.x)

	const shape_points = []
	shape_points.push(start_point)

	for (let i = 1; i <= Math.abs(n); i++) {
		let theta = ((2 * Math.PI) / n) * i + start_angle
		let x_component = radius * Math.cos(theta)
		let y_component = radius * Math.sin(theta)
		let point = new THREE.Vector2(x_component, y_component).add(center_point)
		shape_points.push(point)

		let distance_to_end = point.distanceTo(end_point)
		if (distance_to_end <= segment_length) {
			shape_points.push(end_point)
			break
		}
	}
	return shape_points
}

export { circleToPoints, arcToPoints, CIRCLE_TOLERANCE, ARC_TOLERANCE, SKETCH_LINE_WIDTH }

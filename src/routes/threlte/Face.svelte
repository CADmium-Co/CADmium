<script>
	import { T } from '@threlte/core'
	import { Vector2, Shape, MeshStandardMaterial, DoubleSide, ShapeGeometry } from 'three'
	import { flatten, circleToPoints, promoteTo3, arcToPoints } from './projectUtils'

	export let face
	export let id
	export let pointsById

	// a face has an exterior and holes.
	// exterior is a wire, and holes is an array of wires.
	// a wire contains either .Segments or .Circle
	// If a wire has .Segments, it is an array of segments
	// each segment is an object with a field called 'type'
	// if 'type' is 'Line' then there is also .start and .end which are point IDs
	// if 'type' is 'Arc' then there is also .center, .start, and .end which are point IDs and .clockwise which is boolean
	// If a wire has .Circle is an object with:
	// .center which is a point ID, .radius which is a float, and .top which is a point ID
	// holes is an array of wires

	function writeWireToShape(wire, shape) {
		if (wire.Circle) {
			let circle = wire.Circle
			let center = pointsById[circle.center]
			let radius = circle.radius
			let points = circleToPoints(new Vector2(center.twoD.x, center.twoD.y), radius)
			shape.setFromPoints(points)
		} else if (wire.Segments) {
			let points = []
			for (let segment of wire.Segments) {
				if (segment.type === 'Line') {
					let start = pointsById[segment.start]
					let end = pointsById[segment.end]

					if (points.length === 0) {
						points.push(new Vector2(start.twoD.x, start.twoD.y))
					}
					points.push(new Vector2(end.twoD.x, end.twoD.y))
				} else if (segment.type === 'Arc') {
					let center = pointsById[segment.center]
					let start = pointsById[segment.start]
					let end = pointsById[segment.end]

					let arcPoints = arcToPoints(
						new Vector2(center.twoD.x, center.twoD.y),
						new Vector2(start.twoD.x, start.twoD.y),
						new Vector2(end.twoD.x, end.twoD.y),
						segment.clockwise
					)

					if (points.length !== 0) {
						arcPoints.shift()
					}
					points.push(...arcPoints)
				}
			}
			if (points.length > 0) {
				shape.setFromPoints(points)
			}
		}

		// if (shapePoints.length > 0) {
		// 	shape.moveTo(shapePoints[0][0], shapePoints[0][1])
		// 	for (let i = 1; i < shapePoints.length; i++) {
		// 		shape.lineTo(shapePoints[i][0], shapePoints[i][1])
		// 	}
		// }
	}

	const shape = new Shape()
	let exterior = face.exterior
	writeWireToShape(exterior, shape)
	const geometry = new ShapeGeometry(shape)

	const material = new MeshStandardMaterial({
		color: '#525252',
		side: DoubleSide,
		metalness: 0.0,
		transparent: true,
		opacity: 0.1,
		depthWrite: false,
		depthTest: true,
		wireframe: false,
		polygonOffset: true,
		polygonOffsetFactor: -4
	})
</script>

<T.Mesh {geometry} {material} />

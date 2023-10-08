// place files you want to import through the `$lib` alias in this folder.
import * as THREE from 'three'
import { TrackballControls } from 'three/addons/controls/TrackballControls.js'
// import CameraControls from 'camera-controls';
// CameraControls.install({ THREE: THREE });
import { Text } from 'troika-three-text'
import { Line2 } from 'three/addons/lines/Line2.js'
import { LineMaterial } from 'three/addons/lines/LineMaterial.js'
import { LineGeometry } from 'three/addons/lines/LineGeometry.js'

let camera, scene, renderer, controls
const planes = {}
const points = {}
const sketches = {}

const raycaster = new THREE.Raycaster()
const pointer = new THREE.Vector2(-1.0, -1.0)
const last_click = new THREE.Vector2(-1.0, -1.0)

let element

let selectable = []
let selected = []
let moving_camera = false

const ARC_TOLERANCE = 0.0001
const CIRCLE_TOLERANCE = 0.0001

const onPointerMove = (event) => {
	pointer.x = ((event.offsetX * window.devicePixelRatio) / element.width) * 2 - 1
	pointer.y = -((event.offsetY * window.devicePixelRatio) / element.height) * 2 + 1
}

const onPointerClick = (event) => {
	last_click.x = (event.offsetX / element.width) * 2 - 1
	last_click.y = -(event.offsetY / element.height) * 2 + 1
	console.log('Clicked!')

	raycaster.setFromCamera(pointer, camera)
	if (selectable.includes('planes')) {
		let just_meshes = Object.values(planes).map((plane) => plane.mesh)
		const intersections = raycaster.intersectObjects(just_meshes)
		if (intersections.length > 0) {
			let first_intersection = intersections[0]
			let plane_name = first_intersection.object.name
			let plane = planes[plane_name]
			plane.setSelectionStatus('selected')
			selected.push({ type: 'plane', name: plane_name, object: plane })
		}
	}
}

class Sketch {
	constructor(name, real_sketch, real_plane) {
		this.name = name
		this.real_plane = real_plane

		this.points = real_sketch.points
		this.line_segments = real_sketch.line_segments
		this.circles = real_sketch.circles
		this.arcs = real_sketch.arcs
		console.log('A whole new sketch!', real_sketch)
		this.group = new THREE.Group()
		for (let [point_id, point] of Object.entries(this.points)) {
			let point_2d = real_sketch.points_2d[point_id]
			let newPoint = new Point(point_id, point, point_2d, (parent = name))
			let extendedKey = `${name}:${point_id}`
			points[extendedKey] = newPoint
			if (point.hidden) {
				continue
			}
			newPoint.addTo(this.group)
		}

		for (let [line_segment_id, line_segment] of Object.entries(this.line_segments)) {
			let newLineSegment = new LineSegment(line_segment_id, line_segment, name)
			newLineSegment.addTo(this.group)
		}

		for (let [circle_id, circle] of Object.entries(this.circles)) {
			let newCircle = new Circle(circle_id, circle, this.real_plane, name)
			newCircle.addTo(this.group)
		}

		for (let [arc_id, arc] of Object.entries(this.arcs)) {
			let newArc = new Arc(arc_id, arc, this.real_plane, name)
			newArc.addTo(this.group)
		}

		for (let face of real_sketch.faces) {
			let newFace = new Face(face, this.real_plane, name)
			newFace.addTo(this.group)
		}
	}

	addTo(object) {
		object.add(this.group)
	}
}

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
		material.depthTest = false
		const mesh = new THREE.Points(geom, material)
		this.mesh = mesh
		this.mesh.renderOrder = 2
	}

	addTo(object) {
		object.add(this.mesh)
	}
}

class LineSegment {
	constructor(name, { start, end }, parent) {
		this.name = name
		this.start = start
		this.end = end
		this.parent = parent

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

		this.defaultMaterial = new LineMaterial({
			color: '#000000',
			linewidth: (this.lineWidth = 2.0 * window.devicePixelRatio * window.devicePixelRatio),
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

class Circle {
	constructor(name, { center, radius }, real_plane, parent) {
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
			color: '#000000',
			linewidth: 5.0,
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

class Face {
	constructor(face, real_plane, parent) {
		this.real_plane = real_plane

		const shape = new THREE.Shape()

		let exterior = face.exterior
		console.log('ext', exterior)
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

class Arc {
	constructor(name, { center, start, end, clockwise }, real_plane) {
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
			linewidth: (this.lineWidth = 2.0 * window.devicePixelRatio * window.devicePixelRatio),
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

class Plane {
	constructor(name, { plane, width, height }) {
		let { origin, primary, secondary, tertiary } = plane
		this.origin = origin
		this.primary = primary
		this.secondary = secondary
		this.tertiary = tertiary
		this.width = width
		this.height = height
		this.name = name

		this.fillColor = '#525292'
		this.strokeColor = '#42a7eb'
		this.lineWidth = 2.0 * window.devicePixelRatio * window.devicePixelRatio
		this.material = new THREE.MeshStandardMaterial({
			color: this.fillColor,
			side: THREE.DoubleSide,
			metalness: 0.0,
			transparent: true,
			opacity: 0.05,
			depthWrite: false
		})
		this.lineMaterial = new LineMaterial({
			color: this.strokeColor,
			linewidth: this.lineWidth,
			depthTest: false,
			transparent: true,
			dashed: false,
			resolution: new THREE.Vector2(
				element.width * window.devicePixelRatio,
				element.height * window.devicePixelRatio
			)
		})

		this.mouseOverFillColor = '#525292'
		this.mouseOverStrokeColor = '#ffa500'
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
			depthTest: false,
			transparent: true,
			dashed: false,
			resolution: new THREE.Vector2(
				element.width * window.devicePixelRatio,
				element.height * window.devicePixelRatio
			)
		})

		this.selectedFillColor = '#525292'
		this.selectedStrokeColor = '#ff0000'
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
			depthTest: false,
			transparent: true,
			dashed: false,
			resolution: new THREE.Vector2(
				element.width * window.devicePixelRatio,
				element.height * window.devicePixelRatio
			)
		})

		this.selectionStatus = 'unselected' // could also be 'mouseOver' or 'selected'

		origin = new THREE.Vector3(origin.x, origin.y, origin.z)
		primary = new THREE.Vector3(primary.x, primary.y, primary.z)
		secondary = new THREE.Vector3(secondary.x, secondary.y, secondary.z)
		tertiary = new THREE.Vector3(tertiary.x, tertiary.y, tertiary.z)

		let half_width = width / 2
		let half_height = height / 2

		const upper_right = origin
			.clone()
			.addScaledVector(primary, half_width)
			.addScaledVector(secondary, half_height)
		const upper_left = origin
			.clone()
			.addScaledVector(primary, -half_width)
			.addScaledVector(secondary, half_height)
		const lower_right = origin
			.clone()
			.addScaledVector(primary, half_width)
			.addScaledVector(secondary, -half_height)
		const lower_left = origin
			.clone()
			.addScaledVector(primary, -half_width)
			.addScaledVector(secondary, -half_height)
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

		geometry.setAttribute('position', new THREE.BufferAttribute(vertices, 3))
		geometry.setAttribute('normal', new THREE.BufferAttribute(normals, 3))

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
		label.text = ' ' + name
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
		const ea = new THREE.Euler(0, 0, 0, 'XYZ')
		ea.setFromRotationMatrix(m, 'XYZ')
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

	setSelectionStatus(status) {
		if (status === 'unselected') {
			this.mesh.material.color.set(this.fillColor)
			this.line.material = this.lineMaterial
		} else if (status === 'mouseOver') {
			this.mesh.material.color.set(this.mouseOverFillColor)
			this.line.material = this.mouseOverLineMaterial
		} else if (status === 'selected') {
			this.mesh.material.color.set(this.selectedFillColor)
			this.line.material = this.selectedLineMaterial
		} else {
			throw new Error('Invalid selection status: ', status)
		}
		this.selectionStatus = status
	}
}

export const createScene = (el) => {
	element = el
	const clock = new THREE.Clock()
	scene = new THREE.Scene()

	const { width, height } = el.getBoundingClientRect()
	const aspectRatio = width / height
	const worldWidth = 3
	const worldHeight = worldWidth / aspectRatio
	camera = new THREE.OrthographicCamera(
		worldWidth / -2,
		worldWidth / 2,
		worldHeight / 2,
		worldHeight / -2,
		0.1,
		1000
	)

	// camera.position.x = 7.8;
	// camera.position.y = -25.8;
	// camera.position.z = 8.55;
	camera.position.x = 16.8
	camera.position.y = -25.8
	camera.position.z = 20.55
	camera.up = new THREE.Vector3(0, 0, 1)
	camera.lookAt(0, 0, 0)

	// const axesHelper = new THREE.AxesHelper(5);
	// scene.add(axesHelper);

	// camera-controls
	// const cameraControls = new CameraControls(camera, el);

	// TrackballControls
	controls = new TrackballControls(camera, el)
	controls.rotateSpeed = 3.0

	const directionalLight = new THREE.DirectionalLight(0x9090aa)
	directionalLight.position.set(-10, 10, -10).normalize()
	scene.add(directionalLight)

	const hemisphereLight = new THREE.HemisphereLight(0xffffff, 0x444444)
	hemisphereLight.position.set(1, 1, 1)
	scene.add(hemisphereLight)
	let count = 0

	const handleMouseover = () => {
		// First just deselect everything. Start by deselecting all planes
		for (let [plane_name, plane] of Object.entries(planes)) {
			if (plane.selectionStatus === 'mouseOver') {
				plane.setSelectionStatus('unselected')
			}
		}
		// then deselect all solids, all lines, all points, etc

		// Now check for intersections but only for things that should
		// be selectable right now
		raycaster.setFromCamera(pointer, camera)
		if (selectable.includes('planes')) {
			let just_meshes = Object.values(planes).map((plane) => plane.mesh)
			const intersections = raycaster.intersectObjects(just_meshes)
			if (intersections.length > 0) {
				let first_intersection = intersections[0]
				let plane_name = first_intersection.object.name
				let plane = planes[plane_name]
				plane.setSelectionStatus('mouseOver')
			}
		}
	}

	const render = () => {
		const delta = clock.getDelta()
		controls.update(delta)
		// const hasControlsUpdated = cameraControls.update(delta);

		requestAnimationFrame(render)

		// required if controls.enableDamping or controls.autoRotate are set to true
		// controls.update();

		handleMouseover()

		count += 1
		if (count === 60) {
			count = 0
		}

		// you can skip this condition to render though
		// if (hasControlsUpdated) {
		renderer.render(scene, camera)
		// }
	}

	const resize = () => {
		const { width, height } = el.getBoundingClientRect()
		renderer.setSize(width, height)
		camera.aspect = width / height
		camera.updateProjectionMatrix()
	}

	const getStarted = (el) => {
		renderer = new THREE.WebGLRenderer({ antialias: true, canvas: el })
		renderer.setPixelRatio(window.devicePixelRatio)
		renderer.setClearColor('#F8F8F8')
		resize()
		render()
	}

	window.addEventListener('resize', resize)

	el.addEventListener('pointermove', onPointerMove)
	el.addEventListener('click', onPointerClick)

	getStarted(el)
}

export const setRealization = (realization) => {
	// for now just delete every old plane and create a new one each time
	// in the future, we can make this more efficient by updating the existing planes
	// for (const [key, value] of Object.entries(planes)) {
	//     scene.remove(value);
	// }

	console.log('Realization: ', realization)

	// create a new plane for each plane in the realization
	for (const [name, plane] of Object.entries(realization.planes)) {
		planes[name] = new Plane(name, plane)
		planes[name].addTo(scene)
	}

	// create a new point for each point in the realization
	console.log('P2D', realization.points_2d)
	for (const [name, point] of Object.entries(realization.points)) {
		// let p2d = realization.points[name]
		points[name] = new Point(name, point, {}, (parent = null))
		points[name].addTo(scene)
	}

	for (const [name, sketch] of Object.entries(realization.sketches)) {
		let unsplit = sketch[0]
		let split = sketch[1]
		// console.log('Name', name, 'sketch', split.points, unsplit.points)

		let plane_name = sketch[0].plane_name
		let real_plane = realization.planes[plane_name]
		sketches[name] = new Sketch(name, split, real_plane)
		sketches[name].addTo(scene)
	}
}

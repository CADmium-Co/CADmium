// place files you want to import through the `$lib` alias in this folder.
import * as THREE from 'three'
import { TrackballControls } from 'three/addons/controls/TrackballControls.js'
// import CameraControls from 'camera-controls';
// CameraControls.install({ THREE: THREE });
import { Text } from 'troika-three-text'

import { Point } from './point.js'
import { Plane } from './plane.js'
import { Sketch } from './sketch.js'

let camera, scene, renderer, controls
const sketches = {}
const planes = {}
const points = {}
const circles = {}
const arcs = {}
const faces = {}
const lines = {}

const raycaster = new THREE.Raycaster()
const pointer = new THREE.Vector2(-1.0, -1.0)
const last_click = new THREE.Vector2(-1.0, -1.0)

let element

let selectable = []
let selected = []
let moving_camera = false

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

	camera.position.x = 16.8
	camera.position.y = -25.8
	camera.position.z = 20.55
	camera.up = new THREE.Vector3(0, 0, 1)
	camera.lookAt(0, 0, 0)

	// const axesHelper = new THREE.AxesHelper(5)
	// scene.add(axesHelper)

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
	if (!element) {
		console.log('element is not set!', element, renderer)
		return
	}

	// console.log('Inside Set Realization: ', realization)
	// console.log('Points: ', realization.sketches['Sketch 1'][0])

	// for now just delete every old plane and create a new one each time
	// in the future, we can make this more efficient by updating the existing planes
	for (const [name, value] of Object.entries(planes)) {
		planes[name].removeFrom(scene)
	}
	for (const [name, value] of Object.entries(points)) {
		points[name].removeFrom(scene)
	}
	for (const [name, value] of Object.entries(sketches)) {
		sketches[name].removeFrom(scene)
	}

	// create a new plane for each plane in the realization
	for (const [name, plane] of Object.entries(realization.planes)) {
		planes[name] = new Plane(name, plane, element)
		planes[name].addTo(scene)
	}
	for (const [name, point] of Object.entries(realization.points)) {
		points[name] = new Point(name, point, {}, (parent = null))
		points[name].addTo(scene)
	}
	for (const [name, sketch] of Object.entries(realization.sketches)) {
		let unsplit = sketch[0]
		let split = sketch[1]
		let plane_name = sketch[0].plane_name
		let real_plane = realization.planes[plane_name]
		sketches[name] = new Sketch(
			name,
			unsplit,
			real_plane,
			points,
			lines,
			arcs,
			circles,
			faces,
			element
		)
		sketches[name].addTo(scene)
	}
}

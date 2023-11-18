// place files you want to import through the `$lib` alias in this folder.
import * as THREE from 'three'
import { TrackballControls } from 'three/addons/controls/TrackballControls.js'
// import CameraControls from 'camera-controls';
// CameraControls.install({ THREE: THREE });

import gsap from 'gsap'

import { Point } from './point.js'
import { Plane } from './plane.js'
import { Sketch } from './sketch.js'
import { Solid } from './solid.js'

import { EffectComposer } from 'three/addons/postprocessing/EffectComposer.js'
import { RenderPass } from 'three/addons/postprocessing/RenderPass.js'
import { ShaderPass } from 'three/addons/postprocessing/ShaderPass.js'
import { OutlinePass } from 'three/addons/postprocessing/OutlinePass.js'
import { OutputPass } from 'three/addons/postprocessing/OutputPass.js'
import { FXAAShader } from 'three/addons/shaders/FXAAShader.js'

let camera, scene, renderer, controls, outlinePass
const sketches = {}
const planes = {}
const points = {}
const circles = {}
const arcs = {}
const faces = {}
const lines = {}
const solids = {}

const raycaster = new THREE.Raycaster()
const pointer = new THREE.Vector2(-1.0, -1.0)
const last_click = new THREE.Vector2(-1.0, -1.0)

let element

let selected = []
let moving_camera = false
let looking_for = []
let onFound = () => {}

const onPointerMove = (event) => {
	pointer.x = ((event.offsetX * window.devicePixelRatio) / element.width) * 2 - 1
	pointer.y = -((event.offsetY * window.devicePixelRatio) / element.height) * 2 + 1
}

export const setOnFound = (foundHandler) => {
	onFound = foundHandler
	// console.log('Have a new on found handler')
}

const onPointerClick = (event) => {
	event.preventDefault()
	event.stopImmediatePropagation()
	last_click.x = (event.offsetX / element.width) * 2 - 1
	last_click.y = -(event.offsetY / element.height) * 2 + 1
	// console.log('Clicked! Could be looking for anything:', looking_for)

	raycaster.setFromCamera(pointer, camera)
	if (looking_for.includes('plane')) {
		console.log('okay looking for a plane...')
		let just_meshes = Object.values(planes).map((plane) => plane.mesh)
		const intersections = raycaster.intersectObjects(just_meshes)
		if (intersections.length > 0) {
			console.log('OH I FOUND A PLANE')
			let first_intersection = intersections[0]
			let plane_name = first_intersection.object.name
			let plane = planes[plane_name]
			plane.setSelectionStatus('selected')
			selected.push({ type: 'plane', name: plane_name, object: plane })
			onFound({ type: 'plane', name: plane_name, object: plane })
		}
	}

	if (looking_for.includes('line')) {
		console.log('okay looking for a line...')
		let just_meshes = Object.values(lines).map((line) => line.mesh)
		const intersections = raycaster.intersectObjects(just_meshes)
		if (intersections.length > 0) {
			console.log('OH I FOUND A LINE')
			let first_intersection = intersections[0]
			let line_name = first_intersection.object.name
			let line = lines[line_name]
			line.setSelectionStatus('selected')
			selected.push({ type: 'line', name: line_name, object: line })
			// onFound({ type: 'line', name: line_name, object: line })
		}
	}
}

export const set_looking_for = (lf) => {
	looking_for = lf
}

export const setCameraViewPlane = (plane) => {
	const secondary = plane.data.plane.secondary
	let normal = plane.data.plane.tertiary
	normal = new THREE.Vector3(normal.x, normal.y, normal.z)
	normal.multiplyScalar(20)

	gsap.to(camera.position, {
		x: normal.x,
		y: normal.y,
		z: normal.z,
		duration: 1,
		onUpdate: function () {
			camera.lookAt(0, 0, 0)
		}
	})

	gsap.to(camera.up, {
		x: secondary.x,
		y: secondary.y,
		z: secondary.z,
		duration: 1
	})
}

const handleMouseover = () => {
	// First just deselect everything. Start by deselecting all planes
	for (let [plane_name, plane] of Object.entries(planes)) {
		if (plane.selectionStatus === 'mouseOver') {
			plane.setSelectionStatus('unselected')
		}
	}
	// then deselect all solids, all lines, all points, etc
	for (let [line_name, line] of Object.entries(lines)) {
		if (line.selectionStatus === 'mouseOver') {
			line.setSelectionStatus('unselected')
		}
	}

	// Now check for intersections but only for things that we are looking_for
	raycaster.setFromCamera(pointer, camera)
	if (looking_for.includes('plane')) {
		let just_meshes = Object.values(planes).map((plane) => plane.mesh)
		const intersections = raycaster.intersectObjects(just_meshes)
		if (intersections.length > 0) {
			let first_intersection = intersections[0]
			let plane_name = first_intersection.object.name
			let plane = planes[plane_name]
			plane.setSelectionStatus('mouseOver')
		}
	}
	if (looking_for.includes('line')) {
		let just_meshes = Object.values(lines).map((line) => line.mesh)
		const intersections = raycaster.intersectObjects(just_meshes)
		if (intersections.length > 0) {
			let first_intersection = intersections[0]
			// console.log('first_intersection: ', first_intersection)
			let line_name = first_intersection.object.name
			// console.log('line name', line_name)
			let line = lines[line_name]
			// console.log('line', line)

			if (line.selectionStatus === 'selected') {
				return
			}

			line.setSelectionStatus('mouseOver')
		}
	}
}

export const createScene = (el) => {
	element = el
	const clock = new THREE.Clock()
	scene = new THREE.Scene()
	let composer

	const params = {
		edgeStrength: 10.0,
		edgeGlow: 0.0,
		edgeThickness: 3.0,
		pulsePeriod: 0,
		rotate: false,
		usePatternTexture: false,
		visibleEdgeColor: '#00a7ff',
		hiddenEdgeColor: '#00a7ff'
	}

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

	const ambientLight = new THREE.AmbientLight(0xb0b0b0) // soft white ambientLight
	scene.add(ambientLight)

	const directionalLight = new THREE.DirectionalLight(0x9090aa)
	directionalLight.position.set(-10, 10, -10).normalize()
	scene.add(directionalLight)

	const hemisphereLight = new THREE.HemisphereLight(0xffffff, 0x444444)
	hemisphereLight.position.set(1, 1, 1)
	scene.add(hemisphereLight)
	let count = 0

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

		composer.render()

		// you can skip this condition to render though
		// if (hasControlsUpdated) {
		// renderer.render(scene, camera)
		// }
	}

	const resize = () => {
		const { width, height } = el.getBoundingClientRect()
		renderer.setSize(width, height)
		camera.aspect = width / height
		camera.updateProjectionMatrix()
	}

	const getStarted = (el) => {
		const { width, height } = el.getBoundingClientRect()
		renderer = new THREE.WebGLRenderer({
			antialias: false,
			canvas: el
			// logarithmicDepthBuffer: true
		})
		renderer.setPixelRatio(window.devicePixelRatio)
		renderer.setSize(width, height)
		renderer.setClearColor('#F8F8F8')

		composer = new EffectComposer(renderer)
		const renderPass = new RenderPass(scene, camera)
		composer.addPass(renderPass)

		// outlinePass = new OutlinePass(new THREE.Vector2(width / 2, height * 2), scene, camera)
		outlinePass = new OutlinePass(undefined, scene, camera)
		composer.addPass(outlinePass)
		outlinePass.edgeStrength = Number(params.edgeStrength)
		outlinePass.edgeGlow = Number(params.edgeGlow)
		outlinePass.edgeThickness = Number(params.edgeThickness)
		outlinePass.pulsePeriod = Number(params.pulsePeriod)
		outlinePass.rotate = Boolean(params.rotate)
		outlinePass.usePatternTexture = Boolean(params.usePatternTexture)
		outlinePass.visibleEdgeColor.set(params.visibleEdgeColor)
		outlinePass.hiddenEdgeColor.set(params.hiddenEdgeColor)
		outlinePass.overlayMaterial.blending = THREE.SubtractiveBlending

		const effectFXAA = new ShaderPass(FXAAShader)
		effectFXAA.uniforms['resolution'].value.set(
			1 / width / window.devicePixelRatio,
			1 / height / window.devicePixelRatio
		)
		composer.addPass(effectFXAA)

		const outputPass = new OutputPass()
		composer.addPass(outputPass)

		resize()
		render()
	}

	window.addEventListener('resize', resize)

	el.addEventListener('pointermove', onPointerMove)
	el.addEventListener('pointerdown', (event) => {
		event.preventDefault()
		// event.stopImmediatePropagation()
	})
	el.addEventListener('click', onPointerClick)

	getStarted(el)
}

export const setRealization = (realization, sketch_being_edited) => {
	console.log('setting realization')
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
	for (const [name, value] of Object.entries(solids)) {
		solids[name].removeFrom(scene)
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
			split,
			real_plane,
			points,
			lines,
			arcs,
			circles,
			faces,
			element
		)
		sketches[name].addTo(scene)

		if (sketch_being_edited === name) {
			sketches[name].setEditing(true)
		}
	}

	for (const [name, solid] of Object.entries(realization.solids)) {
		solids[name] = new Solid(name, solid, element)
		solids[name].addTo(scene)
	}
}

export const setOutlined = (outlined) => {
	const to_be_outlined = outlined.map((solid) => solids[solid].group)
	if (outlinePass) {
		outlinePass.selectedObjects = to_be_outlined
	}
}

<script>
	import { useThrelte } from '@threlte/core'
	import { LineMaterial } from 'three/addons/lines/LineMaterial.js'
	import { LineGeometry } from 'three/addons/lines/LineGeometry.js'
	import { T } from '@threlte/core'
	import { Vector2, Vector3 } from 'three'
	import { CIRCLE_TOLERANCE } from './projectUtils.js'

	export let id
	export let center
	export let radius
	export let plane

	const { size, dpr } = useThrelte()

	let o = new Vector3(plane.origin.x, plane.origin.y, plane.origin.z)
	let x = new Vector3(plane.primary.x, plane.primary.y, plane.primary.z)
	let y = new Vector3(plane.secondary.x, plane.secondary.y, plane.secondary.z)
	let c = new Vector3(center.threeD.x, center.threeD.y, center.threeD.z)

	$: lineMaterial = new LineMaterial({
		color: '#000000',
		linewidth: 1.0 * $dpr,
		depthTest: false,
		transparent: true,
		dashed: true,
		dashSize: 0.1,
		gapSize: 0.1,
		dashScale: 3,
		resolution: new Vector2($size.width * $dpr, $size.height * $dpr)
	})

	// see https://math.stackexchange.com/a/4132095/816177
	const tolerance = CIRCLE_TOLERANCE // in meters
	const k = tolerance / radius
	// more precise but slower to calculate:
	// const n = Math.ceil(Math.PI / Math.acos(1 - k))
	// faster to calculate, at most only overestimates by 1:
	const n = Math.ceil(Math.PI / Math.sqrt(2 * k))

	const lineVertices = []
	for (let i = 0; i <= n; i++) {
		let theta = ((2 * Math.PI) / n) * i
		let xComponent = x.clone().multiplyScalar(radius * Math.cos(theta))
		let yComponent = y.clone().multiplyScalar(radius * Math.sin(theta))
		let point = o.clone().add(xComponent).add(yComponent)
		point.add(c)
		lineVertices.push(point.x, point.y, point.z)
	}
	const lineGeometry = new LineGeometry()
	lineGeometry.setPositions(lineVertices)
</script>

<T.Line2
	geometry={lineGeometry}
	material={lineMaterial}
	on:create={({ ref }) => {
		ref.computeLineDistances()
	}}
/>

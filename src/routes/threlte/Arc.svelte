<script>
	import { useThrelte } from '@threlte/core'
	import { LineMaterial } from 'three/addons/lines/LineMaterial.js'
	import { LineGeometry } from 'three/addons/lines/LineGeometry.js'
	import { T } from '@threlte/core'
	import { Vector2, Vector3 } from 'three'
	import { CIRCLE_TOLERANCE } from './projectUtils.js'

	// export let id
	export let center
	export let start
	export let end
	export let plane

	const { size, dpr } = useThrelte()

	let o = new Vector3(plane.origin.x, plane.origin.y, plane.origin.z)
	let x = new Vector3(plane.primary.x, plane.primary.y, plane.primary.z)
	let y = new Vector3(plane.secondary.x, plane.secondary.y, plane.secondary.z)
	let c = new Vector3(center.threeD.x, center.threeD.y, center.threeD.z)
	let s = new Vector3(start.threeD.x, start.threeD.y, start.threeD.z)
	let e = new Vector3(end.threeD.x, end.threeD.y, end.threeD.z)
	const radius = s.distanceTo(c)

	$: dottedLineMaterial = new LineMaterial({
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

	$: solidLineMaterial = new LineMaterial({
		color: '#000000',
		linewidth: 1.5 * $dpr,
		depthTest: true,
		transparent: true,
		dashed: false,
		resolution: new Vector2($size.width * $dpr, $size.height * $dpr)
	})

	// see https://math.stackexchange.com/a/4132095/816177
	const tolerance = CIRCLE_TOLERANCE // in meters
	const k = tolerance / radius
	// more precise but slower to calculate:
	// const n = Math.ceil(Math.PI / Math.acos(1 - k))
	// faster to calculate, at most only overestimates by 1:
	const n = Math.ceil(Math.PI / Math.sqrt(2 * k))
	const segmentAngle = (2 * Math.PI) / n
	const segmentLength = radius * segmentAngle
	const startAngle = Math.atan2(start.twoD.y - center.twoD.y, start.twoD.x - center.twoD.x)

	// console.log('arc:', n, center, start, end)
	const lineVertices = []
	lineVertices.push(s.x, s.y, s.z)
	for (let i = 1; i <= n; i++) {
		let theta = ((2 * Math.PI) / n) * i + startAngle
		let xComponent = x.clone().multiplyScalar(radius * Math.cos(theta))
		let yComponent = y.clone().multiplyScalar(radius * Math.sin(theta))
		let point = o.clone().add(xComponent).add(yComponent)
		point.add(c)
		lineVertices.push(point.x, point.y, point.z)

		let distanceToEnd = point.distanceTo(e)
		if (distanceToEnd <= segmentLength) {
			lineVertices.push(e.x, e.y, e.z)
			break
		}
	}
	const lineGeometry = new LineGeometry()
	lineGeometry.setPositions(lineVertices)
</script>

<T.Group>
	<T.Line2
		geometry={lineGeometry}
		material={dottedLineMaterial}
		on:create={({ ref }) => {
			ref.computeLineDistances()
		}}
	/>
	<T.Line2
		geometry={lineGeometry}
		material={solidLineMaterial}
		on:create={({ ref }) => {
			ref.computeLineDistances()
		}}
	/>
</T.Group>

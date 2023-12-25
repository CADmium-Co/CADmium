<script>
	import { useThrelte } from '@threlte/core'
	import { LineMaterial } from 'three/addons/lines/LineMaterial.js'
	import { LineGeometry } from 'three/addons/lines/LineGeometry.js'
	import { T } from '@threlte/core'
	import { Vector2, Vector3 } from 'three'
	import { flatten, circleToPoints, promoteTo3 } from './projectUtils'

	export let id
	export let center
	export let radius

	const { size, dpr } = useThrelte()

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

	let points = flatten(promoteTo3(circleToPoints(center.twoD, radius)))

	const lineGeometry = new LineGeometry()
	lineGeometry.setPositions(points)
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

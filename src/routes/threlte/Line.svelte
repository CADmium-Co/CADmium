<script>
	import { useThrelte } from '@threlte/core'
	import { LineMaterial } from 'three/addons/lines/LineMaterial.js'
	import { LineGeometry } from 'three/addons/lines/LineGeometry.js'
	import { Vector2 } from 'three'
	import { T } from '@threlte/core'
	import { flatten, promoteTo3 } from './projectUtils'
	import { currentlyMousedOver, sketchTool } from './stores'

	export let start
	export let end
	export let id

	const { size, dpr } = useThrelte()

	$: dashedLineMaterial = new LineMaterial({
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

	$: collisionLineMaterial = new LineMaterial({
		color: '#FFFFFF',
		linewidth: 10.0 * $dpr,
		depthTest: false,
		depthWrite: false,
		transparent: true,
		opacity: 0,
		dashed: false,
		resolution: new Vector2($size.width * $dpr, $size.height * $dpr)
	})

	const points = flatten(
		promoteTo3([new Vector2(start.twoD.x, start.twoD.y), new Vector2(end.twoD.x, end.twoD.y)])
	)

	const lineGeometry = new LineGeometry()
	lineGeometry.setPositions(points)
</script>

<T.Group>
	<T.Line2
		geometry={lineGeometry}
		material={dashedLineMaterial}
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
	<T.Line2
		geometry={lineGeometry}
		material={collisionLineMaterial}
		on:create={({ ref }) => {
			ref.computeLineDistances()
		}}
		on:pointerover={() => {
			if ($sketchTool === 'select') {
				solidLineMaterial.color.set('#ffaa00')
				dashedLineMaterial.color.set('#ffaa00')
				$currentlyMousedOver = [...$currentlyMousedOver, { type: 'line', id: id }]
			}
		}}
		on:pointerout={() => {
			if ($sketchTool === 'select') {
				solidLineMaterial.color.set('#000000')
				dashedLineMaterial.color.set('#000000')
				$currentlyMousedOver = $currentlyMousedOver.filter(
					(item) => !(item.id === id && item.type === 'line')
				)
			}
		}}
	/>
</T.Group>

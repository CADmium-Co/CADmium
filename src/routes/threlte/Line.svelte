<script>
	import { useThrelte } from '@threlte/core'
	import { LineMaterial } from 'three/addons/lines/LineMaterial.js'
	import { LineGeometry } from 'three/addons/lines/LineGeometry.js'
	import { Vector2 } from 'three'
	import { T } from '@threlte/core'
	import { flatten, promoteTo3 } from './projectUtils'
	import { currentlySelected, currentlyMousedOver, sketchTool } from './stores'

	export let start
	export let end
	export let id
	const type = 'line'

	const { size, dpr } = useThrelte()

	let hovered = false
	$: selected = $currentlySelected.some((e) => e.id === id && e.type === type) ? true : false

	// TODO: why does every line have its own copy of all these materials? They should share a single copy
	$: dashedLineMaterial = new LineMaterial({
		color: hovered ? '#ffaa00' : '#000000',
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

	$: solidHoveredMaterial = new LineMaterial({
		color: '#88aa00',
		linewidth: 5.5 * $dpr,
		depthTest: true,
		transparent: true,
		dashed: false,
		resolution: new Vector2($size.width * $dpr, $size.height * $dpr)
	})

	$: solidSelectedMaterial = new LineMaterial({
		color: '#ffaa00',
		linewidth: 5.5 * $dpr,
		depthTest: true,
		transparent: true,
		dashed: false,
		resolution: new Vector2($size.width * $dpr, $size.height * $dpr)
	})

	$: collisionLineMaterial = new LineMaterial({
		color: '#FFFFFF',
		linewidth: 12.0 * $dpr,
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
		material={hovered ? solidHoveredMaterial : selected ? solidSelectedMaterial : solidLineMaterial}
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
				hovered = true
				$currentlyMousedOver = [...$currentlyMousedOver, { type: 'line', id: id }]
			}
		}}
		on:pointerout={() => {
			if ($sketchTool === 'select') {
				hovered = false
				$currentlyMousedOver = $currentlyMousedOver.filter(
					(item) => !(item.id === id && item.type === 'line')
				)
			}
		}}
	/>
</T.Group>

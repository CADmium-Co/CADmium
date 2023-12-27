<script>
	import { LineGeometry } from 'three/addons/lines/LineGeometry.js'
	import { T } from '@threlte/core'
	import { flatten, circleToPoints, promoteTo3 } from './projectUtils'
	import { currentlySelected, currentlyMousedOver, sketchTool } from './stores'

	export let id, center, radius

	export let dashedLineMaterial,
		dashedHoveredMaterial,
		solidLineMaterial,
		solidHoveredMaterial,
		solidSelectedMaterial,
		collisionLineMaterial

	const type = 'circle'

	let hovered = false
	$: selected = $currentlySelected.some((e) => e.id === id && e.type === type) ? true : false

	let points = flatten(promoteTo3(circleToPoints(center.twoD, radius)))

	const lineGeometry = new LineGeometry()
	lineGeometry.setPositions(points)
</script>

<T.Group>
	<T.Line2
		geometry={lineGeometry}
		material={hovered ? dashedHoveredMaterial : dashedLineMaterial}
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
				$currentlyMousedOver = [...$currentlyMousedOver, { type: type, id: id }]
			}
		}}
		on:pointerout={() => {
			if ($sketchTool === 'select') {
				hovered = false
				$currentlyMousedOver = $currentlyMousedOver.filter(
					(item) => !(item.id === id && item.type === type)
				)
			}
		}}
	/>
</T.Group>

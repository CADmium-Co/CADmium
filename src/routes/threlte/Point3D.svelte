<script>
	import { LineGeometry } from 'three/addons/lines/LineGeometry.js'
	import { useTexture } from '@threlte/extras'
	import { BufferGeometry, Float32BufferAttribute, PointsMaterial, Vector2 } from 'three'
	import { currentlySelected, currentlyMousedOver, sketchTool } from './stores'
	import { flatten, promoteTo3 } from './projectUtils'
	import { T } from '@threlte/core'

	export let x, y, z, hidden, id
	export let isPreview = false

	export let collisionLineMaterial

	let source = '/actions/point_min.svg'
	let outlineSource = '/actions/point_outline.svg'

	const pointTexture = useTexture(source)
	const outlineTexture = useTexture(outlineSource)

	const type = 'point3D'

	let hovered = false
	$: selected = $currentlySelected.some((e) => e.id === id && e.type === type) ? true : false

	const delta = 0.0001
	const pointsH = flatten(promoteTo3([new Vector2(x - delta, y), new Vector2(x + delta, y)]))
	const pointsV = flatten(promoteTo3([new Vector2(x, y - delta), new Vector2(x, y + delta)]))

	const lineGeometryH = new LineGeometry()
	lineGeometryH.setPositions(pointsH)
	const lineGeometryV = new LineGeometry()
	lineGeometryV.setPositions(pointsV)

	let geom = new BufferGeometry()
	let vertices = new Float32Array([x, y, z])
	geom.setAttribute('position', new Float32BufferAttribute(vertices, 3))

	const validTools = ['select', 'line', 'circle', 'rectangle']
</script>

{#if !hidden}
	{#await pointTexture then pointImg}
		{#await outlineTexture then outlineImg}
			<T.Group
				on:pointerover={() => {
					if (isPreview) return
					if (validTools.includes($sketchTool)) {
						hovered = true
						$currentlyMousedOver = [
							...$currentlyMousedOver,
							{ type: type, id: id, x: x, y: y, z: z }
						]
					}
				}}
				on:pointerout={() => {
					if (isPreview) return
					if (validTools.includes($sketchTool)) {
						hovered = false
						$currentlyMousedOver = $currentlyMousedOver.filter(
							(item) => !(item.id === id && item.type === type)
						)
					}
				}}
			>
				<T.Line2
					geometry={lineGeometryH}
					material={collisionLineMaterial}
					on:create={({ ref }) => {
						ref.computeLineDistances()
					}}
				/>

				<T.Line2
					geometry={lineGeometryV}
					material={collisionLineMaterial}
					on:create={({ ref }) => {
						ref.computeLineDistances()
					}}
				/>
			</T.Group>
			<T.Points geometry={geom}>
				<T.PointsMaterial
					size={12}
					map={pointImg}
					renderOrder={-10}
					transparent={true}
					sizeAttenuation={false}
					depthTest={false}
					depthWrite={false}
				/>
			</T.Points>
			{#if hovered || selected}
				<T.Points geometry={geom}>
					<T.PointsMaterial
						size={18}
						map={outlineImg}
						renderOrder={-10}
						transparent={true}
						sizeAttenuation={false}
						depthTest={false}
						depthWrite={false}
					/>
				</T.Points>
			{/if}
		{/await}
	{/await}
{/if}

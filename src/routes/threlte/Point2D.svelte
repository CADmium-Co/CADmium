<script>
	import { useTexture } from '@threlte/extras'
	import { BufferGeometry, Float32BufferAttribute, PointsMaterial } from 'three'
	import { T } from '@threlte/core'

	export let x, y, hidden, id
	export let snappedTo = false

	let source = '/actions/simple_point_min.svg'
	let outlineSource = '/actions/point_outline.svg'

	const texture = useTexture(source)
	const outlineTexture = useTexture(outlineSource)

	let geom = new BufferGeometry()
	let vertices = new Float32Array([x, y, 0])
	geom.setAttribute('position', new Float32BufferAttribute(vertices, 3))
</script>

{#if !hidden}
	<T.Group>
		{#await texture then ordinaryPointMap}
			<T.Points geometry={geom}>
				<T.PointsMaterial
					size={6}
					map={ordinaryPointMap}
					renderOrder={-10}
					transparent={true}
					sizeAttenuation={false}
					depthTest={false}
					depthWrite={false}
				/>
			</T.Points>
		{/await}
		{#await outlineTexture then ordinaryPointMap}
			{#if snappedTo}
				<T.Points geometry={geom}>
					<T.PointsMaterial
						size={18}
						map={ordinaryPointMap}
						renderOrder={-10}
						transparent={true}
						sizeAttenuation={false}
						depthTest={false}
						depthWrite={false}
					/>
				</T.Points>
			{/if}
		{/await}
	</T.Group>
{/if}

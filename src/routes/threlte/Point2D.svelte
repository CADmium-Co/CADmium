<script>
	import { useTexture } from '@threlte/extras'
	import { BufferGeometry, Float32BufferAttribute } from 'three'
	import { T } from '@threlte/core'

	export let x
	export let y
	export let hidden
	export let snappedTo = false

	let source = '/actions/simple_point_min.svg'

	const texture = useTexture(source)

	let geom = new BufferGeometry()
	let vertices = new Float32Array([x, y, 0])
	geom.setAttribute('position', new Float32BufferAttribute(vertices, 3))
</script>

{#if !hidden}
	{#await texture then sprite}
		<T.Points geometry={geom}>
			<T.PointsMaterial
				size={snappedTo ? 16 : 6}
				map={sprite}
				renderOrder={-10}
				transparent={true}
				sizeAttenuation={false}
				depthTest={false}
				depthWrite={false}
			/>
		</T.Points>
	{/await}
{/if}

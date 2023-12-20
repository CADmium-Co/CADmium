<script>
	import { useTexture } from '@threlte/extras'
	import { BufferGeometry, Float32BufferAttribute } from 'three'
	import { T } from '@threlte/core'

	export let x
	export let y
	export let z
	export let hidden
	export let name

	let source = '/actions/point_min.svg'

	const texture = useTexture(source)

	let geom = new BufferGeometry()
	let vertices = new Float32Array([x, y, z])
	geom.setAttribute('position', new Float32BufferAttribute(vertices, 3))
</script>

{#if !hidden}
	{#await texture then sprite}
		<T.Points geometry={geom}>
			<T.PointsMaterial
				size={12}
				map={sprite}
				transparent={true}
				sizeAttenuation={false}
				depthTest={false}
			/>
		</T.Points>
	{/await}
{/if}

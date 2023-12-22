<script>
	import { useTexture } from '@threlte/extras'
	import { BufferGeometry, Float32BufferAttribute } from 'three'
	import { T } from '@threlte/core'

	export let x
	export let y
	export let z
	export let hidden
	export let name

	let source = '/actions/simple_point_min.svg'

	const texture = useTexture(source)

	let geom = new BufferGeometry()
	let vertices = new Float32Array([x, y, z])
	geom.setAttribute('position', new Float32BufferAttribute(vertices, 3))

	// const geometry = new THREE.SphereGeometry(15, 32, 16)
	// const material = new THREE.MeshBasicMaterial({ color: 0xffff00 })
	// const sphere = new THREE.Mesh(geometry, material)
</script>

{#if !hidden}
	{#await texture then sprite}
		<!-- <T.Mesh position.x={x} position.y={y} position.z={z}>
			<T.MeshStandardMaterial color="#FF0000" />
			<T.SphereGeometry args={[0.015, 32, 16]} />
		</T.Mesh> -->

		<T.Points geometry={geom}>
			<T.PointsMaterial
				size={6}
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

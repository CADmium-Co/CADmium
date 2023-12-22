<script>
	import { useThrelte } from '@threlte/core'
	import { Line2 } from 'three/addons/lines/Line2.js'
	import { LineMaterial } from 'three/addons/lines/LineMaterial.js'
	import { LineGeometry } from 'three/addons/lines/LineGeometry.js'
	import { Matrix4, Euler, MeshStandardMaterial, DoubleSide, Vector2, Vector3 } from 'three'
	import { T, extend } from '@threlte/core'

	export let start
	export let end

	let lineMaterial

	const { size, dpr } = useThrelte()

	$: {
		lineMaterial = new LineMaterial({
			color: '#000000',
			linewidth: 1.0 * $dpr,
			depthTest: false,
			transparent: false,
			dashed: true,
			dashSize: 0.1,
			gapSize: 0.1,
			dashScale: 3,
			resolution: new Vector2($size.width * $dpr, $size.height * $dpr)
		})
		lineMaterial.defines.USE_DASH = ''
	}

	const points = [
		start.threeD.x,
		start.threeD.y,
		start.threeD.z,
		end.threeD.x,
		end.threeD.y,
		end.threeD.z
	]
	const lineGeometry = new LineGeometry()
	lineGeometry.setPositions(points)
</script>

<T.Line2
	geometry={lineGeometry}
	material={lineMaterial}
	on:create={({ ref }) => {
		ref.computeLineDistances()
	}}
/>

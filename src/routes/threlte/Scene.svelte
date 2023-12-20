<script>
	import { T } from '@threlte/core'
	import { TrackballControls, Gizmo } from '@threlte/extras'
	import { Vector3, BoxGeometry } from 'three'

	import { realization } from './stores.js'

	import Point3D from './Point3D.svelte'
	import Plane from './Plane.svelte'

	$: points = $realization.points ? Object.entries($realization.points) : []
	$: planes = $realization.planes ? Object.entries($realization.planes) : []

	$: console.log(planes)
</script>

<T.OrthographicCamera makeDefault position={[160.8, -250.8, 200.55]} zoom={200} up={[0, 0, 1]}>
	<TrackballControls
		rotateSpeed={1.8}
		on:create={({ ref }) => {
			ref.up = new Vector3(0, 0, 1.0)
			ref.panSpeed = 0.6
		}}
	/>
</T.OrthographicCamera>

<T.DirectionalLight position.x={10} position.y={10} position.z={10} />
<T.DirectionalLight position.x={-15} position.y={-10} position.z={10} />
<T.AmbientLight intensity={0.3} />

<!-- <T.GridHelper args={[2, 2]} rotation.x={Math.PI / 2} /> -->

<!-- <T.Mesh position.z={1} geometry={new BoxGeometry(2, 2, 2)}>
	<T.MeshStandardMaterial />
</T.Mesh> -->

{#each points as [pointName, point]}
	<Point3D name={pointName} x={point.x} y={point.y} z={point.z} hidden={point.hidden} />
{/each}

{#each planes as [planeName, plane]}
	<Plane
		name={planeName}
		height={plane.height}
		width={plane.width}
		origin={plane.plane.origin}
		primary={plane.plane.primary}
		secondary={plane.plane.secondary}
		tertiary={plane.plane.tertiary}
	/>
{/each}

<Gizmo verticalPlacement={top} size={110} paddingX={10} paddingY={10} />

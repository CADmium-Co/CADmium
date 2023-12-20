<script>
	import { T } from '@threlte/core'
	import { TrackballControls, Gizmo, Environment } from '@threlte/extras'
	import { Vector3 } from 'three'

	import { realization } from './stores.js'

	import Point3D from './Point3D.svelte'
	import Plane from './Plane.svelte'
	import Solid from './Solid.svelte'

	$: points = $realization.points ? Object.entries($realization.points) : []
	$: planes = $realization.planes ? Object.entries($realization.planes) : []
	$: solids = $realization.solids ? Object.entries($realization.solids) : []
</script>

<T.OrthographicCamera makeDefault position={[160.8, -250.8, 200.55]} zoom={400} up={[0, 0, 1]}>
	<TrackballControls
		rotateSpeed={1.8}
		on:create={({ ref }) => {
			ref.up = new Vector3(0, 0, 1.0)
			ref.panSpeed = 0.6
		}}
	/>
</T.OrthographicCamera>

<!-- <T.DirectionalLight args={['#ff8888', 50.0]} position.x={-10} position.y={0} position.z={0} />
<T.DirectionalLight args={['#88ff88', 50.0]} position.x={10} position.y={0} position.z={0} />
<T.DirectionalLight args={['#8888ff', 50.0]} position.x={0} position.y={0} position.z={10} /> -->
<!-- <T.DirectionalLight position.x={10} position.y={0} position.z={0} /> -->

<!-- <T.PointLight args={['#ffffff', 5000.0]} position.x={3} position.y={3} position.z={15} />
<T.PointLight args={['#ffffff', 3000.0]} position.x={3} position.y={-3} position.z={-15} />
<T.PointLight args={['#ffffff', 3000.0]} position.x={10} position.y={-13} position.z={1.1} />
<T.PointLight args={['#ffffff', 3000.0]} position.x={-10.5} position.y={11} position.z={0.86} /> -->

<!-- <T.AmbientLight intensity={0.6} /> -->

<Environment
	path="/envmap/hdr/"
	files="kloofendal_28d_misty_puresky_1k.hdr"
	isBackground={false}
	format="hdr"
/>

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

{#each solids as [solidName, solid]}
	<Solid
		name={solidName}
		indices={solid.indices}
		vertices={solid.vertices}
		normals={solid.normals}
	/>
{/each}

<Gizmo verticalPlacement={top} size={110} paddingX={10} paddingY={10} />

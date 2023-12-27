<script>
	import { T } from '@threlte/core'
	import { TrackballControls, Gizmo, Environment } from '@threlte/extras'
	import { Vector3, MOUSE } from 'three'
	import { interactivity } from '@threlte/extras'

	import { realization, workbench, sketchBeingEdited } from './stores.js'

	import Point3D from './Point3D.svelte'
	import Plane from './Plane.svelte'
	import Solid from './Solid.svelte'
	import Sketch from './Sketch.svelte'

	interactivity()

	$: points = $realization.points ? Object.entries($realization.points) : []
	$: planes = $realization.planes ? Object.entries($realization.planes) : []
	$: planesById = planes ? Object.fromEntries(planes) : {}
	$: solids = $realization.solids ? Object.entries($realization.solids) : []
	$: sketches = $realization.sketches ? Object.entries($realization.sketches) : []
	$: console.log('sketches', sketches)

	// mouseButtons={{ LEFT: 0, MIDDLE: 1, RIGHT: 2 }} 0 // standard
	// mouseButtons={{ LEFT: 0, MIDDLE: 2, RIGHT: 1 }} 1 // no
	// mouseButtons={{ LEFT: 1, MIDDLE: 0, RIGHT: 2 }} 2 // close!
	// mouseButtons={{ LEFT: 1, MIDDLE: 2, RIGHT: 0 }} 3 // close?
	// mouseButtons={{ LEFT: 2, MIDDLE: 1, RIGHT: 0 }} 5 // no
	// mouseButtons={{ LEFT: 2, MIDDLE: 0, RIGHT: 1 }} 4 // seems to meet most people's expectations
	// mouseButtons={{ LEFT: 2, MIDDLE: 50, RIGHT: 1 }} 4 // disable left click entirely--free it up for interaction

	// camera position: [160.8, -250.8, 200.55] looks good and angular
</script>

<T.OrthographicCamera makeDefault position={[160.8, -250.8, 200.55]} zoom={400} up={[0, 0, 1]}>
	<TrackballControls
		rotateSpeed={1.8}
		on:create={({ ref }) => {
			ref.up = new Vector3(0, 0, 1.0)
			ref.panSpeed = 0.6
		}}
		mouseButtons={{ LEFT: 2, MIDDLE: 50, RIGHT: 1 }}
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

{#each points as [pointName, point] (`${$workbench.name}-${pointName}`)}
	<Point3D name={pointName} x={point.x} y={point.y} z={point.z} hidden={point.hidden} />
{/each}

{#each planes as [planeName, plane] (`${$workbench.name}-${planeName}`)}
	<Plane
		name={plane.name}
		uniqueId={planeName}
		height={plane.height}
		width={plane.width}
		origin={plane.plane.origin}
		primary={plane.plane.primary}
		secondary={plane.plane.secondary}
		tertiary={plane.plane.tertiary}
	/>
{/each}

{#each sketches as [sketchId, sketchTuple] (`${$workbench.name}-${sketchId}`)}
	<Sketch
		uniqueId={sketchId}
		name={sketchTuple[2]}
		{sketchTuple}
		editing={$sketchBeingEdited === sketchId}
		plane={planesById[sketchTuple[0].plane_id]}
	/>
{/each}

{#each solids as [solidName, solid] (`${$workbench.name}-${solidName}`)}
	<Solid
		name={solidName}
		indices={solid.indices}
		vertices={solid.vertices}
		normals={solid.normals}
	/>
{/each}

<Gizmo verticalPlacement={top} size={110} paddingX={10} paddingY={10} />

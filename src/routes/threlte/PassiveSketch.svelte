<script>
	import Point2D from './Point2D.svelte'
	import Line from './Line.svelte'
	import Circle from './Circle.svelte'

	import { hiddenSketches } from './stores.js'

	export let uniqueId
	export let name
	export let sketch
	export let plane

	const pointIds = Object.keys(sketch.points)
	const pointTuples = []
	const pointsById = {}
	for (let pointId of pointIds) {
		const point3D = sketch.points[pointId]
		const point2D = sketch.points_2d[pointId]
		pointTuples.push({ id: pointId, twoD: point2D, threeD: point3D })
		pointsById[pointId] = { twoD: point2D, threeD: point3D }
	}

	const lineTuples = []
	for (let lineId of Object.keys(sketch.line_segments)) {
		const line = sketch.line_segments[lineId]
		const start = pointsById[line.start]
		const end = pointsById[line.end]

		lineTuples.push({ id: lineId, start, end })
	}

	const circleTuples = []
	for (let circleId of Object.keys(sketch.circles)) {
		const circle = sketch.circles[circleId]
		const center = pointsById[circle.center]
		const radius = circle.radius
		circleTuples.push({ id: circleId, center, radius })
	}

	$: hidden = $hiddenSketches.includes(uniqueId)
</script>

{#if !hidden}
	{#each pointTuples as { id, twoD, threeD } (id)}
		<Point2D {name} x={threeD.x} y={threeD.y} z={threeD.z} hidden={threeD.hidden} />
	{/each}

	{#each lineTuples as line (line.id)}
		<Line start={line.start} end={line.end} />
	{/each}

	{#each circleTuples as circle (circle.id)}
		<Circle center={circle.center} radius={circle.radius} {plane} id={circle.id} />
	{/each}
{/if}

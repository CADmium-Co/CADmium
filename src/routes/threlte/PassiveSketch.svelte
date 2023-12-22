<script>
	import Point2D from './Point2D.svelte'
	import Line from './Line.svelte'

	export let name
	export let sketch

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

	console.log('lineTuples', lineTuples)
</script>

{#each pointTuples as { id, twoD, threeD } (id)}
	<Point2D {name} x={threeD.x} y={threeD.y} z={threeD.z} hidden={false} />
{/each}

{#each lineTuples as line (line.id)}
	<Line start={line.start} end={line.end} />
{/each}

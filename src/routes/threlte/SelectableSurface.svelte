<script>
	import nurbs from 'nurbs'
	export let truck_face, truck_vertices, truck_edges

	let curve = nurbs({
		points: [
			[-1, 0],
			[-0.5, 0.5],
			[0.5, -0.5],
			[1, 0]
		],
		degree: 2
	})
	console.log('Hello!', curve.domain)
</script>

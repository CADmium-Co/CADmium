export const currentlySelected = [
	{
		type: "line",
		id: "9"
	},
	{
		type: "meshFace",
		id: "8"
	},
	{
		type: "point3D",
		id: "7"
	},
	{
		type: "point",
		id: "6"
	},
	{
		type: "plane",
		id: "5"
	},
	{
		type: "plane",
		id: "4"
	},
	{
		type: "face",
		id: "3"
	},
	{
		type: "arc",
		id: "2"
	},
	{
		type: "circle",
		id: "1"
	}
]

export const project = {
	name: "First Project",
	assemblies: [],
	workbenches: [
		{
			name: "Workbench 1",
			history: [
				{
					name: "Origin",
					unique_id: "Point-0",
					suppressed: false,
					data: {
						type: "Point",
						point: {
							x: 0,
							y: 0,
							z: 0,
							hidden: false
						}
					}
				},
				{
					name: "Top",
					unique_id: "Plane-0",
					suppressed: false,
					data: {
						type: "Plane",
						plane: {
							origin: {
								x: 0,
								y: 0,
								z: 0,
								hidden: false
							},
							primary: {
								x: 1,
								y: 0,
								z: 0
							},
							secondary: {
								x: 0,
								y: 1,
								z: 0
							},
							tertiary: {
								x: 0,
								y: 0,
								z: 1
							}
						},
						width: 100,
						height: 100
					}
				},
				{
					name: "Front",
					unique_id: "Plane-1",
					suppressed: false,
					data: {
						type: "Plane",
						plane: {
							origin: {
								x: 0,
								y: 0,
								z: 0,
								hidden: false
							},
							primary: {
								x: 1,
								y: 0,
								z: 0
							},
							secondary: {
								x: 0,
								y: 0,
								z: 1
							},
							tertiary: {
								x: 0,
								y: -1,
								z: 0
							}
						},
						width: 100,
						height: 100
					}
				},
				{
					name: "Right",
					unique_id: "Plane-2",
					suppressed: false,
					data: {
						type: "Plane",
						plane: {
							origin: {
								x: 0,
								y: 0,
								z: 0,
								hidden: false
							},
							primary: {
								x: 0,
								y: 1,
								z: 0
							},
							secondary: {
								x: 0,
								y: 0,
								z: 1
							},
							tertiary: {
								x: 1,
								y: 0,
								z: 0
							}
						},
						width: 100,
						height: 100
					}
				},
				{
					name: "Sketch 1",
					unique_id: "Sketch-0",
					suppressed: false,
					data: {
						type: "Sketch",
						plane_description: {
							PlaneId: "Plane-0"
						},
						width: 1.25,
						height: 0.75,
						sketch: {
							points: {
								"1": {
									x: -52.95156757144787,
									y: 7.317926785688712,
									m: 1,
									dx: 0,
									dy: 0,
									fx: 0,
									fy: 0,
									fixed: false,
									hidden: false
								},
								"2": {
									x: 76.03513162109667,
									y: -45.27256950397148,
									m: 1,
									dx: 0,
									dy: 0,
									fx: 0,
									fy: 0,
									fixed: false,
									hidden: false
								},
								"3": {
									x: 76.03513162109667,
									y: 7.317926785688712,
									m: 1,
									dx: 0,
									dy: 0,
									fx: 0,
									fy: 0,
									fixed: false,
									hidden: false
								},
								"4": {
									x: -52.95156757144787,
									y: -45.27256950397148,
									m: 1,
									dx: 0,
									dy: 0,
									fx: 0,
									fy: 0,
									fixed: false,
									hidden: false
								},
								"5": {
									x: -9.20153199031077,
									y: -21.640848057170956,
									m: 1,
									dx: 0,
									dy: 0,
									fx: 0,
									fy: 0,
									fixed: false,
									hidden: false
								},
								"6": {
									x: 3.837447277628939,
									y: -26.044145549343227,
									m: 1,
									dx: 0,
									dy: 0,
									fx: 0,
									fy: 0,
									fixed: false,
									hidden: true
								}
							},
							highest_point_id: 6,
							line_segments: {
								"1": {
									start: 1,
									end: 4
								},
								"2": {
									start: 4,
									end: 2
								},
								"3": {
									start: 2,
									end: 3
								},
								"4": {
									start: 3,
									end: 1
								}
							},
							highest_line_segment_id: 4,
							circles: {
								"1": {
									center: 5,
									radius: 13.76241291178012,
									top: 6
								}
							},
							highest_circle_id: 1,
							arcs: {},
							highest_arc_id: 0,
							constraints: {},
							highest_constraint_id: 0
						}
					}
				},
				{
					name: "Extrusion 1",
					unique_id: "Extrusion-0",
					suppressed: false,
					data: {
						type: "Extrusion",
						extrusion: {
							sketch_id: "Sketch-0",
							face_ids: [1],
							length: 200,
							offset: 0,
							direction: "Normal",
							mode: "New"
						}
					}
				}
			],
			step_counters: {
				Sketch: 1,
				Extrusion: 1,
				Point: 1,
				Plane: 3
			}
		}
	]
}


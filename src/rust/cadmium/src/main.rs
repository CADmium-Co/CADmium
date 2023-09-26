#![allow(dead_code, unused)]
use cadmium::project::Project;

fn main() {
    let mut p = Project::new("First Project");
    p.add_defaults();

    let wb = p.workbenches.get_mut(0).unwrap();
    wb.add_sketch("Sketch1", "Top");

    let sketch = wb.get_sketch_mut("Sketch1").unwrap();

    // let id0 = sketch.add_fixed_point(0.0, 0.0);
    // let id1 = sketch.add_fixed_point(1.0, 0.0);
    // let id2 = sketch.add_point(0.5, 0.5);

    // TODO: instead of point masses and springs, implement an API
    // that looks a lot more like regular 2D constraints: lines with lengths,
    // defined angles, etc.

    // let spring0 = sketch.add_spring(id0, id2, 1.0);
    // let spring1 = sketch.add_spring(id1, id2, 1.0);

    // for i in 0..10 {
    //     sketch.step();
    //     sketch.print_state_minimal();
    // }
    // println!("A mut sketch: {:?}", sketch);

    // sketch.print_state_minimal();
    // sketch.solve(100);
    // sketch.print_state_minimal();

    // println!("Project: {:?}", p);

    // Add a nice circle
    // let id4 = sketch.add_point(-0.75, 0.5);
    // let c_id0 = sketch.add_circle(id4, 0.5);

    // let serialized = serde_json::to_string(&p).unwrap();
    // println!("serialized = {}", serialized);

    // A line off on its own
    // let id5 = sketch.add_point(-0.5, -0.5);
    // let id6 = sketch.add_point(-0.75, -0.75);
    // sketch.add_segment(id5, id6);

    sketch.save_svg("test_svgs/test2.svg");

    // let realized = wb.realize(30);
    // println!("Real: {:?}", realized);
    // println!("Real json: {}", serde_json::to_string(&realized).unwrap())

    // wb.add_extrusion("Ext1", "Sketch1", 0.5);
}

// TODO: WHEN YOU RETURN:
// 1. Move the circle to the right, inside the rectangle.
// 2. Make a tiny circle and put it inside the innermost rounded rectangle.

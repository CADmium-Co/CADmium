#![allow(dead_code, unused)]
// use cadmium::sketch::test_svg;
use cadmium::sketch::test_svg;
use cadmium::Project;

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

    // let seg_0 = sketch.add_segment(id0, id1);
    // let seg_1 = sketch.add_segment(id1, id2);
    // let seg_1 = sketch.add_segment(id2, id0);

    // A big square
    let id0 = sketch.add_point(-0.1, -1.0);
    let id1 = sketch.add_point(1.0, -1.0);
    let id2 = sketch.add_point(1.0, 1.0);
    let id3 = sketch.add_point(-0.1, 1.0);

    sketch.add_segment(id0, id1);
    sketch.add_segment(id1, id2);
    sketch.add_segment(id2, id3);
    sketch.add_segment(id3, id0);

    // add an arc
    let s = sketch.add_point(0.25, -0.25);
    let e = sketch.add_point(0.0, -0.50);
    let c = sketch.add_point(0.25, -0.50);
    let t0 = sketch.add_point(0.0, -0.75);
    let t1 = sketch.add_point(0.5, -0.75);
    let t2 = sketch.add_point(0.5, -0.25);
    let arc_id = sketch.add_arc(c, s, e);
    sketch.add_segment(e, t0);
    sketch.add_segment(t0, t1);
    sketch.add_segment(t1, t2);
    sketch.add_segment(t2, s);

    // sketch.add_line_segment(0.0, 0.0, 1.0, 0.0);

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
    let id4 = sketch.add_point(-0.75, 0.5);
    let c_id0 = sketch.add_circle(id4, 0.5);
    // let c_id1 = sketch.add_circle(id1, 0.25);
    // let c_id2 = sketch.add_circle(id2, 0.35);
    // let c_id3 = sketch.add_circle(id3, 0.45);

    // let serialized = serde_json::to_string(&p).unwrap();
    // println!("serialized = {}", serialized);

    sketch.save_svg("test1.svg");

    // let rings = sketch.find_rings(false);
    // println!("Rings: {:?}", rings);

    // let faces = sketch.find_faces(false);
    // println!("Faces: {:?}", faces);

    // let realized = wb.realize(30);
    // println!("Real: {:?}", realized);
    // println!("Real json: {}", serde_json::to_string(&realized).unwrap())

    // wb.add_extrusion("Ext1", "Sketch1", 0.5);
}

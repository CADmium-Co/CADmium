#![allow(dead_code, unused)]
use cadmium::Project;

fn main() {
    let mut p = Project::new("First Project");
    p.add_defaults();

    let wb = p.workbenches.get_mut(0).unwrap();
    wb.add_sketch("Sketch1", "Top");

    let sketch = wb.get_sketch_mut("Sketch1").unwrap();

    let id0 = sketch.add_fixed_point(0.0, 0.0);
    let id1 = sketch.add_point(2.0, 0.0);

    let spring0 = sketch.add_spring(id0, id1, 1.0);

    // for i in 0..10 {
    //     sketch.step();
    //     sketch.print_state_minimal();
    // }
    // println!("A mut sketch: {:?}", sketch);

    sketch.print_state_minimal();
    sketch.solve(100);
    sketch.print_state_minimal();

    // println!("Project: {:?}", p);

    // let serialized = serde_json::to_string(&p).unwrap();
    // println!("serialized = {}", serialized);

    // let realized = wb.realize(30);
    // println!("Real: {:?}", realized);
    // println!("Real json: {}", serde_json::to_string(&realized).unwrap())
}

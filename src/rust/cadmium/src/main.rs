#![allow(dead_code, unused)]
use cadmium::Project;

fn main() {
    let mut p = Project::new("First Project");
    p.add_defaults();

    let wb = p.workbenches.get_mut(0).unwrap();
    let sketch = wb.add_sketch("Sketch1", "Top");

    // println!("Project: {:?}", p);

    // let serialized = serde_json::to_string(&p).unwrap();
    // println!("serialized = {}", serialized);

    let realized = wb.realize(30);
    // println!("Real: {:?}", realized);
    println!("Real json: {}", serde_json::to_string(&realized).unwrap())
}

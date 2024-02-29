use super::*;

#[test]
fn my_test() {
    let mut p = Project::new("Test Project");
    p.add_defaults();
}

#[test]
fn secondary_extrusion_simple() {
    let mut p = Project::new("Test Project");
    p.add_defaults();
    let mut wb = p.workbenches.get_mut(0).unwrap();
    wb.add_sketch_to_plane("Sketch 1", "Plane-0");
    let mut s = wb.get_sketch_mut("Sketch 1").unwrap();
    let ll = s.add_point(2.0, 2.0);
    let lr = s.add_point(42.0, 2.0);
    let ul = s.add_point(2.0, 42.0);
    let ur = s.add_point(42.0, 42.0);
    s.add_segment(ll, lr);
    s.add_segment(lr, ur);
    s.add_segment(ur, ul);
    s.add_segment(ul, ll);

    let extrusion = Extrusion::new(
        "Sketch-0".to_owned(),
        vec![0],
        25.0,
        0.0,
        Direction::Normal,
        ExtrusionMode::New,
    );
    wb.add_extrusion("Ext1", extrusion);

    let s2_id = wb.add_sketch_to_solid_face("Sketch-2", "Ext1:0", Vector3::new(0.0, 0.0, 1.0));
    let mut s2 = wb.get_sketch_mut("Sketch-2").unwrap();

    // smaller
    let ll = s2.add_point(12.0, 12.0);
    let lr = s2.add_point(32.0, 12.0);
    let ul = s2.add_point(12.0, 32.0);
    let ur = s2.add_point(32.0, 32.0);
    // bigger!
    // let ll = s2.add_point(-10.0, -10.0);
    // let lr = s2.add_point(50.0, -10.0);
    // let ul = s2.add_point(-10.0, 50.0);
    // let ur = s2.add_point(50.0, 50.0);
    s2.add_segment(ll, lr);
    s2.add_segment(lr, ur);
    s2.add_segment(ur, ul);
    s2.add_segment(ul, ll);

    // println!("S2: {:?}", s2);

    let extrusion2 = Extrusion::new(
        s2_id.to_owned(),
        vec![0],
        25.0,
        0.0,
        Direction::Normal,
        ExtrusionMode::Add(vec!["Ext1:0".to_string()]),
    );
    wb.add_extrusion("Ext2", extrusion2);

    let realization = p.get_realization(0, 1000);
    let solids = realization.solids;

    let num_solids = solids.len();
    println!("Num Solids: {:?}", num_solids);
    assert!(num_solids == 1);

    let final_solid = &solids["Ext1:0"];
    let mut mesh = final_solid.truck_solid.triangulation(0.02).to_polygon();
    mesh.put_together_same_attrs();
    let file = std::fs::File::create("secondary_extrusion.obj").unwrap();
    obj::write(&mesh, file).unwrap();

    let as_json = serde_json::to_string(&p).unwrap();
    let file = std::fs::File::create("secondary_extrusion.json").unwrap();
    // println!("As json: {}", as_json);
    serde_json::to_writer(file, &p).unwrap();
}

use cadmium::{
    extrusion::{Direction, Extrusion, ExtrusionMode},
    project::Project,
};

fn main() {
    let mut p = Project::new("Example Project");
    let wb = p.workbenches.get_mut(0).unwrap();
    wb.add_sketch_to_plane("Sketch 1", "Plane-0");
    let s = wb.get_sketch_mut("Sketch 1").unwrap();
    let ll = s.add_point(0.0, 0.0);
    let lr = s.add_point(40.0, 0.0);
    let ul = s.add_point(0.0, 40.0);
    let ur = s.add_point(40.0, 40.0);
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

    let realization = p.get_realization(0, 1000);
    let solids = realization.solids;
    let solid = &solids["Ext1:0"];

    println!("{:?}", solid);

    println!("Dump example files");
    solid.save_as_step("example.step");
    solid.save_as_obj("example.obj", 0.001);
}

use cadmium::archetypes::PlaneDescription;
use cadmium::feature::extrusion::{self, Direction, Mode};
use cadmium::isketch::primitive::{AddLine, SketchAddPointMessage};
use cadmium::message::MessageHandler as _;
use cadmium::project::Project;
use cadmium::workbench::AddSketch;

fn main() {
    let p = Project::new("Test Project");
    let wb_ref = p.workbenches.first().unwrap();
    let plane_description = PlaneDescription::PlaneId(0);
    let sketch_id = AddSketch { plane_description }
        .handle_message(wb_ref.clone())
        .unwrap()
        .unwrap()
        .0;
    let sketch = wb_ref.borrow().get_sketch_by_id(sketch_id).unwrap();

    let ll = SketchAddPointMessage { x: 0.0, y: 0.0 }
        .handle_message(sketch.clone())
        .unwrap()
        .unwrap()
        .0;
    let lr = SketchAddPointMessage { x: 40.0, y: 0.0 }
        .handle_message(sketch.clone())
        .unwrap()
        .unwrap()
        .0;
    let ul = SketchAddPointMessage { x: 0.0, y: 40.0 }
        .handle_message(sketch.clone())
        .unwrap()
        .unwrap()
        .0;
    let ur = SketchAddPointMessage { x: 40.0, y: 40.0 }
        .handle_message(sketch.clone())
        .unwrap()
        .unwrap()
        .0;

    AddLine { start: ll, end: lr }
        .handle_message(sketch.clone())
        .unwrap();
    AddLine { start: lr, end: ur }
        .handle_message(sketch.clone())
        .unwrap();
    AddLine { start: ur, end: ul }
        .handle_message(sketch.clone())
        .unwrap();
    AddLine { start: ul, end: ll }
        .handle_message(sketch.clone())
        .unwrap();

    extrusion::Add {
        sketch_id,
        faces: vec![0],
        length: 25.0,
        offset: 0.0,
        direction: Direction::Normal,
        mode: Mode::New,
    }
    .handle_message(wb_ref.clone())
    .unwrap();

    let wb = wb_ref.borrow();
    let feature_ref = wb.features.first_key_value().unwrap().1;
    let solid_like = feature_ref.borrow().as_solid_like().to_solids().unwrap();
    let solid = solid_like.get(0).unwrap();

    println!("{:?}", solid);

    println!("Dump example files");
    solid.save_as_step("example.step");
    solid.save_as_obj("example.obj", 0.001);
}

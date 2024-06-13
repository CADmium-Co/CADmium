use cadmium::archetypes::PlaneDescription;
use cadmium::feature::extrusion::{self, Direction, Mode};
use cadmium::isketch::primitive::{AddLine, SketchAddPointMessage};
use cadmium::message::idwrap::IDWrapable;
use cadmium::message::MessageHandler as _;
use cadmium::message::ProjectMessageHandler;
use cadmium::project::Project;
use cadmium::step::{StepHash, StepResult};
use cadmium::workbench::AddSketch;

fn main() {
    let mut p = Project::new("Test Project");
    let wb_hash = StepHash::from_int(0);
    let plane_description = PlaneDescription::PlaneId(0);
    let sketch_id = AddSketch { plane_description }
        .id_wrap(wb_hash)
        .handle_project_message(&mut p)
        .unwrap()
        .unwrap();

    let wb_ref = p.workbenches.first().unwrap().clone();
    let step = wb_ref.borrow().get_step_by_hash(sketch_id).unwrap();

    let StepResult::Sketch { sketch, .. } = step.borrow().result.clone() else {
        panic!("Expected a sketch");
    };

    let ll = SketchAddPointMessage { x: 0.0, y: 0.0 }
        .id_wrap(sketch_id)
        .id_wrap(wb_hash)
        .handle_project_message(&mut p)
        .unwrap()
        .unwrap();
    let lr = SketchAddPointMessage { x: 40.0, y: 0.0 }
        .id_wrap(sketch_id)
        .id_wrap(wb_hash)
        .handle_project_message(&mut p)
        .unwrap()
        .unwrap();
    let ul = SketchAddPointMessage { x: 0.0, y: 40.0 }
        .id_wrap(sketch_id)
        .id_wrap(wb_hash)
        .handle_project_message(&mut p)
        .unwrap()
        .unwrap();
    let ur = SketchAddPointMessage { x: 40.0, y: 40.0 }
        .id_wrap(sketch_id)
        .id_wrap(wb_hash)
        .handle_project_message(&mut p)
        .unwrap()
        .unwrap();

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

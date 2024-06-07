use cadmium::archetypes::PlaneDescription;
use cadmium::feature::extrusion;
use cadmium::isketch::primitive::{AddLine, AddPoint};
use cadmium::message::idwrap::IDWrap;
use cadmium::message::ProjectMessageHandler;
use cadmium::project::Project;
use cadmium::workbench::AddSketch;
use cadmium::IDType;

pub fn create_test_project() -> Project {
    let mut p = Project::new("Test Project");
    let plane_description = PlaneDescription::PlaneId(0);
    let sketch_id = IDWrap { id: 0, inner: AddSketch { plane_description } }.handle_project_message(&mut p).unwrap().unwrap();

    add_test_rectangle(&mut p, sketch_id, 0.0, 0.0, 40.0, 40.0);

    IDWrap {
        id: 0,
        inner: extrusion::Add {
            sketch_id: 0,
            faces: vec![0],
            length: 25.0,
            offset: 0.0,
            direction: extrusion::Direction::Normal,
            mode: extrusion::Mode::New
        }
    }.handle_project_message(&mut p).unwrap().unwrap();

    p
}

pub fn add_test_rectangle(p: &mut Project, sketch_id: IDType, x_start: f64, y_start: f64, x_end: f64, y_end: f64) {
    let ll = IDWrap { id: 0, inner: IDWrap { id: sketch_id, inner: AddPoint { x: x_start, y: y_start } } }.handle_project_message(p).unwrap().unwrap();
    let lr = IDWrap { id: 0, inner: IDWrap { id: sketch_id, inner: AddPoint { x: x_end,  y: y_start } } }.handle_project_message(p).unwrap().unwrap();
    let ul = IDWrap { id: 0, inner: IDWrap { id: sketch_id, inner: AddPoint { x: x_start,  y: y_end } } }.handle_project_message(p).unwrap().unwrap();
    let ur = IDWrap { id: 0, inner: IDWrap { id: sketch_id, inner: AddPoint { x: x_end, y: y_end } } }.handle_project_message(p).unwrap().unwrap();

    IDWrap { id: 0, inner: IDWrap { id: sketch_id, inner: AddLine { start: ll, end: lr } } }.handle_project_message(p).unwrap().unwrap();
    IDWrap { id: 0, inner: IDWrap { id: sketch_id, inner: AddLine { start: lr, end: ur } } }.handle_project_message(p).unwrap().unwrap();
    IDWrap { id: 0, inner: IDWrap { id: sketch_id, inner: AddLine { start: ur, end: ul } } }.handle_project_message(p).unwrap().unwrap();
    IDWrap { id: 0, inner: IDWrap { id: sketch_id, inner: AddLine { start: ul, end: ll } } }.handle_project_message(p).unwrap().unwrap();
}

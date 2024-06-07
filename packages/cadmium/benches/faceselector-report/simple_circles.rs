use cadmium::isketch::primitive::AddCircle;
use cadmium::message::idwrap::IDWrap;
use cadmium::message::ProjectMessageHandler;
use cadmium::IDType;

use crate::TestCase;

#[derive(Debug)]
pub struct SingleCircle();
impl TestCase for SingleCircle {
    fn pre_selection(&self, p: &mut cadmium::project::Project, sketch_id: IDType) {
        IDWrap { id: 0, inner: IDWrap { id: sketch_id, inner: AddCircle { center: 0, radius: 10.0 } } }.handle_project_message(p).unwrap().unwrap();
    }
    fn post_selection(&self, _p: &mut cadmium::project::Project, _sketch_id: IDType) {}
}

#[derive(Debug)]
pub struct SingleCircleAddAnother();
impl TestCase for SingleCircleAddAnother {
    fn pre_selection(&self, p: &mut cadmium::project::Project, sketch_id: IDType) {
        SingleCircle().pre_selection(p, sketch_id)
    }
    fn post_selection(&self, p: &mut cadmium::project::Project, sketch_id: IDType) {
        IDWrap { id: 0, inner: IDWrap { id: sketch_id, inner: AddCircle { center: 0, radius: 20.0 } } }.handle_project_message(p).unwrap().unwrap();
    }
}

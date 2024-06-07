pub use isotope::decompose::face::Face;

use crate::IDType;

use super::ISketch;

pub trait FaceSelector {
    fn get_selected_faces(&self, isketch: &ISketch) -> Vec<Face>;
    fn from_face_ids(ids: Vec<IDType>) -> Self;
}

#[derive(Debug)]
pub struct IDSelector {
    pub ids: Vec<IDType>,
}

impl FaceSelector for IDSelector {
    fn get_selected_faces(&self, isketch: &ISketch) -> Vec<Face> {
        isketch
            .sketch()
            .borrow()
            .get_merged_faces()
            .iter()
            .enumerate()
            .filter_map(|(id, f)| {
                if self.ids.contains(&(id as IDType)) {
                    Some(f.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    fn from_face_ids(ids: Vec<IDType>) -> Self {
        Self { ids }
    }
}

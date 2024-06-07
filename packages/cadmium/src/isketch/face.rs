pub use isotope::decompose::face::Face;
use serde::{Deserialize, Serialize};

use crate::IDType;

use super::ISketch;

// Configuration of which selector to use
// As soon as we land on a single selector, this trait should no longer be required
// it's mainly used for the bench-faceselector-report
pub type Selector = IDSelector;

pub trait FaceSelector {
    fn get_selected_faces(&self, isketch: &ISketch) -> Vec<Face>;
    fn from_face_ids(ids: Vec<IDType>) -> Self;
}

/// The most simple selector, just select faces by their ID
/// If the number or order of faces change for any reason, this selector will break
#[derive(Debug, Clone, Serialize, Deserialize)]
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

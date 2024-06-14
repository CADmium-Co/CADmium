use geo::{Centroid, EuclideanDistance as _};
use isotope::primitives::point2::Point2;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

pub use isotope::decompose::face::Face;

use crate::IDType;

use super::ISketch;

// Configuration of which selector to use
// As soon as we land on a single selector, this trait should no longer be required
// it's mainly used for the bench-faceselector-report
pub type Selector = IDSelector;

pub trait FaceSelector {
    fn get_selected_faces(&self, isketch: &ISketch) -> Vec<Face>;
    fn from_face_ids(sketch: &ISketch, ids: Vec<IDType>) -> Self;
}

/// The most simple selector, just select faces by their ID
/// If the number or order of faces change for any reason, this selector will break
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IDSelector {
    pub ids: Vec<IDType>,
}

impl FaceSelector for IDSelector {
    fn get_selected_faces(&self, sketch: &ISketch) -> Vec<Face> {
        sketch.get_face_ids(self.ids.clone())
    }

    fn from_face_ids(_sketch: &ISketch, ids: Vec<IDType>) -> Self {
        Self { ids }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CentroidSelector {
    pub centroids: Vec<Point2>,
}

impl CentroidSelector {
    pub fn get_face_centroid(&self, face: &Face) -> Point2 {
        let centroid = face.as_polygon().centroid().unwrap();
        Point2::new(centroid.x(), centroid.y())
    }
}

impl FaceSelector for CentroidSelector {
    fn get_selected_faces(&self, sketch: &ISketch) -> Vec<Face> {
        self.centroids
            .iter()
            .filter_map(|c| {
                let point = geo::Point::new(c.x(), c.y());
                let faces = sketch.faces();
                let min = faces.iter().min_by(|a, b| {
                    let Some(a_centroid) = &a.as_polygon().centroid() else {
                        return std::cmp::Ordering::Greater;
                    };
                    let Some(b_centroid) = &b.as_polygon().centroid() else {
                        return std::cmp::Ordering::Greater;
                    };
                    let a_distance = a_centroid.euclidean_distance(&point);
                    let b_distance = b_centroid.euclidean_distance(&point);
                    a_distance.partial_cmp(&b_distance).unwrap()
                });

                min.cloned()
            })
            .collect_vec()
    }

    fn from_face_ids(sketch: &ISketch, ids: Vec<IDType>) -> Self {
        Self {
            centroids: sketch
                .get_face_ids(ids)
                .iter()
                .filter_map(|f| {
                    // We're straight-up skipping faces without a centroid
                    if let Some(centroid) = f.as_polygon().centroid() {
                        Some(Point2::new(centroid.x(), centroid.y()))
                    } else {
                        None
                    }
                })
                .collect(),
        }
    }
}

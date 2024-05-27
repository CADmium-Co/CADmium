
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

use crate::archetypes::{Plane, PlaneDescription, Point3};
use crate::extrusion::Extrusion;
use crate::isketch::ISketch;
use crate::solid::Solid;
use crate::workbench::Workbench;

#[derive(Tsify, Debug, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum StepOperation {
    Add,
    Update,
    Delete,
}

#[derive(Tsify, Debug, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Step {
    pub(crate) operation: StepOperation,
    pub(crate) name: String,
    pub(crate) unique_id: String,
    pub(crate) suppressed: bool,
    pub(crate) data: StepData,
}

macro_rules! define_steps {
    ($($parent:ident {
        $($name:ident {
            $($field:ident: $type:ty),* $(,)?
        }),* $(,)?
    }),* $(,)?) => {
        paste::paste! {
            #[derive(tsify::Tsify, Debug, serde::Serialize, serde::Deserialize)]
            #[tsify(into_wasm_abi, from_wasm_abi)]
            pub enum StepData {
                $(
                    $(
                        [<$parent $name>] {
                            $($field: $type),*
                        }
                    ),*
                ),*
            }
        }

        impl crate::workbench::Workbench {
            $(
                paste::paste! {
                    pub fn [<delete_ $parent:lower _id>](name: String, id: crate::IDType) -> Step {
                        todo!("Delete ID step")
                    }
                }

                $(
                    paste::paste! {
                        pub fn [<add_ $parent:lower _ $name:lower>](&mut self, name: String, $($field: $type),*) -> Result<crate::IDType, anyhow::Error> {
                            let id = $parent::[<add_ $name:lower>](self, $($field),* )?;

                            let step = Step {
                                name,
                                operation: StepOperation::Add,
                                unique_id: format!(concat!("Add:", stringify!($parent), stringify!($name), "-{}"), id),
                                suppressed: false,
                                data: StepData::[<$parent $name>] {
                                    $($field),*
                                },
                            };

                            self.history.push(step);

                            Ok(id)
                        }

                        pub fn [<update_ $parent:lower $name:lower>](name: String, $($field: $type),*) -> Step {
                            todo!("Update step")
                        }
                    }

                )*
            )*

            paste::paste! {
                pub fn do_step(&self, step: Step) -> Result<(), crate::error::CADmiumError> {
                    match step.data {
                        $(
                            $(
                                StepData::[<$parent $name>] {
                                    $($field),*
                                } => {
                                    todo!("Do step")
                                }
                            ),*
                        ),*
                    }

                    // Ok(())
                }

                pub fn undo_step(&self, step: Step) -> Result<(), crate::error::CADmiumError> {
                    match step.data {
                        $(
                            $(
                                StepData::[<$parent $name>] {
                                    $($field),*
                                } => {
                                    todo!("Undo step")
                                }
                            ),*
                        ),*
                    }

                    // Ok(())
                }
            }
        }
    }
}

define_steps! {
    // ISketch {
    //     Point {
    //         x: f64,
    //         y: f64,
    //     },
    //     Line {
    //         start: isotope::primitives::point2::Point2,
    //         end: isotope::primitives::point2::Point2,
    //     }
    // },
    Workbench {
        Point {
            point: Point3,
        },
        Plane {
            plane: Plane,
            width: f64,
            height: f64,
        },
        Sketch {
            plane_description: PlaneDescription,
            // sketch: ISketch,
            // width: f64,
            // height: f64,
        },
    },
    // Solid {
    //     Extrusion {
    //         extrusion: Extrusion,
    //     },
    // }
}

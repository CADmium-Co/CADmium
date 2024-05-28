
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

use crate::archetypes::{Plane, PlaneDescription, Point2, Point3};
use crate::extrusion::Extrusion;
use crate::isketch::ISketch;
use crate::solid::Solid;
use crate::workbench::Workbench;
use crate::IDType;

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
    ($($wb_field:literal => $parent_type:ty {
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
                        [<$parent_type $name>] {
                            workbench_id: crate::IDType,
                            [<$parent_type:snake _id>]: crate::IDType,
                            $($field: $type),*
                        }
                    ),*
                ),*
            }
        }

        impl crate::Project {
            $(
                paste::paste! {
                    pub fn [<delete_ $parent_type:snake _id>](name: String, id: crate::IDType) -> Step {
                        todo!("Delete ID step")
                    }
                }

                $(
                    paste::paste! {
                        pub fn [<add_ $parent_type:snake _ $name:snake>](&mut self, workbench_id: crate::IDType, [< $parent_type:snake _id >]: crate::IDType, name: String, $($field: $type),*) -> Result<crate::IDType, anyhow::Error> {
                            let wb_ = self.native.workbenches.get_mut(workbench_id as usize).ok_or(anyhow::anyhow!("Could not find workbench"))?;
                            let parent_ = wb_.[<$wb_field>].get_mut(&[< $parent_type:snake _id >]).ok_or(anyhow::anyhow!("Could not find parent"))?;
                            let result_id_ = parent_.borrow_mut().[< add_ $name:snake >]($($field),* )?;

                            let step_ = Step {
                                name,
                                operation: StepOperation::Add,
                                unique_id: format!(concat!("Add:", stringify!($parent_type), stringify!($name), "-{}"), result_id_),
                                suppressed: false,
                                data: StepData::[<$parent_type $name>] {
                                    workbench_id,
                                    [< $parent_type:snake _id >],
                                    $($field),*
                                },
                            };

                            wb_.history.push(step_);

                            Ok(result_id_)
                        }

                        pub fn [<update_ $parent_type:snake $name:snake>](name: String, $($field: $type),*) -> Step {
                            todo!("Update step")
                        }
                    }

                )*
            )*

            // paste::paste! {
            //     pub fn do_step(&self, step: Step) -> Result<(), crate::error::CADmiumError> {
            //         match step.data {
            //             $(
            //                 $(
            //                     StepData::[<$parent_type $name>] {
            //                         $($field),*
            //                     } => {
            //                         todo!("Do step")
            //                     }
            //                 ),*
            //             ),*
            //         }

            //         // Ok(())
            //     }

            //     pub fn undo_step(&self, step: Step) -> Result<(), crate::error::CADmiumError> {
            //         match step.data {
            //             $(
            //                 $(
            //                     StepData::[<$parent_type $name>] {
            //                         $($field),*
            //                     } => {
            //                         todo!("Undo step")
            //                     }
            //                 ),*
            //             ),*
            //         }

            //         // Ok(())
            //     }
            // }
        }
    };
}

define_steps! {
    // "self" => Workbench {
    //     Point {
    //         point: Point3,
    //     },
    //     Plane {
    //         plane: Plane,
    //         width: f64,
    //         height: f64,
    //     },
    //     Sketch {
    //         plane_description: PlaneDescription,
    //         // sketch: ISketch,
    //         // width: f64,
    //         // height: f64,
    //     },
    // },
    "sketches" => ISketch {
        Point {
            point: Point2,
        },
        Arc {
            center: IDType,
            radius: f64,
            clockwise: bool,
            start_angle: f64,
            end_angle: f64,
        },
        Circle {
            center: IDType,
            radius: f64,
        },
        Line {
            start: IDType,
            end: IDType,
        }
    },
    // Solid {
    //     Extrusion {
    //         extrusion: Extrusion,
    //     },
    // }
}

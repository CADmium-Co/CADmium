use crate::sketch::Sketch;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub assemblies: Vec<Assembly>,
    pub workbenches: Vec<Workbench>,
}

impl Project {
    pub fn new(name: &str) -> Self {
        let p = Project {
            name: name.to_owned(),
            assemblies: vec![],
            workbenches: vec![],
        };

        p
    }

    pub fn add_defaults(&mut self) {
        let mut w = Workbench::new("Workbench 1");
        w.add_defaults();
        self.workbenches.push(w);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Assembly {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Workbench {
    name: String,
    history: Vec<Step>,
}

impl Workbench {
    pub fn new(name: &str) -> Self {
        Workbench {
            name: name.to_owned(),
            history: vec![],
        }
    }

    pub fn get_sketch_mut(&mut self, name: &str) -> Option<&mut Sketch> {
        for step in self.history.iter_mut() {
            match &mut step.data {
                StepData::Sketch {
                    plane_name: _,
                    width: _,
                    height: _,
                    sketch,
                } => {
                    if name == step.name {
                        return Some(sketch);
                    }
                }
                _ => {}
            }
        }
        None
    }

    pub fn add_defaults(&mut self) {
        self.history.push(Step::new_plane("Top", Plane::top()));
        self.history.push(Step::new_plane("Front", Plane::front()));
        self.history.push(Step::new_plane("Right", Plane::right()));
    }

    pub fn add_sketch(&mut self, name: &str, plane_name: &str) {
        self.history.push(Step::new_sketch(name, plane_name));
    }

    // pub fn add_extrusion(&mut self, name: &str, sketch_name: &str, height: f64) {
    //     let sketch = self.get_sketch_mut(sketch_name).unwrap();
    //     // let mut plane = Plane::top();
    //     // plane.origin.z = height;
    //     // self.history.push(Step::new_plane(name, plane));
    // }

    pub fn realize(&self, max_steps: u32) -> Realization {
        let mut realized = Realization::new();
        let max_steps = max_steps as usize; // just coerce the type once

        for (step_n, step) in self.history.iter().enumerate() {
            // println!("{:?}", step);
            if step_n >= max_steps {
                break;
            }

            let step_data = &step.data;
            // println!("{:?}", step_data);
            match step_data {
                StepData::Plane {
                    plane,
                    width,
                    height,
                } => {
                    let rp = RealPlane {
                        plane: plane.clone(),
                        width: *width,
                        height: *height,
                    };
                    realized.planes.insert(step.name.to_owned(), rp);
                }
                _ => println!("Unknown step type"),
            }
        }

        realized
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Realization {
    // a Realization is what you get if you apply the steps in a Workbench's
    // history and build a bunch of geometry
    pub planes: HashMap<String, RealPlane>,
}

impl Realization {
    pub fn new() -> Self {
        Realization {
            planes: HashMap::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Step {
    name: String,
    suppressed: bool,
    data: StepData,
}

impl Step {
    pub fn new_plane(name: &str, plane: Plane) -> Self {
        Step {
            name: name.to_owned(),
            suppressed: false,
            data: StepData::Plane {
                plane,
                height: 1.0,
                width: 1.0,
            },
        }
    }

    pub fn new_sketch(name: &str, plane_name: &str) -> Self {
        Step {
            name: name.to_owned(),
            suppressed: false,
            data: StepData::Sketch {
                plane_name: plane_name.to_owned(),
                width: 0.5,
                height: 0.5,
                sketch: Sketch::new(),
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum StepData {
    Plane {
        plane: Plane,
        width: f64,
        height: f64,
    },
    Sketch {
        plane_name: String,
        width: f64,
        height: f64,
        sketch: Sketch,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plane {
    origin: Point3,
    primary: Vector3,
    secondary: Vector3,
    tertiary: Vector3, // aka Normal
}

impl Plane {
    /*

    z    y

    ^   ^
    |  /
    | /
    |/
    |-------->  x

    So "front" is xz plane with -y normal
    and "top" is xy plane with z normal
    and "right" is yz plane with x normal

     */

    pub fn new(origin: Point3, primary: Vector3, secondary: Vector3, tertiary: Vector3) -> Self {
        Plane {
            origin,
            primary,
            secondary,
            tertiary,
        }
    }

    pub fn front() -> Self {
        Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            primary: Vector3::new(1.0, 0.0, 0.0),
            secondary: Vector3::new(0.0, 0.0, 1.0),
            tertiary: Vector3::new(0.0, -1.0, 0.0),
        }
    }

    pub fn top() -> Self {
        Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            primary: Vector3::new(1.0, 0.0, 0.0),
            secondary: Vector3::new(0.0, 1.0, 0.0),
            tertiary: Vector3::new(0.0, 0.0, 1.0),
        }
    }

    pub fn right() -> Self {
        Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            primary: Vector3::new(0.0, 1.0, 0.0),
            secondary: Vector3::new(0.0, 0.0, 1.0),
            tertiary: Vector3::new(1.0, 0.0, 0.0),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3 { x, y, z }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Point3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point3 { x, y, z }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealPlane {
    pub plane: Plane,
    pub width: f64,
    pub height: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_project() {
        let mut p = Project::new("Test Project");
        p.add_defaults();
        println!("{:?}", p);
    }
}

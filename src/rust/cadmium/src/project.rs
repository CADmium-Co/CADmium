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
    fn empty_to_svg() {
        let mut p = Project::new("First Project");
        p.add_defaults();
        let wb = p.workbenches.get_mut(0).unwrap();
        wb.add_sketch("Sketch1", "Top");
        let sketch = wb.get_sketch_mut("Sketch1").unwrap();

        sketch.save_svg("test_svgs/empty.svg");
    }

    #[test]
    fn no_rings_to_svg() {
        let mut p = Project::new("First Project");
        p.add_defaults();
        let wb = p.workbenches.get_mut(0).unwrap();
        wb.add_sketch("Sketch1", "Top");
        let sketch = wb.get_sketch_mut("Sketch1").unwrap();

        let center = sketch.add_point(0.0, 0.0);
        let right = sketch.add_point(1.0, 0.0);
        let top = sketch.add_point(0.0, 1.0);
        let left = sketch.add_point(-1.0, 0.0);
        let bottom = sketch.add_point(0.0, -1.0);

        sketch.add_segment(center, right);
        sketch.add_segment(center, top);
        sketch.add_segment(center, left);
        sketch.add_segment(center, bottom);

        sketch.save_svg("test_svgs/no_rings.svg");
    }

    #[test]
    fn circle_to_svg() {
        let mut p = Project::new("First Project");
        p.add_defaults();
        let wb = p.workbenches.get_mut(0).unwrap();
        wb.add_sketch("Sketch1", "Top");
        let sketch = wb.get_sketch_mut("Sketch1").unwrap();

        let id0 = sketch.add_point(1.0, 0.0);
        sketch.add_circle(id0, 1.0);

        sketch.save_svg("test_svgs/circle.svg");
    }

    #[test]
    fn square_to_svg() {
        let mut p = Project::new("First Project");
        p.add_defaults();
        let wb = p.workbenches.get_mut(0).unwrap();
        wb.add_sketch("Sketch1", "Top");
        let sketch = wb.get_sketch_mut("Sketch1").unwrap();

        let id0 = sketch.add_point(0.0, 0.0);
        let id1 = sketch.add_point(1.0, 0.0);
        let id2 = sketch.add_point(1.0, 1.0);
        let id3 = sketch.add_point(0.0, 1.0);

        sketch.add_segment(id0, id1);
        sketch.add_segment(id1, id2);
        sketch.add_segment(id2, id3);
        sketch.add_segment(id3, id0);

        sketch.save_svg("test_svgs/square.svg");
    }

    #[test]
    fn rounded_square_to_svg() {
        let mut p = Project::new("First Project");
        p.add_defaults();
        let wb = p.workbenches.get_mut(0).unwrap();
        wb.add_sketch("Sketch1", "Top");
        let sketch = wb.get_sketch_mut("Sketch1").unwrap();

        let a = sketch.add_point(0.25, 0.0);
        let b = sketch.add_point(0.75, 0.0);
        let c = sketch.add_point(1.0, 0.25);
        let d = sketch.add_point(1.0, 0.75);
        let e = sketch.add_point(0.75, 1.0);
        let f = sketch.add_point(0.25, 1.0);
        let g = sketch.add_point(0.0, 0.75);
        let h = sketch.add_point(0.0, 0.25);
        let i = sketch.add_point(0.75, 0.25);
        let j = sketch.add_point(0.75, 0.75);
        let k = sketch.add_point(0.25, 0.75);
        let l = sketch.add_point(0.25, 0.25);

        sketch.add_segment(a, b);
        sketch.add_arc(i, b, c);
        sketch.add_segment(c, d);
        sketch.add_arc(j, d, e);
        sketch.add_segment(e, f);
        sketch.add_arc(k, f, g);
        sketch.add_segment(g, h);
        sketch.add_arc(l, h, a);

        sketch.save_svg("test_svgs/rounded_square.svg");
    }

    #[test]
    fn square_with_hole_to_svg() {
        let mut p = Project::new("First Project");
        p.add_defaults();
        let wb = p.workbenches.get_mut(0).unwrap();
        wb.add_sketch("Sketch1", "Top");
        let sketch = wb.get_sketch_mut("Sketch1").unwrap();

        let a = sketch.add_point(0.0, 0.0);
        let b = sketch.add_point(1.0, 0.0);
        let c = sketch.add_point(1.0, 1.0);
        let d = sketch.add_point(0.0, 1.0);

        let e = sketch.add_point(0.25, 0.25);
        let f = sketch.add_point(0.75, 0.25);
        let g = sketch.add_point(0.75, 0.75);
        let h = sketch.add_point(0.25, 0.75);

        sketch.add_segment(a, b);
        sketch.add_segment(b, c);
        sketch.add_segment(c, d);
        sketch.add_segment(d, a);

        sketch.add_segment(e, f);
        sketch.add_segment(f, g);
        sketch.add_segment(g, h);
        sketch.add_segment(h, e);

        sketch.save_svg("test_svgs/square_with_hole.svg");
    }

    #[test]
    fn square_with_circular_hole_to_svg() {
        let mut p = Project::new("First Project");
        p.add_defaults();
        let wb = p.workbenches.get_mut(0).unwrap();
        wb.add_sketch("Sketch1", "Top");
        let sketch = wb.get_sketch_mut("Sketch1").unwrap();

        let a = sketch.add_point(0.0, 0.0);
        let b = sketch.add_point(1.0, 0.0);
        let c = sketch.add_point(1.0, 1.0);
        let d = sketch.add_point(0.0, 1.0);
        let center = sketch.add_point(0.5, 0.5);

        sketch.add_segment(a, b);
        sketch.add_segment(b, c);
        sketch.add_segment(c, d);
        sketch.add_segment(d, a);

        sketch.add_circle(center, 0.4);

        sketch.save_svg("test_svgs/square_with_circular_hole.svg");
    }
}

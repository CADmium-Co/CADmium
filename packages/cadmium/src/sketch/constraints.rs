use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::sketch::{Arc2, Circle2, IncrementingMap, Line2, Point2, Sketch};
use std::collections::HashMap;
use std::f64::consts::{PI, TAU};

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Constraint {
    SegmentLength {
        segment_id: u64,
        length: f64,
        normal_offset: f64,
        parallel_offset: f64,
        kp: f64, // kp is the proportional gain, the spring constant
        kd: f64, // kd is the derivative gain, the damping constant
        error: f64,
    },
    SegmentAngle {
        segment_id: u64,
        angle: f64,
        x_offset: f64,
        y_offset: f64,
        kp: f64,
        kd: f64,
        error: f64,
    },
    CircleDiameter {
        circle_id: u64,
        diameter: f64,
        angle_offset: f64,
        r_offset: f64,
        kp: f64,
        kd: f64,
        error: f64,
    },
    SegmentsEqual {
        segment_a_id: u64,
        segment_b_id: u64,
        kp: f64,
        kd: f64,
        error: f64,
    },
}

impl Sketch {
    pub fn add_segment_length_constraint(&mut self, segment_id: u64, length: f64) -> u64 {
        let mut constraint = Constraint::SegmentLength {
            segment_id,
            length,
            normal_offset: 0.15,
            parallel_offset: 0.0,
            kp: 2.0,
            kd: 0.3,
            error: 0.0,
        };

        let id = self.highest_constraint_id + 1;
        self.constraints.insert(id, constraint);
        self.highest_constraint_id += 1;

        let err = self.constraint_error(id);
        let c = self.constraints.get_mut(&id).unwrap();
        if let Constraint::SegmentLength { error, .. } = c {
            *error = err;
        }

        id
    }

    pub fn add_segment_vertical_constraint(&mut self, segment_id: u64) -> u64 {
        let current_angle = self.segment_angle(segment_id);
        if current_angle >= 0.0 {
            // it roughly points up
            self.add_segment_angle_constraint(segment_id, PI / 2.0)
        } else {
            self.add_segment_angle_constraint(segment_id, -PI / 2.0)
        }
    }

    pub fn add_segment_horizontal_constraint(&mut self, segment_id: u64) -> u64 {
        let current_angle = self.segment_angle(segment_id);
        if current_angle.abs() <= PI / 2.0 {
            // it roughly points right
            self.add_segment_angle_constraint(segment_id, 0.0)
        } else {
            self.add_segment_angle_constraint(segment_id, PI)
        }
    }

    pub fn add_segment_angle_constraint(&mut self, segment_id: u64, angle: f64) -> u64 {
        let constraint = Constraint::SegmentAngle {
            segment_id,
            angle,
            x_offset: 0.0,
            y_offset: 0.0,
            kp: 2.0,
            kd: 0.3,
            error: 0.0,
        };

        let id = self.highest_constraint_id + 1;
        self.constraints.insert(id, constraint);
        self.highest_constraint_id += 1;

        let err = self.constraint_error(id);
        let c = self.constraints.get_mut(&id).unwrap();
        if let Constraint::SegmentAngle { error, .. } = c {
            *error = err;
        }

        id
    }

    pub fn add_circle_diameter_constraint(&mut self, circle_id: u64, diameter: f64) -> u64 {
        let constraint = Constraint::CircleDiameter {
            circle_id,
            diameter,
            angle_offset: 3.0 * PI / 4.0,
            r_offset: 0.20,
            kp: 2.0,
            kd: 0.3,
            error: 0.0,
        };

        let id = self.highest_constraint_id + 1;
        self.constraints.insert(id, constraint);
        self.highest_constraint_id += 1;

        let err = self.constraint_error(id);
        let c = self.constraints.get_mut(&id).unwrap();
        if let Constraint::CircleDiameter { error, .. } = c {
            *error = err;
        }

        id
    }

    pub fn add_segments_equal_constraint(&mut self, segment_a_id: u64, segment_b_id: u64) -> u64 {
        let constraint = Constraint::SegmentsEqual {
            segment_a_id,
            segment_b_id,
            kp: 2.0,
            kd: 0.3,
            error: 0.0,
        };

        let id = self.highest_constraint_id + 1;
        self.constraints.insert(id, constraint);
        self.highest_constraint_id += 1;

        let err = self.constraint_error(id);
        let c = self.constraints.get_mut(&id).unwrap();
        if let Constraint::SegmentsEqual { error, .. } = c {
            *error = err;
        }

        id
    }

    pub fn compute_constraint_errors(&mut self) {
        let key_to_errors = self
            .constraints
            .iter()
            .map(|(k, _v)| (*k, self.constraint_error(*k)))
            .collect::<HashMap<_, _>>();
        for (constraint_id, err) in key_to_errors.iter() {
            let constraint = self.constraints.get_mut(constraint_id).unwrap();
            match constraint {
                Constraint::SegmentLength { error, .. } => {
                    *error = *err;
                }
                Constraint::CircleDiameter { error, .. } => {
                    *error = *err;
                }
                Constraint::SegmentAngle { error, .. } => {
                    *error = *err;
                }
                Constraint::SegmentsEqual { error, .. } => {
                    *error = *err;
                }
            }
        }
    }

    pub fn constraint_error(&self, constraint_id: u64) -> f64 {
        let constraint = self.constraints.get(&constraint_id).unwrap();
        let value = self.constraint_value(constraint_id);
        match constraint {
            Constraint::SegmentLength { length, .. } => value - length,
            Constraint::CircleDiameter { diameter, .. } => value - diameter,
            Constraint::SegmentAngle { angle, .. } => value - angle,
            Constraint::SegmentsEqual { .. } => value,
        }
    }

    pub fn constraint_value(&self, constraint_id: u64) -> f64 {
        let constraint = self.constraints.get(&constraint_id).unwrap();
        match constraint {
            Constraint::SegmentLength {
                segment_id, length, ..
            } => {
                let segment = self.line_segments.get(&segment_id).unwrap();
                let start = self.points.get(&segment.start).unwrap();
                let end = self.points.get(&segment.end).unwrap();
                start.distance_to(end)
            }

            Constraint::CircleDiameter {
                circle_id,
                diameter,
                ..
            } => {
                let circle = self.circles.get(&circle_id).unwrap();
                circle.radius * 2.0
            }

            Constraint::SegmentAngle {
                segment_id, angle, ..
            } => {
                let segment = self.line_segments.get(&segment_id).unwrap();
                let start = self.points.get(&segment.start).unwrap();
                let end = self.points.get(&segment.end).unwrap();
                start.angle_to(end)
            }

            Constraint::SegmentsEqual {
                segment_a_id,
                segment_b_id,
                ..
            } => {
                let a = self.segment_length(*segment_a_id);
                let b = self.segment_length(*segment_b_id);
                a - b
            }
        }
    }

    pub fn constraint_is_satisfied(&self, constraint_id: u64) -> bool {
        let tolerance = 1e-10;
        let constraint = self.constraints.get(&constraint_id).unwrap();
        let error = self.constraint_error(constraint_id);
        error.abs() < tolerance
    }

    pub fn all_constraints_are_satisfied(&self) -> bool {
        for (constraint_id, _constraint) in self.constraints.iter() {
            if !self.constraint_is_satisfied(*constraint_id) {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::project::Project;

    use super::*;

    #[test]
    fn segment_length_constraint() {
        let mut sketch = Sketch::new();

        let a = sketch.add_point(0.0, 0.0);
        let b = sketch.add_point(1.0, 0.0);

        let segment_id = sketch.add_segment(a, b);

        let constraint_id = sketch.add_segment_length_constraint(segment_id, 2.0);

        assert!(sketch.solve(1000));
        println!("Segment length: {}", sketch.segment_length(segment_id));
        assert!(sketch.constraint_is_satisfied(constraint_id));
    }

    #[test]
    fn segment_angle_constraint() {
        let mut sketch = Sketch::new();

        let a = sketch.add_point(0.0, 0.0);
        let b = sketch.add_point(1.0, 0.0);

        let segment_id = sketch.add_segment(a, b);

        let constraint_id = sketch.add_segment_angle_constraint(segment_id, PI / 4.0);

        sketch.solve(10000);

        assert!(sketch.constraint_is_satisfied(constraint_id));
    }
}

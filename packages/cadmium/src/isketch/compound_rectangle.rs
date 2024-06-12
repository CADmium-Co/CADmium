use std::cell::RefCell;
use std::rc::Rc;

use isotope::primitives::line::Line;
use isotope::primitives::point2::Point2;
use isotope::primitives::PrimitiveCell;
use serde::{Deserialize, Serialize};
use tsify_next::Tsify;

use crate::message::MessageHandler;
use crate::{interop, IDType};

use super::compound::{Compound, CompoundLike};
use super::ISketch;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Rectangle {
    start: Rc<RefCell<Point2>>,
    end: Rc<RefCell<Point2>>,
    new_points: [Rc<RefCell<Point2>>; 2],
    new_lines: [Rc<RefCell<Line>>; 4],
}

impl Rectangle {
    pub fn new(start: Rc<RefCell<Point2>>, end: Rc<RefCell<Point2>>) -> Self {
        let start_point = start.borrow().clone();
        let end_point = end.borrow().clone();

        let other_start = Rc::new(RefCell::new(Point2::new(start_point.x(), end_point.y())));
        let other_end = Rc::new(RefCell::new(Point2::new(end_point.x(), start_point.y())));

        let new_lines = [
            Rc::new(RefCell::new(Line::new(start.clone(), other_start.clone()))),
            Rc::new(RefCell::new(Line::new(other_start.clone(), end.clone()))),
            Rc::new(RefCell::new(Line::new(end.clone(), other_end.clone()))),
            Rc::new(RefCell::new(Line::new(other_end.clone(), start.clone()))),
        ];
        Self {
            start,
            end,
            new_points: [other_start, other_end],
            new_lines,
        }
    }
}

impl CompoundLike for Rectangle {
    fn references(&self) -> Vec<PrimitiveCell> {
        vec![
            PrimitiveCell::Point2(self.start.clone()),
            PrimitiveCell::Point2(self.end.clone()),
        ]
    }

    fn created_references(&self) -> Vec<PrimitiveCell> {
        vec![
            PrimitiveCell::Point2(self.new_points[0].clone()),
            PrimitiveCell::Point2(self.new_points[1].clone()),
            PrimitiveCell::Line(self.new_lines[0].clone()),
            PrimitiveCell::Line(self.new_lines[1].clone()),
            PrimitiveCell::Line(self.new_lines[2].clone()),
            PrimitiveCell::Line(self.new_lines[3].clone()),
        ]
    }
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(from_wasm_abi, into_wasm_abi)]
pub struct Add {
    pub start: IDType,
    pub end: IDType,
}

impl MessageHandler for Add {
    type Parent = Rc<RefCell<ISketch>>;
    fn handle_message(
        &self,
        sketch_ref: Rc<RefCell<ISketch>>,
    ) -> anyhow::Result<Option<(IDType, interop::Node)>> {
        let mut isketch = sketch_ref.borrow_mut();
        let mut sketch = isketch.sketch.borrow_mut();

        let start_point =
            if let PrimitiveCell::Point2(point) = sketch.get_primitive_by_id(self.start).unwrap() {
                point
            } else {
                return Err(anyhow::anyhow!("Start point is not a point"));
            };
        let end_point =
            if let PrimitiveCell::Point2(point) = sketch.get_primitive_by_id(self.end).unwrap() {
                point
            } else {
                return Err(anyhow::anyhow!("End point is not a point"));
            };

        let rectangle = Rectangle::new(start_point.clone(), end_point.clone());
        rectangle.populate_created_references(&mut sketch)?;
        drop(sketch);

        let compound = Rc::new(RefCell::new(Compound::Rectangle(rectangle)));
        let rectangle_id = isketch.compounds_next_id;
        isketch.compounds.insert(rectangle_id, compound.clone());
        isketch.compounds_next_id += 1;

        Ok(Some((rectangle_id, interop::Node::Compound(compound))))
    }
}

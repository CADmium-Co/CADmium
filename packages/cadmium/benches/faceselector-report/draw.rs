use svg::node::element::path::Data;
use svg::node::element::Path as SvgPath;

use cadmium::isketch::face::FaceSelector;
use cadmium::project::Project;
use cadmium::IDType;

pub const COLORS: [&str; 6] = ["#FF0000", "#00FF00", "#0000FF", "#FFFF00", "#FF00FF", "#00FFFF"];

pub fn draw_sketch_faces<T: FaceSelector + std::fmt::Debug>(p: &mut Project, selector: &Box<T>, sketch_id: IDType, name: String) {
    let wb_ref = p.get_workbench_by_id(0).unwrap();
    let wb = wb_ref.borrow();
    let sketch_ref = wb.get_sketch_by_id(sketch_id).unwrap();
    let sketch = sketch_ref.borrow();

    let all_faces = sketch.sketch().borrow().get_merged_faces();
    let faces = selector.get_selected_faces(&sketch);

    // viewBox is min-x, min-y, width, height
    let mut svg_doc = svg::Document::new().set("viewBox", (-50, -50, 100, 100));

    for (i, face) in all_faces.iter().enumerate() {
        let polygon = face.as_polygon();
        let color = COLORS[i % COLORS.len()];
        let selected = faces.contains(face);
        println!("Selected: {}", selected);

        svg_doc = svg_doc.add(draw_polygon(&polygon, color, selected));
    }

    svg::save(format!("bench-faceselector-report/{}.svg", name), &svg_doc).unwrap();
}

pub fn draw_polygon(polygon: &geo::Polygon<f64>, color: &str, selected: bool) -> SvgPath {
    let mut data = Data::new();

    for line in polygon.exterior().lines() {
        data = data.move_to((line.start.x, line.start.y));
        data = data.line_to((line.end.x, line.end.y));
    }

    for hole in polygon.interiors() {
        for line in hole.lines() {
            data = data.move_to((line.start.x, line.start.y));
            data = data.line_to((line.end.x, line.end.y));
        }
    }
    data = data.close();

    SvgPath::new()
        // TODO: Fill doesn't work!
        .set("fill", color)
        .set("fill-opacity", if selected { "0.5" } else { "0" })
        .set("stroke", color)
        .set("stroke-width", 1)
        .set("d", data)
}

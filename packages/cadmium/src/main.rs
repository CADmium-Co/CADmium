use truck_meshalgo::tessellation::{MeshableShape, MeshedShape};
use truck_modeling::builder::{rsweep, try_attach_plane, tsweep, vertex};
use truck_modeling::{Point3, Vector3};
use truck_polymesh::{obj, Rad};
use truck_shapeops::{and, or};

fn main() {
    // Make a cube with side length 100
    let origin = vertex(Point3::new(0.0, 0.0, 0.0));
    let x_axis = tsweep(&origin, Vector3::new(100.0, 0.0, 0.0));
    let xy_square = tsweep(&x_axis, Vector3::new(0.0, 100.0, 0.0));
    let cube = tsweep(&xy_square, Vector3::new(0.0, 0.0, 100.0));

    // Save it as an obj file
    // let mesh = cube.triangulation(0.01).to_polygon();
    // let file = std::fs::File::create("test_cube.obj").unwrap();
    // obj::write(&mesh, file).unwrap();

    // Make a cylinder that is centered at (50, 50) so it will interfere with the cube
    let point = vertex(Point3::new(104.0, 50.0, -20.0));
    let circle = rsweep(
        &point,
        Point3::new(80.0, 50.0, -20.0),
        Vector3::new(0.0, 0.0, 1.0),
        Rad(7.0),
    );
    let disk = try_attach_plane(&[circle]).unwrap();
    let cylinder = tsweep(&disk, Vector3::new(0.0, 0.0, 140.0));

    // save the cylinder to a file
    // let mesh = cylinder.triangulation(0.01).to_polygon();
    // let file = std::fs::File::create("test_cylinder.obj").unwrap();
    // obj::write(&mesh, file).unwrap();

    // Now we let's do the boolean operations!

    // let and_result = and(&cube, &cylinder, 1.0);
    // let mesh = and_result.unwrap().triangulation(0.01).to_polygon();
    // let file = std::fs::File::create("test_AND.obj").unwrap();
    // obj::write(&mesh, file).unwrap();
    // This results in the cylinder, but truncated. This is the region where the cylinder intersects the cube
    // Aka the region of space which is both inside the cube AND inside the cylinder

    let or_result = or(&cube, &cylinder, 0.5);
    let mesh = or_result.unwrap().triangulation(0.01).to_polygon();
    let file = std::fs::File::create("test_OR.obj").unwrap();
    obj::write(&mesh, file).unwrap();
    // This results in a cube on a stick, aka the union of the cube and the cylinder
    // Aka the region of space which is inside the cube OR inside the cylinder

    // let mut not_cylinder = cylinder.clone();
    // not_cylinder.not();
    // // not_cylinder is a weird thing...it's the entire universe _except_ the cylinder
    // let and_not_result = and(&cube, &not_cylinder, 1.0);
    // let mesh = and_not_result.unwrap().triangulation(0.01).to_polygon();
    // let file = std::fs::File::create("test_AND_NOT.obj").unwrap();
    // obj::write(&mesh, file).unwrap();
    // This results in a cube with a hole in it, aka the cube with the cylinder subtracted from it
    // Aka the region of space which is inside the cube AND NOT inside the cylinder
}

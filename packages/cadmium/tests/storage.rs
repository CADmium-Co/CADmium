use wasm_bindgen_test::*;

// mod helpers;
// use helpers::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_indexed_db() {
    let mut rp = cadmium::Project::new("Test Project");
    rp.save_to_indexed_db().unwrap();
    rp.set_name("Test Project 2".to_string());
    rp = cadmium::Project::load_from_indexed_db("Test Project").unwrap();

    assert_eq!(rp.name(), "Test Project");
}

use message::{Message, MessageResult};
use tsify_next::declare;
use wasm_bindgen::prelude::*;
extern crate console_error_panic_hook;

pub mod archetypes;
pub mod error;
pub mod isketch;
pub mod message;
pub mod project;
pub mod feature;
#[macro_use]
pub mod step;
pub mod workbench;

#[declare]
pub type IDType = u64;

pub const DB_NAME: &str = "cadmium";
pub const DB_VERSION: u32 = 1;

#[wasm_bindgen]
pub struct Project {
    native: project::Project,
}

#[wasm_bindgen]
impl Project {
    #[wasm_bindgen(constructor)]
    pub fn new(name: &str) -> Project {
        console_error_panic_hook::set_once();
        wasm_logger::init(wasm_logger::Config::default());

        Project {
            native: project::Project::new(name),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.native.name.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_name(&mut self, name: String) {
        self.native.name = name;
    }

    #[wasm_bindgen(getter)]
    pub fn json(&self) -> String {
        self.native.json()
    }

    #[wasm_bindgen]
    pub fn to_json(&self) -> String {
        self.native.json()
    }

    #[wasm_bindgen]
    pub fn from_json(json: String) -> Project {
        let p = project::Project::from_json(&json);
        Project { native: p }
    }

    #[wasm_bindgen]
    pub fn compute_constraint_errors(&mut self) {
        // self.native.compute_constraint_errors();
    }

    #[wasm_bindgen]
    pub fn get_workbench(&self, workbench_index: u32) -> workbench::Workbench {
        // TODO: Use get() and return a Result
        self.native.workbenches
            .get(workbench_index as usize)
            .unwrap()
            .borrow()
            .clone() // This single call polutes Clone derives for all MessageHandlers
    }

    #[wasm_bindgen]
    pub fn send_message(&mut self, message: &Message) -> MessageResult {
        // TODO: Move this to a MessageHandler trait during first stage indirection
        self.get_workbench(0).add_message_step(message);

        message.handle(&mut self.native).into()
    }

    #[wasm_bindgen]
    pub fn save_to_indexed_db(&self) -> Result<(), JsValue> {
        let idb = web_sys::window()
            .ok_or("No window object - is this running in a browser?")?
            .indexed_db()?
            .ok_or("No indexedDB object - is this running in a browser?")?;
        let db = idb.open_with_u32(DB_NAME, DB_VERSION)?;

        let tx = db.transaction()
            .ok_or("Could not open an indexed DB transaction")?
            .db()
            .transaction_with_str(&self.native.name)?;
        let store = tx.object_store(&self.native.name)?;

        let compressed = self.native.compressed();
        let compressed_str = String::from_utf8(compressed).unwrap();

        store.add_with_key(&compressed_str.into(), &self.native.name.clone().into())?;
        Ok(())
    }

    #[wasm_bindgen]
    pub fn load_from_indexed_db(name: &str) -> Result<Project, JsValue> {
        let idb = web_sys::window()
            .ok_or("No window object - is this running in a browser?")?
            .indexed_db()?
            .ok_or("No indexedDB object - is this running in a browser?")?;
        let db = idb.open_with_u32(DB_NAME, DB_VERSION)?;
        let tx = db.transaction()
            .ok_or("Could not open an indexed DB transaction")?
            .db()
            .transaction_with_str(&name)?;
        let store = tx.object_store(&name)?;

        let request = store.get_key(&name.into())?;
        let result = request.result()?.as_string().ok_or("Could not convert result to string")?;
        let data = result.as_bytes();
        let p = project::Project::from_compressed(&data);

        Ok(Project { native: p })
    }
}

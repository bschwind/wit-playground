use wasmtime::{
    component::{bindgen, Component, Linker},
    Config, Engine, Store,
};

bindgen!("model-world" in "model-api/wit");

struct CadHost {
    name: String,
}

// Imports into the world, like the `name` import for this world, are
// satisfied through traits.
impl ModelWorldImports for CadHost {
    fn print(&mut self, msg: String) -> Result<(), wasmtime::Error> {
        println!("{msg} from {}", self.name);
        Ok(())
    }
}

fn main() {
    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = Engine::new(&config).unwrap();

    let component = Component::from_file(&engine, "./model-code/my-component.wasm").unwrap();

    let mut linker = Linker::new(&engine);

    ModelWorld::add_to_linker(&mut linker, |state: &mut CadHost| state).unwrap();

    let mut store = Store::new(&engine, CadHost { name: "me".to_string() });

    let (bindings, _) = ModelWorld::instantiate(&mut store, &component, &linker).unwrap();

    bindings.call_init_model(&mut store).unwrap();
    bindings.call_run(&mut store).unwrap();
}

// Use a procedural macro to generate bindings for the world we specified in
// `host.wit`
wit_bindgen::generate!({
    // the name of the world in the `*.wit` input file
    world: "model-world",
    skip: ["init-model"],
});

pub trait Model: Send + Sync {
    fn new() -> Self
        where
            Self: Sized;

    fn create_model(&mut self) -> u32;
}

// Define a custom type and implement the generated `Guest` trait for it which
// represents implementing all the necessary exported interfaces for this
// component.
pub struct CadModelCode;

impl Guest for CadModelCode {
    fn run() {
        print("Hello, world!");
        let user_created_model = model().create_model();
        print(&format!("We got a user created model: {user_created_model}"));
    }
}

static mut MODEL: Option<Box<dyn Model>> = None;

#[doc(hidden)]
pub fn register_model(build_model: fn() -> Box<dyn Model>) {
    unsafe { MODEL = Some((build_model)()) }
}

fn model() -> &'static mut dyn Model {
    unsafe { MODEL.as_deref_mut().unwrap() }
}

#[macro_export]
macro_rules! register_model {
    ($model_type:ty) => {
        #[export_name = "init-model"]
        pub extern "C" fn __init_model() {
            model_api::register_model(|| {
                Box::new(<$model_type as model_api::Model>::new())
            });
        }
    };
}

// export! defines that the `CadModelCode` struct defined below is going to define
// the exports of the `world`, namely the `run` function.
export!(CadModelCode);

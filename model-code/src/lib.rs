use model_api::print;
use model_api::Model;

struct CableBracket {

}


impl Model for CableBracket {
    fn new() -> Self {
        Self {

        }
    }

    fn create_model(&mut self) -> u32 {
        print("oh heyyyyy");
        20
    }
}

model_api::register_model!(CableBracket);

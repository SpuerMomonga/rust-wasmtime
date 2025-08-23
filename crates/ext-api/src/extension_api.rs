use wit::*;

pub use wit::raykit::extension::commands::Command;

pub struct Commands {}

impl Commands {
    pub fn new() {}
}

impl Commands {
    pub fn register_command<F>(mut self, name: &str, invoke_handler: F) -> Self
    where
        F: Fn() -> String + Send + Sync + 'static,
    {
        self
    }

    pub fn execute_command(mut self) -> Self {
        self
    }
}

mod wit {
    wit_bindgen::generate!({path: "./wit/since_v0.0.1"});
}

wit::export!(Component);

struct Component;

impl wit::Guest for Component {
    fn execute_command(cmd: Command) {}
}

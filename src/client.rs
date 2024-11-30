pub struct ExampleSdk;

impl ExampleSdk {
    pub fn new() -> Self {
        ExampleSdk
    }

    pub fn greet(&self, name: &str) -> String {
        format!("Hello, {}!", name)
    }
}
use template_sdk::client::ExampleSdk;

fn main() {
    let sdk = ExampleSdk::new();
    let greeting = sdk.greet("Rust");
    eprintln!("{}", greeting); // Debug-friendly
}
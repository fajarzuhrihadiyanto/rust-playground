mod macros;
mod api;

#[tokio::main]
async fn main() {
    let crate_name = env!("CARGO_PKG_NAME");
    println!("I am in crate `{}`", crate_name);
}

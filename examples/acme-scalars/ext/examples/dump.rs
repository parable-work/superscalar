//! Print the assembled registry as JSON. scripts/smoke.sh reads the
//! canonical-name-to-id table from it, the way generated bindings would from a
//! codegen run.
//!
//!     cargo run -p acme-scalars --example dump > registry.json

fn main() {
    let dump = acme_scalars::registry().dump();
    println!(
        "{}",
        serde_json::to_string_pretty(&dump).expect("registry dump serializes")
    );
}

//! SigmaOS-configuration-management-engine — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_configuration_management_engine_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-configuration-management-engine: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-configuration-management-engine: ok");
}

//! Integration tests for SigmaOS-configuration-management-engine.

use sigmaos_configuration_management_engine_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}

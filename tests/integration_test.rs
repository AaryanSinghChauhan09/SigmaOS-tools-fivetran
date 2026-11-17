//! Integration tests for SigmaOS-tools-fivetran.

use sigmaos_tools_fivetran_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}

//! Integration tests for SigmaOS-backup-deduplication.

use sigmaos_backup_deduplication_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}

use clap::Parser;
use efivar::{efi::Variable, store::MemoryStore, VarReader};

use crate::{
    cli::{boot::tests::standard_setup, Command},
    exit_code::ExitCode,
};

#[test]
fn delete() {
    let manager = &mut MemoryStore::new();

    standard_setup(manager, 0x0001);

    assert_eq!(
        ExitCode::SUCCESS,
        crate::run(
            Command::parse_from(["efiboot", "boot", "delete", "1"]),
            manager,
        )
    );

    assert!(!manager.exists(&Variable::new("Boot0001")).unwrap());
}
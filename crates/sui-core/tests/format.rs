// Copyright (c) 2021, Facebook, Inc. and its affiliates
// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#[test]
#[cfg_attr(msim, ignore)]
fn test_format() {
    // If this test breaks and you intended a format change, you need to run to get the fresh format:
    // # cargo -q run --example generate-format -- print > crates/sui-core/tests/staged/sui.yaml

    let status = std::process::Command::new("cargo")
        .current_dir("..")
        .args(["run", "--example", "generate-format", "--"])
        .arg("test")
        .status()
        .expect("failed to execute process");
    assert!(
        status.success(),
        "\n\
If this test breaks and you intended a format change, you need to run to get the fresh format:\n\
cargo -q run --example generate-format -- print > crates/sui-core/tests/staged/sui.yaml\n\
        "
    );
}


use move_core_types::account_address::AccountAddress;
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::{TypeTag};
use serde_reflection::*;
use serde::Serialize;
#[derive(Serialize)]
struct FullName<'a> {
    first: &'a str,
    middle: Option<&'a str>,
    last: &'a str,
}
#[test]
fn test_format_error_unknown_format_in_container() -> Result<(), Error>{
    let mut tracer = Tracer::new(TracerConfig::default());
    let mut samples = Samples::new();
    tracer.trace_value(&mut samples, &FullName { first: "", middle: None, last: "" })?;
    assert_eq!(tracer.registry().unwrap_err(), Error::UnknownFormatInContainer("FullName".to_string()));
    Ok(())
}


#[derive(Serialize)]
struct TestStrucTag {
    pub address: AccountAddress,
    pub module: Identifier,
    pub name: Identifier,
    // alias for compatibility with old json serialized data.
    #[serde(rename = "type_args", alias = "type_params")]
    pub type_params: Vec<TypeTag>,
}
// #[test]
// fn test_format_UnknownFormatInContainer()-> Result<(), Error>{
//     let mut tracer = Tracer::new(TracerConfig::default());
//     let mut samples = Samples::new();
//     tracer.trace_type::<StructTag>(&samples)?;
//
//     // tracer.trace_value::<StructTag>(&mut samples, &StructTag {
//     //     address: AccountAddress::random(),
//     //     module: Identifier::new("module").unwrap(),
//     //     name: Identifier::new("name").unwrap(),
//     //     type_params: vec![TypeTag::Bool],
//     // })?;
//     tracer.registry().clone().unwrap();
//     //assert_eq!(tracer.registry().unwrap_err(), Error::UnknownFormatInContainer("FullName".to_string()));
//     Ok(())
// }
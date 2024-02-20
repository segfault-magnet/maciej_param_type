use std::collections::HashMap;

use fuel_abi_types::abi::program::{ProgramABI, TypeDeclaration};
use fuels::types::param_types::ParamType;

fn main() {
    let contents = std::fs::read_to_string("contract/out/debug/contract-abi.json").unwrap();
    let abi: ProgramABI = serde_json::from_str(&contents).unwrap();
    let my_complex_struct_decl = &abi
        .types
        .iter()
        .find(|t| t.type_field == "struct MyComplexStruct")
        .unwrap();

    let lookup = abi
        .types
        .iter()
        .map(|t| (t.type_id, t.clone()))
        .collect::<HashMap<usize, TypeDeclaration>>();

    let my_complex_struct_arg = &abi.functions[0].inputs[0];
    assert_eq!(
        my_complex_struct_arg.type_id,
        my_complex_struct_decl.type_id
    );

    let param_type = ParamType::try_from_type_application(my_complex_struct_arg, &lookup).unwrap();
    dbg!(param_type);
}

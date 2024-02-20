contract;

pub struct MyStruct {
    one: u64,
    two: u64,
}

pub struct MyOtherStruct {
    value: u32
}

pub struct MyComplexStruct {
    one: MyStruct,
    one_one: MyStruct,
    two: MyOtherStruct,
    three: u64,
    four: b256,
    five: bool,
    six: std::u256::U256,
}

abi MyContract {
    fn test_function(arg: MyComplexStruct);
}

impl MyContract for Contract {
    fn test_function(arg: MyComplexStruct){

    }
}

// use jito_restaking_client::instructions::{
//     InitializeConfigBuilder, InitializeNcnBuilder, InitializeOperatorBuilder,
// };

// use jito_weight_table_client::types::Weight;
use jito_weight_table_client::programs::JITO_WEIGHT_TABLE_ID;

pub fn main() {
    // let weight = Weight::new(1, 2).unwrap();
    println!("Hello, world! {:?}", JITO_WEIGHT_TABLE_ID);
}

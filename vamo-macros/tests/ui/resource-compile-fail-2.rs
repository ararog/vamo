use serde::{Deserialize, Serialize};
use vamo_macros::Resource;

#[derive(Debug, Serialize, Deserialize, Resource)]
#[name("tests")]
struct Test {
    #[rid]
    id: String
}

fn main() {}

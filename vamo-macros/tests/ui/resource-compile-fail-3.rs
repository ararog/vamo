use serde::{Deserialize, Serialize};
use vamo_macros::Resource;

#[derive(Debug, Serialize, Deserialize, Resource)]
struct Test {
    #[rid]
    id: String
}

fn main() {}

use serde::{Deserialize, Serialize};
use vamo_macros::Resource;

#[derive(Debug, Serialize, Deserialize, Resource)]
#[name("tests")]
#[body_type(JsonBody)]
struct Test {
    id: String
}

fn main() {}

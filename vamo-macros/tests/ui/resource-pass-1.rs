use deboa::serde::RequestBody;
use deboa_extras::serde::json::JsonBody;
use serde::{Deserialize, Serialize};
use vamo_macros::Resource;

#[derive(Debug, Serialize, Deserialize, Resource)]
#[name("tests")]
#[body_type(JsonBody)]
struct Test {
    #[rid]
    id: String
}

fn main() {}

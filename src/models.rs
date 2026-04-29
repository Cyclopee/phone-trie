use serde::Deserialize;
#[derive(Deserialize, Debug)]
pub struct Contact {
pub nb: String,
pub name: String,
}
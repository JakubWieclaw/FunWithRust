use serde::Serialize;

#[derive(Serialize)]
pub struct Pair{
    pub id: u32,
    pub number: u32
}
#[derive(Debug, PartialEq, Clone)]
pub struct Authentication {
    pub method: String,
    pub data: Vec<u8>,
}

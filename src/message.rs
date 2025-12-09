use crate::Address;

#[derive(Debug, Clone)]
pub struct Message {
    pub src: Address,
    pub dst: Address,
    pub payload: String,
}

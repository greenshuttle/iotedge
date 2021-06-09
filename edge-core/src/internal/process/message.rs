pub struct Message {
    header: MessageHeader,
    router: MessageRouter,
    body: Vec<u8>,
}

pub trait Type {}

pub struct MessageHeader {}

pub struct MessageRouter {}

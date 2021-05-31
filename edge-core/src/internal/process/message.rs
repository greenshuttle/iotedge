pub struct Message {
    header: MessageHeader,
    router: MessageRouter,
    body: dyn Type,
}

pub trait Type {}

pub struct MessageHeader {}

pub struct MessageRouter {}
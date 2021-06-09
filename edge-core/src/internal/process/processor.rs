use crate::internal::process::message::Message;
use tokio::sync::mpsc::{Receiver, Sender};

pub struct Processor {
    name: String,
    tx: Sender<Message>,
    rx: Receiver<Message>,
}

pub struct ProcessConfig {}

pub trait Process {
    fn init(&self);
    fn process(&self);
    fn start(&self);
}

pub struct Context {
    pub processors: Vec<Box<dyn Process>>,
}

impl Context {
    pub fn start_processors(&self) {
        for processor in self.processors.iter() {
            processor.start();
        }
    }
}

pub struct Shadow {
    processor: Processor,
}

impl Process for Processor {
    fn init(&self) {
        todo!()
    }

    fn process(&self) {
        todo!()
    }

    fn start(&self) {
        todo!()
    }
}

impl Process for Shadow {
    fn init(&self) {
        self.processor.init();
    }

    fn process(&self) {
        self.processor.process();
    }

    fn start(&self) {
        self.processor.start();
    }
}

#[cfg(test)]
#[test]
fn test() {
    let (tx, rx) = tokio::sync::mpsc::channel(1024);
}

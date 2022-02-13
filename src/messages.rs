//! Traits for messages to make them easier to send and receive.

use std::fmt::Debug;

#[derive(Debug)]
pub struct BoxAddr(pub Box<dyn TraitAddr>);

pub trait TraitAddr: Send + Debug {
    fn send(&self, msg: BoxMsg);
}

pub struct BoxMsg(Box<dyn TraitMsg>);

pub trait TraitMsg {
    fn moo(&self) -> String;
}

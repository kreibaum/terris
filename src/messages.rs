//! Traits for messages to make them easier to send and receive.

#[derive(Clone)]
pub struct BoxAddr(pub Box<dyn TraitAddr>);

pub trait TraitAddr: Send {
    fn send(&self, msg: BoxMsg);
    fn clone_box(&self) -> Box<dyn TraitAddr>;
}

impl Clone for Box<dyn TraitAddr> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

pub struct BoxMsg(Box<dyn TraitMsg>);

pub trait TraitMsg {
    fn moo(&self) -> String;
}

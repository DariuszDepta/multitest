use crate::{AcceptingModule, FailingModule, Module};

pub trait Ibc: Module {}

pub type IbcAcceptingModule = AcceptingModule;

impl Ibc for IbcAcceptingModule {}

pub type IbcFailingModule = FailingModule;

impl Ibc for IbcFailingModule {}

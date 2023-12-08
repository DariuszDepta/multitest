use crate::{AcceptingModule, FailingModule, Module};

pub trait Gov: Module {}

pub type GovAcceptingModule = AcceptingModule;

impl Gov for GovAcceptingModule {}

pub type GovFailingModule = FailingModule;

impl Gov for GovFailingModule {}

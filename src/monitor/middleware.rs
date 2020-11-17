use crate::runtime::ChainXClient;
use super::Monitor;
use crate::runtime::ChainXRuntime;
use subxt::system::System;

pub type Middleware = fn(ChainXClient, Monitor, <ChainXRuntime as System>::Header) -> ();



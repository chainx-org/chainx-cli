use crate::runtime::ChainXClient;
use super::Monitor;

pub type Middleware = fn(ChainXClient, Monitor) -> ();



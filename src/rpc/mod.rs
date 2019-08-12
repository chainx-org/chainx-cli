mod author;
mod chain;
mod chainx;
mod state;
mod system;

pub use self::author::AuthorRpc;
pub use self::chain::ChainRpc;
pub use self::chainx::ChainXRpc;
pub use self::state::StateRpc;
pub use self::system::SystemRpc;

pub trait Rpc: AuthorRpc + ChainRpc + ChainXRpc + StateRpc + SystemRpc {}

impl<T: AuthorRpc + ChainRpc + ChainXRpc + StateRpc + SystemRpc> Rpc for T {}

#[macro_use]
macro_rules! impl_rpc {
    (
        pub trait $trait_name:ident for $struct:ty {
            $(
                $rpc:expr => fn $fn_name:ident ( &$self:ident $(,)* $($param:ident: $param_ty:ty),* ) -> $return_ty:ty;
            )+
        }
    ) => {
        pub trait $trait_name {
            $(
                fn $fn_name( &$self, $($param: $param_ty),* ) -> $return_ty;
            )+
        }

        impl<T: web3::BatchTransport + 'static> $trait_name for $struct {
            $(
                fn $fn_name( &$self, $($param: $param_ty),* ) -> $return_ty {
                    $self.execute($rpc, vec![ $($crate::util::serialize($param)),* ])
                }
            )+
        }
    }
}

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

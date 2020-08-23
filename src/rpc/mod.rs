#[macro_use]
macro_rules! impl_rpc {
    (
        pub async trait $trait_name:ident for $struct:ty {
            $(
                $rpc:expr => fn $fn_name:ident ( &$self:ident $(,)* $($param:ident: $param_ty:ty),* ) -> $return_ty:ty;
            )+
        }
    ) => {
        #[async_trait::async_trait(?Send)]
        pub trait $trait_name {
            $(
                async fn $fn_name( &$self, $($param: $param_ty),* ) -> $return_ty;
            )+
        }

        #[async_trait::async_trait(?Send)]
        impl<T: web3::BatchTransport + 'static> $trait_name for $struct {
            $(
                async fn $fn_name( &$self, $($param: $param_ty),* ) -> $return_ty {
                    $self.execute($rpc, vec![ $($crate::util::serialize($param)),* ]).await
                }
            )+
        }
    }
}

mod author;
// mod call;
mod chain;
mod chainx;
mod state;
mod system;

pub use self::author::AuthorRpc;
// pub use self::call::ChainXCall;
pub use self::chain::ChainRpc;
pub use self::chainx::ChainXRpc;
// pub use self::state::storage::StorageRpc;
pub use self::state::StateRpc;
pub use self::system::SystemRpc;

pub trait Rpc: ChainRpc + ChainXRpc + StateRpc + SystemRpc {}
impl<T: ChainRpc + ChainXRpc + StateRpc + SystemRpc> Rpc for T {}

// pub trait RpcAndCall: Rpc + StorageRpc + ChainXCall {}
// impl<T: Rpc + StorageRpc + ChainXCall> RpcAndCall for T {}

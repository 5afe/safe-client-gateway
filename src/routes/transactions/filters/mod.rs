pub mod module;
pub mod multisig;
pub mod transfer;

pub trait QueryParam {
    fn as_query_param(&self) -> String;
}

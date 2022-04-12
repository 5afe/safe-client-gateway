pub mod module;
pub mod multisig;
pub mod transfer;

#[cfg(test)]
mod tests;

pub trait QueryParam {
    fn as_query_param(&self) -> String;
}

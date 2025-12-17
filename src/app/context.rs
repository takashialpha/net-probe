use crate::cli::RuntimeArgs;

#[derive(Debug)]
pub struct Context<C> {
    pub config: C,
    pub runtime: RuntimeArgs,
}

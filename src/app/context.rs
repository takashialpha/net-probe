use super::error::AppError;
use crate::config::TomlOptions;
use crate::signals::SignalHandler;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Context<C, A> {
    pub config: C,
    pub args: A,
    pub signals: Arc<SignalHandler>,
    config_path: Option<PathBuf>,
    config_opts: Option<TomlOptions>,
}

impl<C, A> Context<C, A> {
    pub(crate) fn new(
        config: C,
        args: A,
        signals: Arc<SignalHandler>,
        config_path: Option<PathBuf>,
        config_opts: Option<TomlOptions>,
    ) -> Self {
        Self {
            config,
            args,
            signals,
            config_path,
            config_opts,
        }
    }
}

impl<C, A> Context<C, A>
where
    C: serde::de::DeserializeOwned + serde::Serialize + Default,
{
    pub fn reload_config(&mut self) -> Result<(), AppError> {
        let (path, opts) = match (&self.config_path, &self.config_opts) {
            (Some(p), Some(o)) => (Some(p.clone()), o.clone()),
            _ => {
                tracing::debug!(
                    target: "app_base::config",
                    "reload requested but no config is configured"
                );
                return Ok(());
            }
        };

        self.config = crate::config::load::<C>(path, opts)?;
        tracing::debug!(target: "app_base::config", "configuration reloaded");
        Ok(())
    }
}

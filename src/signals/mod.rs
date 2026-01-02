use std::sync::{
    Arc, OnceLock,
    atomic::{AtomicBool, Ordering},
};
use tokio::runtime::Handle;
use tokio::signal::unix::{SignalKind, signal};
use tokio::sync::Notify;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Signal {
    Shutdown,
    Reload,
}

#[derive(Debug, Clone)]
pub struct SignalHandler {
    shutdown: Arc<Notify>,
    reload: Arc<Notify>,
    shutdown_flag: Arc<AtomicBool>,
    installed: Arc<OnceLock<()>>,
}

impl SignalHandler {
    pub fn new() -> Self {
        Self {
            shutdown: Arc::new(Notify::new()),
            reload: Arc::new(Notify::new()),
            shutdown_flag: Arc::new(AtomicBool::new(false)),
            installed: Arc::new(OnceLock::new()),
        }
    }

    /// install signal listeners.
    /// # Panics
    /// panics if called outside a tokio runtime.
    /// panics if called more than once.
    pub fn install(&self) {
        self.installed
            .set(())
            .expect("SignalHandler::install called more than once");

        let handle = Handle::try_current()
            .expect("SignalHandler::install must be called from within a Tokio runtime");

        let shutdown = self.shutdown.clone();
        let shutdown_flag = self.shutdown_flag.clone();

        handle.spawn(async move {
            let mut sigterm =
                signal(SignalKind::terminate()).expect("failed to install SIGTERM handler");
            let mut sigint =
                signal(SignalKind::interrupt()).expect("failed to install SIGINT handler");

            tokio::select! {
                _ = sigterm.recv() => {
                    shutdown_flag.store(true, Ordering::Relaxed);
                    shutdown.notify_waiters();
                }
                _ = sigint.recv() => {
                    shutdown_flag.store(true, Ordering::Relaxed);
                    shutdown.notify_waiters();
                }
            }
        });

        let reload = self.reload.clone();
        handle.spawn(async move {
            let mut sighup =
                signal(SignalKind::hangup()).expect("failed to install SIGHUP handler");

            while sighup.recv().await.is_some() {
                reload.notify_one();
            }
        });
    }

    pub async fn wait_shutdown(&self) {
        self.shutdown.notified().await;
    }

    pub async fn wait_reload(&self) {
        self.reload.notified().await;
    }

    pub fn is_shutdown(&self) -> bool {
        self.shutdown_flag.load(Ordering::Relaxed)
    }

    pub fn trigger_shutdown(&self) {
        self.shutdown_flag.store(true, Ordering::Relaxed);
        self.shutdown.notify_waiters();
    }

    pub fn trigger_reload(&self) {
        self.reload.notify_one();
    }
}

impl Default for SignalHandler {
    fn default() -> Self {
        Self::new()
    }
}

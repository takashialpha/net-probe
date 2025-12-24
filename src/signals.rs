use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
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
}

impl SignalHandler {
    pub fn new() -> Self {
        Self {
            shutdown: Arc::new(Notify::new()),
            reload: Arc::new(Notify::new()),
            shutdown_flag: Arc::new(AtomicBool::new(false)),
        }
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
        tracing::debug!(target: "app_base::signals", "shutdown triggered manually");
        self.shutdown_flag.store(true, Ordering::Relaxed);
        self.shutdown.notify_waiters();
    }

    pub fn trigger_reload(&self) {
        tracing::debug!(target: "app_base::signals", "reload triggered manually");
        self.reload.notify_one();
    }

    pub fn install(self) -> Self {
        let handler = self.clone();

        let shutdown = handler.shutdown.clone();
        let shutdown_flag = handler.shutdown_flag.clone();
        tokio::spawn(async move {
            let mut sigterm =
                signal(SignalKind::terminate()).expect("failed to install SIGTERM handler");
            let mut sigint =
                signal(SignalKind::interrupt()).expect("failed to install SIGINT handler");

            tokio::select! {
                _ = sigterm.recv() => {
                    tracing::debug!(target: "app_base::signals", signal = "SIGTERM", "received shutdown signal");
                    shutdown_flag.store(true, Ordering::Relaxed);
                    shutdown.notify_waiters();
                }
                _ = sigint.recv() => {
                    tracing::debug!(target: "app_base::signals", signal = "SIGINT", "received shutdown signal");
                    shutdown_flag.store(true, Ordering::Relaxed);
                    shutdown.notify_waiters();
                }
            }
        });

        let reload = handler.reload.clone();
        tokio::spawn(async move {
            let mut sighup =
                signal(SignalKind::hangup()).expect("failed to install SIGHUP handler");

            while sighup.recv().await.is_some() {
                tracing::debug!(target: "app_base::signals", signal = "SIGHUP", "received reload signal");
                reload.notify_one();
            }
        });

        handler
    }
}

impl Default for SignalHandler {
    fn default() -> Self {
        Self::new()
    }
}

use std::path::PathBuf;
use std::process::Stdio;
use tokio::process::Child;
use tokio::process::Command;

use super::EngineAdapter;
use crate::error::EngineError;

pub struct ClashMetaAdapter {
    exec_path: PathBuf,
    config_path: PathBuf,
    child: Option<Child>,
}

impl ClashMetaAdapter {
    pub fn new(exec_path: PathBuf, config_path: PathBuf) -> Self {
        Self {
            exec_path,
            config_path,
            child: None,
        }
    }
}

#[async_trait::async_trait]
impl EngineAdapter for ClashMetaAdapter {
    fn name(&self) -> &'static str {
        "Clash.Meta"
    }

    async fn start(&mut self) -> Result<(), EngineError> {
        if self.child.is_some() {
            return Err(EngineError::AlreadyRunning);
        }

        let child = Command::new(&self.exec_path)
            .arg("-d")
            .arg(self.config_path.to_str().unwrap())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        self.child = Some(child);
        Ok(())
    }

    async fn stop(&mut self) -> Result<(), EngineError> {
        if let Some(mut child) = self.child.take() {
            child.kill().await?;
            return Ok(());
        }
        Err(EngineError::NotRunning)
    }

    fn is_running(&self) -> bool {
        self.child.as_ref().map(|c| c.id().is_some()).unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn start_stop_without_binary_fails() {
        let bin = PathBuf::from("clash-meta-mock-bin");
        let cfg = PathBuf::from("config.yaml");
        let mut adapter = ClashMetaAdapter::new(bin, cfg);
        let res = adapter.start().await;
        assert!(res.is_err());
        assert!(!adapter.is_running());
    }
} 
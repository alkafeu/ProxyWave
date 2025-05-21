pub mod clash_meta;

use crate::error::EngineError;

pub trait EngineAdapter {
    fn name(&self) -> &'static str;

    /// Запускает движок в фоновом процессе.
    async fn start(&mut self) -> Result<(), EngineError>;

    /// Останавливает движок, если запущен.
    async fn stop(&mut self) -> Result<(), EngineError>;

    /// Проверяет, запущен ли движок.
    fn is_running(&self) -> bool;
} 
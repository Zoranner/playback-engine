pub mod project;
pub mod state;
pub mod cache;

// 重新导出主要类型
pub use project::ProjectManager;
pub use state::{StateManager, AppState, PlaybackState};
pub use cache::CacheManager;

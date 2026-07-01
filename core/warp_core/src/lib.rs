// Re-export all base modules from warp_core_base
pub use warp_core_base::app_id;
pub use warp_core_base::assertions;
pub use warp_core_base::channel;
pub use warp_core_base::command;
pub use warp_core_base::context_flag;
pub use warp_core_base::errors;
pub use warp_core_base::features;
#[cfg(target_os = "macos")]
pub use warp_core_base::macos;
pub use warp_core_base::paths;
pub use warp_core_base::platform;
pub use warp_core_base::safe_log;
pub use warp_core_base::host_id;
pub use warp_core_base::session_id;
pub use warp_core_base::{AppId, HostId, SessionId};

// Re-export macros and other top-level items from warp_core_base
pub use warp_core_base::{report_error, report_if_error, safe_anyhow, safe_assert, safe_assert_eq, safe_debug, safe_eprintln, safe_error, safe_info, safe_warn};

// UI-dependent modules
#[cfg(feature = "ui")]
pub use settings;
#[cfg(feature = "ui")]
pub use settings::{
    define_setting, define_settings_group, implement_setting_for_enum, maybe_define_setting,
};

pub mod execution_mode;
pub mod interval_timer;
pub mod operating_system_info;
pub mod semantic_selection;
pub mod sync_queue;
pub mod telemetry;
pub mod ui;
pub mod user_preferences;

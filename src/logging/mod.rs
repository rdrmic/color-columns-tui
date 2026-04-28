pub mod file_logger;

#[cfg(feature = "dev-console")]
pub mod dev_console;

// ============================================================================
// "dev-console" feature ENABLED: Working macros
// ============================================================================
#[cfg(feature = "dev-console")]
#[macro_export]
macro_rules! dev_gray { ($($arg:tt)*) => { $crate::logging::dev_console::send_log_message(format!($($arg)*), $crate::logging::dev_console::PrintColor::Gray) }; }
#[cfg(feature = "dev-console")]
#[macro_export]
macro_rules! dev_cyan { ($($arg:tt)*) => { $crate::logging::dev_console::send_log_message(format!($($arg)*), $crate::logging::dev_console::PrintColor::Cyan) }; }
#[cfg(feature = "dev-console")]
#[macro_export]
macro_rules! dev_yellow { ($($arg:tt)*) => { $crate::logging::dev_console::send_log_message(format!($($arg)*), $crate::logging::dev_console::PrintColor::Yellow) }; }
#[cfg(feature = "dev-console")]
#[macro_export]
macro_rules! dev_red { ($($arg:tt)*) => { $crate::logging::dev_console::send_log_message(format!($($arg)*), $crate::logging::dev_console::PrintColor::Red) }; }

// ============================================================================
// "dev-console" feature DISABLED: Dummy macros
// ============================================================================
#[cfg(not(feature = "dev-console"))]
#[macro_export]
macro_rules! dev_gray {
    ($($arg:tt)*) => {};
}
#[cfg(not(feature = "dev-console"))]
#[macro_export]
macro_rules! dev_cyan {
    ($($arg:tt)*) => {};
}
#[cfg(not(feature = "dev-console"))]
#[macro_export]
macro_rules! dev_yellow {
    ($($arg:tt)*) => {};
}
#[cfg(not(feature = "dev-console"))]
#[macro_export]
macro_rules! dev_red {
    ($($arg:tt)*) => {};
}

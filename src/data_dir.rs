use std::path::PathBuf;

use crate::{errors, terminal};

pub fn get_app_data_dir_path() -> Option<PathBuf> {
    match create_data_dir() {
        Ok(path) => Some(path),
        Err(e) => {
            eprint!(
                "\nWARNING!\n\nFailed to create or access application data directory: '{e}'.\nHighscores and logs will not be saved!\n\nPress any key to proceed... "
            );
            terminal::get_key_press();
            eprintln!();

            None
        }
    }
}

fn create_data_dir() -> Result<PathBuf, errors::Error> {
    let mut dir_path = create_target_os_data_dir_path()?;
    dir_path.push(env!("CARGO_PKG_NAME"));

    std::fs::create_dir_all(&dir_path)?;

    Ok(dir_path)
}

#[cfg(target_os = "windows")]
fn create_target_os_data_dir_path() -> Result<PathBuf, errors::Error> {
    let dir_path = get_root_path("LOCALAPPDATA")?;
    Ok(dir_path)
}

#[cfg(target_os = "macos")]
fn create_target_os_data_dir_path() -> Result<PathBuf, errors::Error> {
    let mut dir_path = get_root_path(c"HOME")?;
    dir_path.push("Library/Application Support");
    Ok(dir_path)
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
fn create_target_os_data_dir_path() -> Result<PathBuf, errors::Error> {
    get_root_path(c"XDG_STATE_HOME").or_else(|_| {
        let mut dir_path = get_root_path(c"HOME")?;
        dir_path.push(".local/state");
        Ok(dir_path)
    })
}

#[cfg(target_os = "windows")]
fn get_root_path(env_var: &str) -> Result<PathBuf, errors::Error> {
    match std::env::var_os(env_var) {
        Some(path) => Ok(PathBuf::from(path)),
        None => Err(std::io::Error::from(std::io::ErrorKind::NotFound).into()),
    }
}

#[cfg(not(target_os = "windows"))]
#[allow(unsafe_code)]
fn get_root_path(env_var: &'static std::ffi::CStr) -> Result<PathBuf, errors::Error> {
    use std::{ffi::OsStr, os::unix::ffi::OsStrExt};

    unsafe extern "C" {
        fn getenv(name: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    }

    // SAFETY: `env_var` is NUL-terminated. The returned pointer is checked for
    // null and copied into an owned `PathBuf` before this function returns.
    let value = unsafe { getenv(env_var.as_ptr()) };
    if value.is_null() {
        return Err(std::io::Error::from(std::io::ErrorKind::NotFound).into());
    }

    // SAFETY: A non-null value returned by `getenv` points to a NUL-terminated
    // C string, provided the process environment is not concurrently mutated.
    let bytes = unsafe { std::ffi::CStr::from_ptr(value) }.to_bytes();
    Ok(PathBuf::from(OsStr::from_bytes(bytes)))
}

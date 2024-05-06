use std::env;

/// Returns a string with the current desktop environment or "Unknown" if the environment variable
/// is not set
pub fn desktop_environment() -> String {
    match env::var("XDG_CURRENT_DESKTOP") {
        Ok(de) => de,
        Err(_) => "Unknown".to_string(),
    }
}

/// Returns a string with the current session type (e.g. wayland, x11) or "Unknown" if the
/// environment variable is not set
pub fn session_type() -> String {
    match env::var("XDG_SESSION_TYPE") {
        Ok(session_type) => session_type,
        Err(_) => "Unknown".to_string(),
    }
}

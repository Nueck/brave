use brave_config::GLOBAL_CONFIG;

pub(crate) mod auth_middleware;
pub(crate) mod head_middleware;
pub(crate) mod init_middleware;

pub(crate) fn is_need_verification(path: &str) -> bool {
    !(path == format!("/{}/init", GLOBAL_CONFIG.interface.api_scope)
        || path == format!("/{}/init/state", GLOBAL_CONFIG.interface.api_scope)
        || path == format!("/{}/login", GLOBAL_CONFIG.interface.api_scope)
        || path == format!("/{}/register", GLOBAL_CONFIG.interface.api_scope)
        || path == format!("/{}/sendmail", GLOBAL_CONFIG.interface.api_scope)
        || path == format!("/{}/forget", GLOBAL_CONFIG.interface.api_scope)
        || path == format!("/{}/email-login", GLOBAL_CONFIG.interface.api_scope)
        || path == format!("/{}/upload/img", GLOBAL_CONFIG.interface.api_scope))
}

pub(crate) fn refresh_api(path: &str) -> bool {
    path == format!("/{}/updateToken", GLOBAL_CONFIG.interface.api_scope)
}

use serde::{Deserialize, Serialize};

/*权限信息*/
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthorityConfig {
    pub auth: Option<Vec<String>>,
    pub super_admin: Option<String>,
    pub admin: Option<String>,
}

impl AuthorityConfig {
    #[allow(unused)]
    fn new() -> AuthorityConfig {
        AuthorityConfig {
            /*默认值*/
            auth: Some(vec![
                "super".to_string(),
                "admin".to_string(),
                "user".to_string(),
            ]),
            super_admin: Some("super".to_string()),
            admin: Some("admin".to_string()),
        }
    }

    /*获取默认的*/
    pub fn get_authority_config(&self) -> Self {
        let mut auth_config = AuthorityConfig::new();

        if let Some(auth) = &self.auth {
            auth_config.auth = Some(auth.clone());
        }

        if let Some(admin) = &self.admin {
            auth_config.admin = Some(admin.clone());
        }

        if let Some(super_admin) = &self.super_admin {
            auth_config.super_admin = Some(super_admin.clone());
        }

        auth_config
    }
}

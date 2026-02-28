use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub rpc: RpcConfig,
}

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub bind_address: String,
    pub static_dir: String,
}

#[derive(Debug, Clone)]
pub struct RpcConfig {
    pub url: String,
    pub username: String,
    pub password: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            server: ServerConfig {
                bind_address: env::var("DASHBOOK_BIND")
                    .unwrap_or_else(|_| "0.0.0.0:3000".to_string()),
                static_dir: env::var("DASHBOOK_STATIC_DIR")
                    .unwrap_or_else(|_| "./static".to_string()),
            },
            rpc: RpcConfig {
                url: env::var("DASHBOOK_RPC_URL")
                    .unwrap_or_else(|_| "http://127.0.0.1:19998/".to_string()),
                username: env::var("DASHBOOK_RPC_USER")
                    .unwrap_or_else(|_| "dashrpc".to_string()),
                password: env::var("DASHBOOK_RPC_PASS")
                    .unwrap_or_else(|_| "password".to_string()),
            },
        }
    }
}

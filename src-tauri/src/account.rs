use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;
use thiserror::Error;
use async_imap::Session;
use async_native_tls::TlsConnector;
use std::net::TcpStream;

#[derive(Error, Debug)]
pub enum AccountError {
    #[error("IMAP connection failed: {0}")]
    ImapError(String),
    #[error("Account not found")]
    NotFound,
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

impl Serialize for AccountError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailAccount {
    pub id: String,
    pub name: String,
    pub email: String,
    pub imap_host: String,
    pub imap_port: u16,
    pub smtp_host: String,
    pub smtp_port: u16,
    pub username: String,
    pub password: String,
    pub use_ssl: bool,
}

impl EmailAccount {
    pub fn new(
        name: String,
        email: String,
        imap_host: String,
        imap_port: u16,
        smtp_host: String,
        smtp_port: u16,
        username: String,
        password: String,
        use_ssl: bool,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            email,
            imap_host,
            imap_port,
            smtp_host,
            smtp_port,
            username,
            password,
            use_ssl,
        }
    }
}

#[tauri::command]
pub async fn add_account(
    state: State<'_, crate::AppState>,
    name: String,
    email: String,
    imap_host: String,
    imap_port: u16,
    smtp_host: String,
    smtp_port: u16,
    username: String,
    password: String,
    use_ssl: bool,
) -> Result<EmailAccount, String> {
    tracing::info!("Adding account: {}", email);

    let account = EmailAccount::new(
        name,
        email,
        imap_host,
        imap_port,
        smtp_host,
        smtp_port,
        username,
        password,
        use_ssl,
    );

    let mut accounts = state.accounts.lock().map_err(|e| e.to_string())?;
    accounts.push(account.clone());

    tracing::info!("Account added successfully: {}", account.id);
    Ok(account)
}

#[tauri::command]
pub fn get_accounts(state: State<'_, crate::AppState>) -> Result<Vec<EmailAccount>, String> {
    let accounts = state.accounts.lock().map_err(|e| e.to_string())?;
    Ok(accounts.clone())
}

#[tauri::command]
pub fn delete_account(state: State<'_, crate::AppState>, account_id: String) -> Result<(), String> {
    tracing::info!("Deleting account: {}", account_id);

    let mut accounts = state.accounts.lock().map_err(|e| e.to_string())?;
    accounts.retain(|a| a.id != account_id);

    tracing::info!("Account deleted successfully");
    Ok(())
}

#[tauri::command]
pub async fn test_connection(
    imap_host: String,
    imap_port: u16,
    username: String,
    password: String,
    use_ssl: bool,
) -> Result<bool, String> {
    tracing::info!("Testing IMAP connection to {}:{}", imap_host, imap_port);

    let addr = format!("{}:{}", imap_host, imap_port);
    
    let stream = TcpStream::connect(&addr).map_err(|e| {
        tracing::error!("TCP connection failed: {}", e);
        format!("Connection failed: {}", e)
    })?;

    stream.set_read_timeout(Some(std::time::Duration::from_secs(30))).ok();
    stream.set_write_timeout(Some(std::time::Duration::from_secs(30))).ok();

    let tls = async_native_tls::TlsConnector::new().map_err(|e| e.to_string())?;
    
    let domain = &imap_host;
    
    let session_future = async {
        if use_ssl {
            let tls_stream = tls.connect(domain, stream).await.map_err(|e| {
                tracing::error!("TLS handshake failed: {}", e);
                format!("TLS handshake failed: {}", e)
            })?;
            
            let session = async_imap::Session::new(tls_stream).await;
            session
        } else {
            let session = async_imap::Session::new(stream).await;
            session
        }
    };

    let session = session_future.await.map_err(|e| {
        tracing::error!("IMAP session failed: {}", e);
        format!("IMAP connection failed: {}", e)
    })?;

    let session = session.login(&username, &password).await.map_err(|e| {
        tracing::error!("IMAP login failed: {}", e);
        format!("Login failed: {}", e)
    })?;

    session.logout().await.ok();

    tracing::info!("IMAP connection test successful");
    Ok(true)
}

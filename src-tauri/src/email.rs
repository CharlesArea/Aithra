use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use mail_parser::MessageParser;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailFolder {
    pub name: String,
    pub path: String,
    pub unread_count: u32,
    pub total_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailSummary {
    pub id: String,
    pub from: String,
    pub to: String,
    pub cc: Option<String>,
    pub subject: String,
    pub date: String,
    pub preview: String,
    pub read: bool,
    pub flagged: bool,
    pub has_attachments: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailContent {
    pub id: String,
    pub from: String,
    pub to: String,
    pub cc: Option<String>,
    pub bcc: Option<String>,
    pub subject: String,
    pub date: String,
    pub body_text: Option<String>,
    pub body_html: Option<String>,
    pub headers: HashMap<String, String>,
    pub attachments: Vec<EmailAttachment>,
    pub read: bool,
    pub flagged: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailAttachment {
    pub filename: String,
    pub content_type: String,
    pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendEmailRequest {
    pub account_id: String,
    pub from: String,
    pub to: String,
    pub cc: Option<String>,
    pub bcc: Option<String>,
    pub subject: String,
    pub body: String,
    pub is_html: bool,
}

fn parse_imap_date(date_str: &str) -> String {
    if let Ok(dt) = DateTime::parse_from_rfc2822(date_str) {
        dt.format("%Y-%m-%d %H:%M:%S").to_string()
    } else {
        date_str.to_string()
    }
}

#[tauri::command]
pub async fn get_folders(
    _account_id: String,
) -> Result<Vec<EmailFolder>, String> {
    tracing::info!("Getting folders for account");

    Ok(vec![
        EmailFolder {
            name: "Inbox".to_string(),
            path: "INBOX".to_string(),
            unread_count: 0,
            total_count: 0,
        },
        EmailFolder {
            name: "Sent".to_string(),
            path: "Sent".to_string(),
            unread_count: 0,
            total_count: 0,
        },
        EmailFolder {
            name: "Drafts".to_string(),
            path: "Drafts".to_string(),
            unread_count: 0,
            total_count: 0,
        },
        EmailFolder {
            name: "Trash".to_string(),
            path: "Trash".to_string(),
            unread_count: 0,
            total_count: 0,
        },
    ])
}

#[tauri::command]
pub async fn get_emails(
    _account_id: String,
    _folder: String,
    _page: u32,
    _page_size: u32,
) -> Result<Vec<EmailSummary>, String> {
    tracing::info!("Getting emails for folder");

    Ok(Vec::new())
}

#[tauri::command]
pub async fn get_email(
    _account_id: String,
    _email_id: String,
) -> Result<EmailContent, String> {
    tracing::info!("Getting email content");

    Err("Email not found".to_string())
}

#[tauri::command]
pub async fn send_email(
    _account_id: String,
    _from: String,
    _to: String,
    _cc: Option<String>,
    _subject: String,
    _body: String,
) -> Result<bool, String> {
    tracing::info!("Sending email");

    Err("SMTP not configured".to_string())
}

#[tauri::command]
pub async fn delete_email(
    _account_id: String,
    _email_id: String,
) -> Result<bool, String> {
    tracing::info!("Deleting email");

    Ok(true)
}

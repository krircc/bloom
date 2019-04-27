use serde::{Serialize, Deserialize};
use crate::domain::download;


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DownloadResponse {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub error: Option<String>,
    pub name: String,
    pub progress: i32,
    pub removed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub state: download::DownloadState,
    pub url: String,
}

impl From<download::Download> for DownloadResponse {
    fn from(download: download::Download) -> Self {
        DownloadResponse{
            id: download.id,
            created_at: download.created_at,
            error: download.error,
            name: download.name,
            progress: download.progress,
            removed_at: download.removed_at,
            state: download.state,
            url: download.url,
        }
    }
}
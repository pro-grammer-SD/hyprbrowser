use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Download {
    pub id: String,
    pub url: String,
    pub filename: String,
    pub status: DownloadStatus,
    pub progress: f32,
    pub size: u64,
    pub downloaded: u64,
    pub started_at: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum DownloadStatus {
    Pending,
    Downloading,
    Paused,
    Completed,
    Failed,
}

impl Download {
    // pub fn new(url: String) -> Self {
    //    let filename = url
    //        .split('/')
    //        .last()
    //        .unwrap_or("download")
    //        .to_string();
    //    
    //    Self {
    //        id: uuid::Uuid::new_v4().to_string(),
    //        url,
    //       filename,
    //        status: DownloadStatus::Pending,
    //        progress: 0.0,
    //        size: 0,
    //        downloaded: 0,
    //        started_at: chrono::Local::now().to_rfc3339(),
    //    }
    //}

    // pub fn download_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    //    let exec_dir = std::env::current_exe()?
    //        .parent()
    //        .ok_or("Cannot determine executable directory")?
    //        .to_path_buf();
    //    Ok(exec_dir.join("downloads"))
    // }
}

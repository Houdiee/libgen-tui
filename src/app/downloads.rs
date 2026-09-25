use std::sync::{Arc, Mutex, MutexGuard};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DownloadStatus {
    Pending,
    Completed,
    Failed,
}

#[derive(Debug, Clone)]
pub struct Download {
    pub title: String,
    pub md5: String,
    pub status: DownloadStatus,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Downloads {
    inner: Arc<Mutex<Vec<Download>>>,
}

impl Downloads {
    pub fn new() -> Self {
        Self::default()
    }

    fn lock(&self) -> MutexGuard<'_, Vec<Download>> {
        self.inner.lock().unwrap_or_else(|e| e.into_inner())
    }

    pub fn start(&self, title: &str, md5: &str) {
        let mut downloads = self.lock();

        match downloads.iter_mut().find(|d| d.md5 == md5) {
            Some(existing) => {
                existing.status = DownloadStatus::Pending;
                existing.error = None;
            }
            None => downloads.push(Download {
                title: title.to_string(),
                md5: md5.to_string(),
                status: DownloadStatus::Pending,
                error: None,
            }),
        }
    }

    pub fn complete(&self, md5: &str) {
        if let Some(download) = self.lock().iter_mut().find(|d| d.md5 == md5) {
            download.status = DownloadStatus::Completed;
            download.error = None;
        }
    }

    pub fn fail(&self, md5: &str, error: impl ToString) {
        if let Some(download) = self.lock().iter_mut().find(|d| d.md5 == md5) {
            download.status = DownloadStatus::Failed;
            download.error = Some(error.to_string());
        }
    }

    pub fn snapshot(&self) -> Vec<Download> {
        self.lock().clone()
    }
}

use crate::api::{
    delete::delete_file,
    download::download_file,
    files::{get_all_files, FileEntry},
    rename::rename_file,
    sys::{get_sys_info, SysEntry},
    upload::upload_file,
};
use crate::progress::Progress;
use dioxus::prelude::*;
use std::sync::Arc;

#[derive(Clone)]
pub enum Status {
    Idle,
    Busy(String, Option<Arc<Progress>>),
    Done(String),
    Failed(String),
}

#[derive(Clone, Copy)]
pub struct AppState {
    pub files: Signal<Vec<FileEntry>>,
    pub sys: Signal<Option<SysEntry>>,
    pub status: Signal<Status>,
    pub load_error: Signal<Option<String>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            files: Signal::new(Vec::new()),
            sys: Signal::new(None),
            status: Signal::new(Status::Idle),
            load_error: Signal::new(None),
        }
    }

    pub fn is_busy(&self) -> bool {
        matches!(*self.status.read(), Status::Busy(..))
    }

    pub async fn refresh_files(mut self) {
        match get_all_files().await {
            Ok(files) => {
                self.files.set(files);
                self.load_error.set(None);
            }
            Err(e) => self.load_error.set(Some(e.to_string())),
        }
    }

    pub async fn refresh_sys(mut self) {
        if let Ok(s) = get_sys_info().await {
            self.sys.set(Some(s));
        }
    }

    pub fn upload(mut self) {
        spawn(async move {
            let Some(handle) = rfd::AsyncFileDialog::new().pick_file().await else {
                return;
            };
            let path = handle.path().to_path_buf();
            let name = handle.file_name();

            let progress = Arc::new(Progress::default());
            self.status
                .set(Status::Busy(format!("Uploading {name}"), Some(progress.clone())));

            match upload_file(&path, progress).await {
                Ok(_) => {
                    self.refresh_files().await;
                    self.status.set(Status::Done(format!("Uploaded {name}")));
                }
                Err(e) => self.status.set(Status::Failed(format!("Upload failed: {e}"))),
            }
        });
    }

    pub fn download(mut self, name: String) {
        spawn(async move {
            let progress = Arc::new(Progress::default());
            self.status
                .set(Status::Busy(format!("Downloading {name}"), Some(progress.clone())));

            match download_file(&name, progress).await {
                Ok(path) => self
                    .status
                    .set(Status::Done(format!("Saved to {}", path.display()))),
                Err(e) => self.status.set(Status::Failed(format!("Download failed: {e}"))),
            }
        });
    }

    pub fn rename(mut self, old: String, new: String) {
        let new = new.trim().to_string();
        if new == old {
            return;
        }
        if new.is_empty() || new.contains('/') || new.contains('\\') {
            self.status.set(Status::Failed(format!("Invalid file name: {new:?}")));
            return;
        }
        if self.files.read().iter().any(|f| f.name == new) {
            self.status.set(Status::Failed(format!("{new} already exists")));
            return;
        }

        spawn(async move {
            self.status.set(Status::Busy(format!("Renaming {old} → {new}"), None));

            match rename_file(&old, &new).await {
                Ok(()) => {
                    if let Some(f) = self.files.write().iter_mut().find(|f| f.name == old) {
                        f.name = new.clone();
                    }
                    self.status.set(Status::Done(format!("Renamed {old} → {new}")));
                }
                Err(e) => self.status.set(Status::Failed(format!("Rename failed: {e}"))),
            }
        });
    }

    pub fn delete(mut self, name: String) {
        spawn(async move {
            self.status.set(Status::Busy(format!("Deleting {name}"), None));

            match delete_file(&name).await {
                Ok(()) => {
                    self.files.write().retain(|f| f.name != name);
                    self.status.set(Status::Done(format!("Deleted {name}")));
                }
                Err(e) => self.status.set(Status::Failed(format!("Delete failed: {e}"))),
            }
        });
    }
}

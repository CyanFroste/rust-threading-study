use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

#[derive(Debug)]
pub struct Downloader {
    // queue: Arc<Mutex<Vec<Download>>>,
    queue: Vec<Arc<Mutex<Download>>>,
}

impl<'a> Downloader {
    pub fn new() -> Self {
        Self {
            // queue: Arc::new(Mutex::new(vec![])),
            queue: vec![],
        }
    }

    pub fn add(&mut self, value: Download) {
        self.queue.push(Arc::new(Mutex::new(value)));
    }

    pub fn start(&self) {
        let mut handles = vec![];

        for entry in self.queue.iter() {
            let entry_clone = entry.clone();

            handles.push(thread::spawn(move || {
                thread::sleep(Duration::from_secs(2));
                entry_clone.lock().unwrap().status = DownloadStatus::Done;
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }

    pub fn show_progress(&self) {
        for entry in self.queue.iter() {
            let lock = entry.lock().unwrap();
            println!("{} | {:?}", lock.url, lock.status);
        }
    }
}

#[derive(Debug)]
enum DownloadError {
    HttpError,
    NetworkError,
}

#[derive(Debug)]
enum DownloadStatus {
    Pending,
    InProgress,
    Done,
    Error(DownloadError),
}

#[derive(Debug)]
pub struct Download {
    url: String,
    status: DownloadStatus,
}

impl Download {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            status: DownloadStatus::Pending,
        }
    }
}

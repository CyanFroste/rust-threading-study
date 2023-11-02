mod download_manager;
mod file_operations;

use download_manager::{Download, Downloader};

fn main() {
    // file_operations
    file_operations::read_write_to_same_file().expect(":)");

    // download_manager
    let mut downloader = Downloader::new();

    downloader.add(Download::new("https://www.example.com"));
    downloader.add(Download::new("https://www.placeholder.com"));
    downloader.start();
    downloader.show_progress();
}

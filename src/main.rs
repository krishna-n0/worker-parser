//first id placement of the document
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use std::thread;

fn unix_timestamp_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards")
        .as_secs()
}

fn write_timestamp_file(path: &str) {
    let ts = unix_timestamp_seconds();
    fs::write(path, format!("{}\n", ts)).expect("failed to write timestamp file");
}

fn write_name_file(path: &str, name: String) {
    fs::write(path, format!("{}\n", name)).expect("failed to write name file");
}

fn main() {
    let t1 = thread::spawn(|| {
        write_timestamp_file("timestamp.txt");
    });

    let name = "krishna".to_string();
    let t2 = thread::spawn(move || {
        write_name_file("name.txt", name);
    });

    t1.join().expect("thread 1 panicked");
    t2.join().expect("thread 2 panicked");
}

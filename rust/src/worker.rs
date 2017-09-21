use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn dir_sha1(dir: &Path) -> String {
    let mut sha1s = Vec::new();

    for entry in dir.read_dir().unwrap() {
        let path = entry.unwrap().path();

        if path.is_dir() {
            sha1s.push(::get_contents(&path.join(".sha1")));
        } else if let Some(extension) = path.extension() {
            if extension == "json.sha1" {
                sha1s.push(::get_contents(&path));
            }
        }
    }

    ::make_sha1(&::comma_separated_list(&sha1s))
}

fn process_leaf(path: &Path, root: &Path) -> String {
    let sha1;
    let sha1_file;
    if path.is_dir() {
        sha1 = dir_sha1(path);
        sha1_file = path.join(".sha1");
    } else {
        sha1 = ::make_sha1(&::get_contents(path));
        sha1_file = path.with_extension(format!("{}.sha1", path.extension().unwrap().to_str().unwrap()));
    }
    if sha1 != ::get_contents(&sha1_file) {
        ::write_contents(&sha1_file, &sha1);
    }

    if path != root {
        process_leaf(path.parent().unwrap(), root);
    }

    sha1
}

pub fn work(root: PathBuf, jobs: Arc<Mutex<Vec<PathBuf>>>) {
    let start = ::time::now();

    loop {
        let mut guard = match jobs.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        if !guard.is_empty() {
            let job = guard.pop().unwrap();
            let sha1 = process_leaf(&job, &root);
            println!(
                "root SHA1: {}, duration: {}s",
                sha1,
                (::time::now() - start).num_seconds()
            );
        }

        thread::sleep(Duration::new(0, 500));
    }
}
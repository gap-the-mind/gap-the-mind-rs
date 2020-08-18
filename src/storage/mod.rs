use git2::Repository;
use std::path::Path;

pub struct Store {
    repo: Repository,
}

impl Store {
    pub fn open(path: &Path) -> Store {
        let repo = match Repository::open(path) {
            Ok(repo) => repo,
            Err(e) => panic!("failed to open: {}", e),
        };
        Store { repo }
    }
}

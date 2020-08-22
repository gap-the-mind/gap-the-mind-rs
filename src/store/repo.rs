use super::{Store, StoreError};

use git2::Repository;
use std::path::Path;
use uuid::Uuid;

pub fn open_repo(path: &Path) -> Result<Repository, StoreError> {
    Repository::open(path)
        .or_else(|_| Repository::init(path))
        .or(Err(StoreError::OpenRepoError))
}

impl Store {
    pub fn list_all(&self) {
        println!("{:?}", self.repo.workdir());
    }
}

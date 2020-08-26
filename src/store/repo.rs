use super::model::Entity;
use super::{StorageContext, Store, StoreError};

use git2::{IndexAddOption, Repository, Signature};
use std::fs::File;
use std::io::prelude::Write;
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

    pub fn write_entity<'a>(&self, entity: &impl Entity<'a>) -> Result<(), StoreError> {
        let s = toml::to_string(entity).or(Err(StoreError::MarshallError))?;
        let filename = format!("{}.toml", entity.id());

        let path = self
            .repo
            .workdir()
            .map(|f| f.join(filename))
            .ok_or(StoreError::WriteError)?;

        let file = path.clone();

        let mut file = File::create(file).or(Err(StoreError::WriteError))?;

        file.write_all(s.as_bytes())
            .or(Err(StoreError::WriteError))?;

        let mut index = self.repo.index().or(Err(StoreError::CommitError))?;

        index
            .add_all(["*"].iter(), IndexAddOption::DEFAULT, None)
            .or(Err(StoreError::WriteError))?;
        index.write().or(Err(StoreError::WriteError))?;
        let oid = index.write_tree().or(Err(StoreError::CommitError))?;

        let tree = self.repo.find_tree(oid).or(Err(StoreError::CommitError))?;
        let signature = Signature::now(self.context.name.as_str(), self.context.email.as_str())
            .or(Err(StoreError::CommitError))?;

        match self.repo.commit(
            None,
            &signature,
            &signature,
            format!("Edited {}", entity.id()).as_str(),
            &tree,
            &[],
        ) {
            Ok(_) => Ok(()),
            Err(_) => Err(StoreError::CommitError),
        }
    }
}

use super::model::Entity;
use super::{Store, StoreError};
use std::fs::File;
use std::io::BufReader;

use git2::{IndexAddOption, Repository, Signature};
use std::fs;

use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};

pub fn open_repo(path: &Path) -> Result<Repository, StoreError> {
    Repository::open(path)
        .or_else(|_| Repository::init(path))
        .or(Err(StoreError::OpenRepoError))
}

impl Store {
    pub fn list_all(&self) {
        println!("{:?}", self.repo.workdir());
    }

    pub fn path(&self, filename: &str) -> Result<PathBuf> {
        self.repo
            .workdir()
            .map(|f| f.join(filename))
            .ok_or_else(|| anyhow!("Cannot write in bare repository"))
    }

    pub fn write_entity(&self, entity: &impl Entity) -> Result<()> {
        let s = serde_json::to_string(entity)?;
        let filename = format!("{}.json", entity.id());

        let path = self.path(filename.as_str())?;
        fs::write(path, s)?;

        let mut index = self.repo.index()?;

        index.add_all(["*"].iter(), IndexAddOption::DEFAULT, None)?;

        index.write()?;
        let oid = index.write_tree()?;

        let tree = self.repo.find_tree(oid)?;
        let signature = Signature::now(self.context.name.as_str(), self.context.email.as_str())?;

        let parent = self.repo.head().and_then(|p| p.peel_to_commit())?;

        self.repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            format!("Edited {}", entity.id()).as_str(),
            &tree,
            &[&parent],
        )?;

        Ok(())
    }

    pub fn read_entity<T>(&self, entity: &mut T) -> Result<T>
    where
        T: Entity,
    {
        let filename = format!("{}.json", entity.id());

        let path = self
            .repo
            .workdir()
            .map(|f| f.join(filename))
            .ok_or_else(|| anyhow!("Cannot write in bare repository"))?;

        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let entity = serde_json::from_reader(reader)?;

        Ok(entity)
    }
}

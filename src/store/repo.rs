use super::model::Entity;
use super::{Store, StoreError};

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
            .ok_or(anyhow!("Cannot write in bare repository"))
    }

    pub fn write_entity<'a>(&self, entity: &impl Entity<'a>) -> Result<()> {
        let s = toml::to_string(entity)?;
        let filename = format!("{}.toml", entity.id());

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

    pub fn read_entity<'a>(&self, entity: &mut impl Entity<'a>) -> Result<()> {
        let filename = format!("{}.toml", entity.id());

        let path = self
            .repo
            .workdir()
            .map(|f| f.join(filename))
            .ok_or(anyhow!("Cannot write in bare repository"))?;

        let content = fs::read_to_string(path)?;

        entity.deserialize(toml::Deserializer::new(content.as_str()))
    }
}

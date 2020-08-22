mod graphql;
mod model;
mod repo;

use git2::Repository;
use graphql::Schema;
use repo::open_repo;
use std::path::Path;

#[derive(Debug)]
pub enum StoreError {
    OpenRepoError,
    QueryError,
}

pub struct StorageContext {}

pub struct Store {
    schema: Schema,
    repo: Repository,
    context: StorageContext,
}

impl Store {
    pub fn new(path: &Path, context: StorageContext) -> Result<Store, StoreError> {
        let repo = open_repo(path)?;

        Ok(Store {
            schema: graphql::new_schema(),
            repo,
            context,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempdir::TempDir;

    #[test]
    fn open_repo() {
        let repo_dir = TempDir::new("storage").unwrap();
    }
}

struct SvnRepo {
    path: String,
}

impl SvnRepo {
    fn new(path: &str) -> Self {
        SvnRepo {
            path: path.to_string(),
        }
    }

    fn is_svn_repo(&self) -> bool {
        let output = Command::new("svn")
            .args(["info", &self.path])
            .output()
            .expect("Failed to execute svn info");

        output.status.success()
    }

    fn diff(&self, path: &str) -> std::io::Result<Vec<u8>> {
        let output = Command::new("svn")
            .args(["diff", path])
            .output()?;

        if !output.status.success() {
            eprintln!("Failed to execute svn diff on {}", path);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to execute svn diff"));
        }

        Ok(output.stdout)
    }
}
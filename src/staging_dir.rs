use std::path::{Path, PathBuf};

pub struct StagingDir {
    alive: bool,
    pub path: PathBuf
}
impl StagingDir {
    pub fn try_new(p: &Path) -> Result<Self,std::io::Error> {
        std::fs::create_dir_all(p)?;
        Ok(Self {
            alive: false,
            path: p.to_owned(),
        })
    }
    pub fn persist(&mut self) {
        self.alive = true;
    }
}
impl AsRef<Path> for StagingDir {
    fn as_ref(&self) -> &Path {
        &self.path
    }
}
impl Drop for StagingDir {
    fn drop(&mut self) {
        if !self.alive {
            std::fs::remove_dir_all(&self.path);
        }
    }
}

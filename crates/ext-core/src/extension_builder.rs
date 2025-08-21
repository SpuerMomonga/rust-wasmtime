use anyhow::{Context as _, Result};
use std::{
    fs,
    path::{Path, PathBuf},
};

use ext_manifest::ExtensionManifest;

pub struct ExtensionBuilder {
    cache_dir: PathBuf,
}

impl ExtensionBuilder {
    pub fn new(cache_dir: PathBuf) -> Self {
        Self { cache_dir }
    }
}

impl ExtensionBuilder {
    pub async fn compile_extension(
        &self,
        extension_dir: &Path,
        extension_manifest: &mut ExtensionManifest,
    ) -> Result<()> {
        if extension_dir.is_relative() {
            anyhow::bail!("extension dir {} is not an absolute path", extension_dir.display());
        }

        fs::create_dir_all(&self.cache_dir).context("failed to create cache dir")?;

        Ok(())
    }
}

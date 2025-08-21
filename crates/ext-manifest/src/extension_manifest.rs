use std::{ffi::OsStr, path::Path, sync::Arc};

use anyhow::{Context as _, Result};
use fs::Fs;
use schemars::JsonSchema;
use semver::Version;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionManifest {
    /// Unique identifier for the extension (e.g., 'raykit-clipboard').
    #[schemars(
        title = "The extension's name/identifier.",
        length(min = 3, max = 255),
        pattern(r"^[a-z0-9][a-z0-9-]*[a-z0-9]$")
    )]
    pub name: String,
    #[schemars(title = "The human-friendly extension's name.", length(min = 2, max = 255))]
    pub title: String,
    /// SemVer compatible version.
    #[schemars(title = "The extension's version.")]
    pub version: Version,
    /// The unique identifier of the publisher. Must be all lowercase and contain only letters and numbers.
    #[schemars(
        title = "The extension's publisher ID.",
        length(min = 3, max = 255),
        pattern(r"^[a-z0-9]+$")
    )]
    pub publisher: String,
    /// The open-source licenses accepted currently
    #[schemars(title = "The extension's license.")]
    pub license: LicenseManifest,
    /// A short description of what your extension is and does.
    #[schemars(title = "The extension's description.", length(min = 12, max = 2048))]
    pub description: String,
    /// Path to a 128x128px (or larger) icon for the extension.
    #[schemars(title = "Icon Path", pattern(r"\.(png|svg|jpg)$"))]
    pub icon: String,
    /// An object describing the extension's contributions.
    #[schemars(title = "The extension's contributions.")]
    pub contributes: ContributesManifest,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, PartialEq, Eq, Hash)]
pub enum LicenseManifest {
    #[serde(rename = "Apache-2.0")]
    // #[strum(serialize = "Apache-2.0")]
    Apache2_0,
    #[serde(rename = "BSD-2-Clause")]
    // #[strum(serialize = "BSD-2-Clause")]
    Bsd2Clause,
    #[serde(rename = "BSD-3-Clause")]
    // #[strum(serialize = "BSD-3-Clause")]
    Bsd3Clause,
    #[serde(rename = "GPL-2.0-only")]
    // #[strum(serialize = "GPL-2.0-only")]
    Gpl2_0,
    #[serde(rename = "GPL-3.0-only")]
    // #[strum(serialize = "GPL-3.0-only")]
    Gpl3_0,
    #[serde(rename = "ISC")]
    // #[strum(serialize = "ISC")]
    Isc,
    #[serde(rename = "LGPL-2.0-only")]
    // #[strum(serialize = "LGPL-2.0-only")]
    Lgpl2_0,
    #[serde(rename = "LGPL-3.0-only")]
    // #[strum(serialize = "LGPL-3.0-only")]
    Lgpl3_0,
    #[serde(rename = "MIT")]
    // #[strum(serialize = "MIT")]
    Mit,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ContributesManifest {
    /// List of all commands vended by this extensions.
    #[schemars(title = "Executable extension's commands", length(max = 100))]
    pub commands: Option<Vec<CommandManifest>>,
    pub command_palettes: Option<Vec<CommandPalettesManifest>>,
    pub icons: Option<Vec<IconsManifest>>,
    pub icon_themes: Option<Vec<IconThemesManifest>>,
    pub keybindings: Option<Vec<KeybindingManifest>>,
    pub preferences: Option<Vec<PreferenceManifest>>,
    pub themes: Option<Vec<ThemesManifest>>,
    pub views: Option<Vec<ViewManifest>>,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CommandManifest {}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CommandPalettesManifest {}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct IconsManifest {}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct IconThemesManifest {}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct KeybindingManifest {}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PreferenceManifest {}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ThemesManifest {}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ViewManifest {}

impl ExtensionManifest {
    pub async fn load(fs: Arc<dyn Fs>, extension_dir: &Path) -> Result<Self> {
        let extension_name = extension_dir
            .file_name()
            .and_then(OsStr::to_str)
            .context("invalid extension name")?;

        let extension_manifest_path = extension_dir.join("extension.json");
        if fs.is_file(&extension_manifest_path).await {
            let manifest_content = fs
                .load(&extension_manifest_path)
                .await
                .with_context(|| format!("failed to load {extension_name} extension.json"))?;
            serde_json::from_str::<ExtensionManifest>(&manifest_content)
                .with_context(|| format!("invalid extension.json for extension {extension_name}"))
        } else {
            anyhow::bail!("extension {} is missing required extension.json file", extension_name)
        }
    }
}

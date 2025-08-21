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
pub struct ContributesManifest {}

impl ExtensionManifest {
    // pub async fn load() -> Result<Self> {
    //     Ok()
    // }
}

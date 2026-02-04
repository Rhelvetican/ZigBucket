use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize, de::DeserializeOwned};

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct ZigVersionIndex<'a> {
    #[serde(borrow)]
    master: ZigIndex<'a>,
}

impl<'a> ZigVersionIndex<'a> {
    pub fn master(&self) -> ZigIndex<'a> { self.master }
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct ZigIndex<'a> {
    pub version: &'a str,
    pub date:    &'a str,

    #[serde(rename = "x86_64-windows")]
    winx64:     ZigIndexEntry<'a>,
    #[serde(rename = "aarch64-windows")]
    winaarch64: ZigIndexEntry<'a>,
    #[serde(rename = "x86-windows")]
    winx86:     ZigIndexEntry<'a>,
}

impl Display for ZigIndex<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "Version:       {}", self.version)?;
        writeln!(f, "Date:          {}", self.date)?;
        Ok(())
    }
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct ZigIndexEntry<'a> {
    #[serde(borrow)]
    tarball: &'a str,
    shasum:  &'a str,
}

impl<'a> ZigIndexEntry<'a> {
    pub fn tarball(&self) -> &'a str { self.tarball }

    pub fn shasum(&self) -> &'a str { self.shasum }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScoopEntry {
    pub version:      String,
    pub description:  String,
    pub homepage:     String,
    pub license:      String,
    pub suggest:      Suggest,
    pub architecture: Architecture,
    pub bin:          String,
    pub checkver:     Checkver,
    pub autoupdate:   Autoupdate,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Suggest {
    pub vcredist: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Architecture {
    #[serde(rename = "64bit")]
    pub n64bit: ArchitectureDetail<String>,
    #[serde(rename = "32bit")]
    pub n32bit: ArchitectureDetail<String>,
    pub arm64:  ArchitectureDetail<String>,
}

impl Architecture {
    pub fn update(&mut self, idx: ZigIndex<'_>) {
        self.n64bit.set_hash(String::from(idx.winx64.shasum()));
        self.n64bit.set_url(String::from(idx.winx64.tarball()));
        self.n32bit.set_hash(String::from(idx.winx86.shasum()));
        self.n32bit.set_url(String::from(idx.winx86.tarball()));
        self.arm64.set_hash(String::from(idx.winaarch64.shasum()));
        self.arm64.set_url(String::from(idx.winaarch64.tarball()));
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArchitectureDetail<H: Serialize + DeserializeOwned> {
    pub url:         String,
    #[serde(rename = "extract_dir")]
    pub extract_dir: String,
    #[serde(bound = "H: DeserializeOwned")]
    pub hash:        H,
}

impl<H: Serialize + DeserializeOwned> ArchitectureDetail<H> {
    #[inline]
    pub fn set_url(&mut self, url: String) { self.url = url; }

    #[inline]
    pub fn set_hash(&mut self, hash: H) { self.hash = hash; }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Checkver {
    pub url:      String,
    pub jsonpath: String,
    pub regex:    String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Autoupdate {
    pub architecture: AutoupdateArchitecture,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AutoupdateArchitecture {
    #[serde(rename = "64bit")]
    pub n64bit: ArchitectureDetail<Hash>,
    #[serde(rename = "32bit")]
    pub n32bit: ArchitectureDetail<Hash>,
    pub arm64:  ArchitectureDetail<Hash>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hash {
    pub url:      String,
    pub jsonpath: String,
}

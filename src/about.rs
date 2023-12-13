use serde::Serialize;
use typed_builder::TypedBuilder;

pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

#[derive(Debug, Clone, Serialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct AboutInfo {
    pub name: String,
    pub version: String,
    pub git_version: Option<String>,
    pub git_commitment_hash: Option<String>,
    pub authors: String,
    pub description: String,
    pub homepage: String,
    pub license: String,
    pub rustc_version: String,
    pub target: String,
    pub host: String,
    pub profile: String,
}

impl Default for AboutInfo {
    fn default() -> Self {
        AboutInfo::builder()
            .name(built_info::PKG_NAME)
            .version(built_info::PKG_VERSION)
            .git_version(built_info::GIT_VERSION.map(String::from))
            .git_commitment_hash(built_info::GIT_COMMIT_HASH.map(String::from))
            .authors(built_info::PKG_AUTHORS)
            .description(built_info::PKG_DESCRIPTION)
            .homepage(built_info::PKG_HOMEPAGE)
            .license(built_info::PKG_LICENSE)
            .rustc_version(built_info::RUSTC_VERSION)
            .target(built_info::TARGET)
            .host(built_info::HOST)
            .profile(built_info::PROFILE)
            .build()
    }
}

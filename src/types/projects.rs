use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MemberRole {
    /// The ID of the role
    pub id: String,
    /// The name of the role
    pub name: String,
    /// The flags for this role
    pub flags: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Member {
    /// The ID of the project member
    pub id: String,
    /// The date that this member joined the project
    pub joined_at: String,
    /// If user has multi-factor authentication enabled.
    pub mfa_enabled: bool,
    /// The role that this member has in a project
    pub role: MemberRole,
    // TODO: check these
    pub username: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Secret {
    pub id: String,
    pub name: String,
    pub digest: String,
    pub created_at: String,
}

// TODO: check this
#[derive(Debug, Serialize, Deserialize)]
pub struct Token {}

// TODO: check this
#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    /// The ID of the project
    pub id: String,
    /// The name of the project
    pub name: String,
    /// The tier this project is
    pub tier: String,
    /// The time this project was created at
    pub created_at: String,
    /// An icon for this project
    pub icon: Option<String>,
    /// The registry namespace for this project
    pub namespace: String,
    /// The type of this project. Either regular or personal
    #[serde(rename = "type")]
    pub project_type: ProjectType,
    pub default_quotas: DefaultQuotas,
    pub quota_overrides: QuotaOverrides,
    pub quota_usage: QuotaUsage,
}

// TODO: check this
#[derive(Debug, Serialize, Deserialize)]
pub enum ProjectTier {
    #[serde(rename = "free")]
    Free,
    #[serde(rename = "paid")]
    Paid,
}

// TODO: check this
#[derive(Debug, Serialize, Deserialize)]
pub enum ProjectType {
    /// A standard project type
    #[serde(rename = "regular")]
    Regular,
    /// A personal project are created when you register an account
    #[serde(rename = "personal")]
    Personal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DefaultQuotas {
    pub vcpu: i64,
    pub ram: i64,
    pub volume: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuotaOverrides {}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuotaUsage {
    pub vcpu: i64,
    pub ram: i64,
    pub volume: i64,
}

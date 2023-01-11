use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Deployment {
    pub id: String,
    pub name: String,
    #[serde(rename = "build_id")]
    pub build_id: String,
    #[serde(rename = "active_build")]
    pub active_build: String,
    #[serde(rename = "active_rollout")]
    pub active_rollout: String,
    #[serde(rename = "latest_rollout")]
    pub latest_rollout: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    pub entrypoint: Vec<String>,
    #[serde(rename = "target_container_count")]
    pub target_container_count: i64,
    #[serde(rename = "container_count")]
    pub container_count: i64,
    #[serde(rename = "running_container_count")]
    pub running_container_count: i64,
    pub metadata: String,
    pub config: Config,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub version: String,
    #[serde(rename = "restart_policy")]
    pub restart_policy: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub entrypoint: String,
    pub cmd: String,
    pub image: String,
    pub volume: String,
    // TODO: fix env
    pub env: Env,
    #[serde(rename = "container_strategy")]
    pub container_strategy: String,
    pub resources: Resources,
}

// TODO: fix env
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Env {
    pub additional_prop1: AdditionalProp1,
}

// TODO: fix env
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalProp1 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resources {
    pub cpu: i64,
    pub ram: String,
    pub vcpu: i64,
    pub vgpu: Vec<Vgpu>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vgpu {
    #[serde(rename = "type")]
    pub type_field: String,
    pub count: i64,
}
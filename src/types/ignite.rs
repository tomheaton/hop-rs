use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use crate::types::pipe::Region;

#[derive(Debug, Serialize, Deserialize)]
pub struct Resources {
    pub vcpu: i64,
    pub ram: String,
    // TODO: use array instead of vector?
    pub vgpu: Vec<Vgpu>,
}

impl Resources {
    pub fn new(
        vcpu: i64,
        ram: &str,
        vgpu: Vec<Vgpu>,
    ) -> Resources {
        return Resources {
            vcpu,
            // TODO: evaluate ram validity here as well?
            ram: ram.to_owned(),
            vgpu,
        };
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum VgpuType {
    #[serde(rename = "a400")]
    A400,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Vgpu {
    #[serde(rename = "type")]
    pub vgpu_type: VgpuType,
    pub count: i64,
}

impl Vgpu {
    pub fn new(
        vgpu_type: VgpuType,
        count: i64,
    ) -> Vgpu {
        return Vgpu {
            vgpu_type,
            count,
        };
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ContainerState {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "stopped")]
    Stopped,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "terminating")]
    Terminating,
    #[serde(rename = "exited")]
    Exited,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ContainerStrategy {
    #[serde(rename = "manual")]
    Manual,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RuntimeType {
    #[serde(rename = "ephemeral")]
    Ephemeral,
    #[serde(rename = "persistent")]
    Persistent,
    #[serde(rename = "stateful")]
    Stateful,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RestartPolicy {
    #[serde(rename = "never")]
    Never,
    #[serde(rename = "always")]
    Always,
    #[serde(rename = "on-failure")]
    OnFailure,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Auth {
    pub username: String,
    pub password: String,
}

impl Auth {
    pub fn new(
        username: &str,
        password: &str,
    ) -> Auth {
        return Auth {
            username: username.to_owned(),
            password: password.to_owned(),
        };
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageGHRepo {
    pub repo_id: i64,
    pub full_name: String,
    pub branch: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    pub name: Option<String>,
    pub auth: Option<Auth>,
    pub gh_repo: Option<ImageGHRepo>,
}

impl Image {
    pub fn new(
        name: Option<&str>,
        auth: Option<Auth>,
        gh_repo: Option<ImageGHRepo>,
    ) -> Image {
        return Image {
            name: name.map(|s| s.to_owned()),
            auth,
            gh_repo,
        };
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeploymentLog {
    pub nonce: String,
    pub timestamp: String,
    pub level: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GatewayType {
    #[serde(rename = "internal")]
    Internal,
    #[serde(rename = "external")]
    External,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DomainState {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "valid_cname")]
    ValidCName,
    #[serde(rename = "ssl_active")]
    SSLActive,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Domain {
    pub id: String,
    pub domain: String,
    pub state: DomainState,
    pub created_at: String,
    pub redirect: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GatewayProtocol {
    #[serde(rename = "http")]
    HTTP,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Gateway {
    pub id: String,
    pub gateway_type: GatewayType,
    pub name: String,
    pub protocol: Option<GatewayProtocol>,
    pub deployment_id: String,
    pub created_at: String,
    pub hopsh_domain: Option<String>,
    pub hopsh_domain_enabled: bool,
    pub internal_domain: Option<String>,
    pub target_port: Option<i64>,
    pub domains: Vec<Domain>,
}

impl Gateway {
    pub fn add_domain(&self) {
        println!("Adding gateway domain");
        panic!("not implemented!")
    }

    pub fn delete_domain(&self) {
        println!("Deleting gateway domain");
        panic!("not implemented!")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BuildSettings {
    pub root_directory: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeploymentMetaData {
    pub root_directory: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Deployment {
    pub id: String,
    pub name: String,
    pub container_count: i64,
    pub created_at: String,
    pub metadata: Option<String>,
    pub build_cache_enabled: bool,
    pub build_settings: BuildSettings,
}

impl Deployment {
    pub fn get_containers(&self) {
        println!("Getting deployment containers");
        panic!("not implemented!")
    }

    pub fn create_container(&self) {
        println!("Creating deployment container");
        panic!("not implemented!")
    }

    pub fn create_gateway(&self) {
        println!("Creating deployment gateway");
        panic!("not implemented!")
    }

    pub fn get_storage_stats(&self) {
        println!("Getting deployment storage stats");
        panic!("not implemented!")
    }

    pub fn delete(&self) {
        println!("Deleting deployment");
        panic!("not implemented!")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeploymentConfig {
    name: String,
    container_strategy: ContainerStrategy,
    #[serde(rename = "type")]
    runtime_type: RuntimeType,
    version: String,
    cmd: Option<Vec<String>>,
    image: Image,
    env: Option<HashMap<String, String>>,
    resources: Resources,
    restart_policy: RestartPolicy,
    volume: Option<String>,
    entrypoint: Option<Vec<String>>,
}

impl DeploymentConfig {
    pub fn new(
        name: &str,
        // container_strategy: ContainerStrategy,
        runtime_type: RuntimeType,
        // version: String,
        cmd: Option<Vec<String>>,
        image: Image,
        env: Option<HashMap<&str, &str>>,
        resources: Resources,
        restart_policy: RestartPolicy,
        volume: Option<&str>,
        entrypoint: Option<Vec<&str>>,
    ) -> DeploymentConfig {
        return DeploymentConfig {
            name: name.to_owned(),
            container_strategy: ContainerStrategy::Manual,
            runtime_type: runtime_type.clone(),
            version: "12-12-2022".to_owned(),
            cmd,
            image,
            env: env.map(|e| e.into_iter().map(|(k, v)| (k.to_owned(), v.to_owned())).collect()),
            resources,
            restart_policy,
            volume: match runtime_type {
                RuntimeType::Stateful => volume.map(|v| v.to_owned()),
                _ => None,
            },
            entrypoint: entrypoint.map(|e| e.into_iter().map(|e| e.to_owned()).collect()),
        };
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Container {
    pub id: String,
    pub created_at: String,
    pub region: Region,
    pub state: ContainerState,
    pub deployment_id: String,
    pub internal_ip: Option<String>,
    pub uptime: Option<String>,
    #[serde(rename = "type")]
    pub runtime_type: RuntimeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthCheck {
    pub id: String,
    pub protocol: HealthCheckProtocol,
    pub path: String,
    pub port: i64,
    pub interval: i64,
    pub timeout: i64,
    pub initial_delay: i64,
    pub max_retries: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum HealthCheckProtocol {
    #[serde(rename = "http")]
    HTTP,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthCheckConfig {
    pub protocol: HealthCheckProtocol,
    pub path: String,
    pub port: i64,
    pub interval: i64,
    pub timeout: i64,
    pub initial_delay: i64,
    pub max_retries: i64,
}

// OLD //

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Deployment {
//     pub id: String,
//     pub name: String,
//     #[serde(rename = "build_id")]
//     pub build_id: String,
//     #[serde(rename = "active_build")]
//     pub active_build: String,
//     #[serde(rename = "active_rollout")]
//     pub active_rollout: String,
//     #[serde(rename = "latest_rollout")]
//     pub latest_rollout: String,
//     #[serde(rename = "created_at")]
//     pub created_at: String,
//     pub entrypoint: Vec<String>,
//     #[serde(rename = "target_container_count")]
//     pub target_container_count: i64,
//     #[serde(rename = "container_count")]
//     pub container_count: i64,
//     #[serde(rename = "running_container_count")]
//     pub running_container_count: i64,
//     pub metadata: String,
//     pub config: Config,
// }
//
// impl Deployment {
//     pub fn new(
//         id: &str,
//         name: &str,
//         build_id: &str,
//         active_build: &str,
//         active_rollout: &str,
//         latest_rollout: &str,
//         created_at: &str,
//         entrypoint: Vec<&str>,
//         target_container_count: i64,
//         container_count: i64,
//         running_container_count: i64,
//         metadata: &str,
//         config: Option<Config>,
//     ) -> Deployment {
//         return Deployment {
//             id: id.to_owned(),
//             name: name.to_owned(),
//             build_id: build_id.to_owned(),
//             active_build: active_build.to_owned(),
//             active_rollout: active_rollout.to_owned(),
//             latest_rollout: latest_rollout.to_owned(),
//             created_at: created_at.to_owned(),
//             entrypoint: entrypoint.iter().map(|s| s.to_owned()).collect(),
//             target_container_count,
//             container_count,
//             running_container_count,
//             metadata: metadata.to_owned(),
//             config: config.unwrap_or_default(),
//         };
//     }
// }
//
// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Config {
//     pub version: String,
//     #[serde(rename = "restart_policy")]
//     pub restart_policy: String,
//     #[serde(rename = "type")]
//     pub type_field: String,
//     pub entrypoint: String,
//     pub cmd: String,
//     pub image: String,
//     pub volume: String,
//     // TODO: fix env
//     pub env: Env,
//     #[serde(rename = "container_strategy")]
//     pub container_strategy: String,
//     pub resources: Resources,
// }
//
// impl Config {
//     pub fn new(
//         version: &str,
//         restart_policy: &str,
//         type_field: &str,
//         entrypoint: &str,
//         cmd: &str,
//         image: &str,
//         volume: &str,
//         env: Env,
//         container_strategy: &str,
//         resources: Resources,
//     ) -> Config {
//         return Config {
//             version: version.to_owned(),
//             restart_policy: restart_policy.to_owned(),
//             type_field: type_field.to_owned(),
//             entrypoint: entrypoint.to_owned(),
//             cmd: cmd.to_owned(),
//             image: image.to_owned(),
//             volume: volume.to_owned(),
//             env,
//             container_strategy: container_strategy.to_owned(),
//             resources,
//         };
//     }
// }
//
// // TODO: fix env
// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Env {
//     pub additional_prop1: AdditionalProp1,
// }
//
// impl Env {
//     pub fn new(
//         additional_prop1: AdditionalProp1
//     ) -> Env {
//         return Env {
//             additional_prop1,
//         };
//     }
// }
//
// // TODO: fix env
// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct AdditionalProp1 {}
//
// impl AdditionalProp1 {
//     pub fn new() -> AdditionalProp1 {
//         return AdditionalProp1 {};
//     }
// }
//
// // #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Resources {
//     pub vcpu: i64,
//     pub cpu: i64,
//     pub ram: String,
//     pub vgpu: Vec<Vgpu>,
// }
//
// impl Resources {
//     pub fn new(
//         cpu: i64,
//         ram: &str,
//         vcpu: i64,
//         vgpu: Vec<Vgpu>,
//     ) -> Resources {
//         return Resources {
//             cpu,
//             ram: ram.to_owned(),
//             vcpu,
//             vgpu,
//         };
//     }
// }
//
// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Vgpu {
//     #[serde(rename = "type")]
//     pub type_field: String,
//     pub count: i64,
// }
//
// impl Vgpu {
//     pub fn new(
//         type_field: &str,
//         count: i64,
//     ) -> Vgpu {
//         return Vgpu {
//             type_field: type_field.to_owned(),
//             count,
//         };
//     }
// }

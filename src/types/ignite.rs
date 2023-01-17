use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub struct Resources {
    pub vcpu: i64,
    pub ram: String,
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
            ram: ram.to_owned(),
            vgpu,
        };
    }
}

enum VgpuType {
    A400,
}

pub struct Vgpu {
    pub vgpu_type: VgpuType,
    pub count: i64,
}

pub enum ContainerStrategy {
    #[serde(rename = "manual")]
    Manual,
}

pub enum RuntimeType {
    #[serde(rename = "ephemeral")]
    Ephemeral,
    #[serde(rename = "persistent")]
    Persistent,
    #[serde(rename = "stateful")]
    Stateful,
}

pub enum RestartPolicy {
    #[serde(rename = "never")]
    Never,
    #[serde(rename = "always")]
    Always,
    #[serde(rename = "on-failure")]
    OnFailure,
}

pub struct Image {
    name: String,
    // auth: Option<String>,
}

impl Image {
    pub fn new(
        name: &str
    ) -> Image {
        return Image {
            name: name.to_owned(),
        };
    }
}

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, CreateConstructor)]
#[derive(Debug)]
pub struct DeploymentConfig {
    name: String,
    container_strategy: ContainerStrategy,
    runtime_type: RuntimeType,
    version: String,
    cmd: Option<Vec<String>>,
    image: Image,
    env: Option<HashMap<String, String>>,
    resources: String,
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
        resources: &str,
        restart_policy: RestartPolicy,
        volume: Option<&str>,
        entrypoint: Option<Vec<&str>>,
    ) -> DeploymentConfig {
        return DeploymentConfig {
            name: name.to_owned(),
            container_strategy: ContainerStrategy::Manual,
            runtime_type,
            version: "12-12-2022".to_owned(),
            cmd,
            image,
            env: env.map(|env| env.into_iter().map(|(k, v)| (k.to_owned(), v.to_owned())).collect()),
            resources: resources.to_owned(),
            restart_policy,
            volume: match runtime_type {
                RuntimeType::Stateful => volume.map(|v| v.to_owned()),
                _ => None,
            },
            entrypoint: entrypoint.map(|e| e.into_iter().map(|e| e.to_owned()).collect()),
        };
    }
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

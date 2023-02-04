use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::types::pipe::Region;

#[derive(Debug, Serialize, Deserialize)]
pub struct Resources {
    pub vcpu: f64,
    pub ram: String,
    // TODO: use array instead of vector?
    pub vgpu: Option<Vec<Vgpu>>,
}

impl Resources {
    pub fn new(
        vcpu: f64,
        ram: &str,
        vgpu: Option<Vec<Vgpu>>,
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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
pub struct GatewayConfig {
    #[serde(rename = "type")]
    pub gateway_type: GatewayType,
    pub protocol: GatewayProtocol,
    pub target_port: i64,
    pub name: String,
    pub internal_domain: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Gateway {
    pub id: String,
    #[serde(rename = "type")]
    pub gateway_type: GatewayType,
    pub name: Option<String>,
    pub protocol: Option<GatewayProtocol>,
    pub deployment_id: String,
    pub created_at: String,
    pub hopsh_domain: Option<String>,
    pub hopsh_domain_enabled: Option<bool>,
    pub internal_domain: Option<String>,
    pub target_port: Option<i64>,
    pub primary: Option<bool>,
    // TODO: this is not in GET/gateways response
    // pub domains: Vec<Domain>,
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

// TODO: this
#[derive(Debug, Serialize, Deserialize)]
pub struct Build {}

#[derive(Debug, Serialize, Deserialize)]
pub struct BuildSettings {
    pub root_directory: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeploymentMetaData {
    pub root_directory: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeploymentOld {
    pub id: String,
    pub name: String,
    pub container_count: i64,
    pub created_at: String,
    pub metadata: Option<String>,
    pub build_cache_enabled: bool,
    pub build_settings: BuildSettings,
}

// TODO: this
// #[derive(Debug, Serialize, Deserialize)]
pub struct DeploymentRollout {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Deployment {
    pub id: String,
    pub name: String,
    pub build_id: Option<String>,
    pub active_build: Option<String>,
    pub active_rollout: Option<String>,
    // TODO: this
    // pub latest_rollout: Option<DeploymentRollout>,
    pub created_at: String,
    pub target_container_count: i64,
    pub container_count: i64,
    pub running_container_count: Option<i64>,
    pub build_settings: Option<BuildSettings>,
    pub build_cache_enabled: bool,
    // TODO: this
    // pub metadata: Option<String>,
    pub config: DeploymentConfig,
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
pub enum VolumeFormat {
    #[serde(rename = "ext4")]
    EXT4,
    #[serde(rename = "xfs")]
    XFS,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeDefinition {
    pub fs: VolumeFormat,
    // TODO: better type?
    pub size: String,
    // TODO: check if hop accepts my hop-js sdk fix
    pub mountpath: String,
}

// TODO: check this
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDeploymentConfig {
    pub name: Option<String>,
    pub container_strategy: Option<ContainerStrategy>,
    #[serde(rename = "type")]
    pub runtime_type: Option<RuntimeType>,
    pub version: Option<String>,
    pub cmd: Option<Vec<String>>,
    pub image: Option<Image>,
    pub env: Option<HashMap<String, String>>,
    pub resources: Option<Resources>,
    pub restart_policy: Option<RestartPolicy>,
    // pub volume: Option<HashMap<String, String>>,
    pub volume: Option<VolumeDefinition>,
    // pub volume: Option<Vec<String>>,
    pub entrypoint: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDeploymentConfig {
    pub name: String,
    pub container_strategy: ContainerStrategy,
    #[serde(rename = "type")]
    pub runtime_type: RuntimeType,
    pub version: String,
    pub cmd: Option<Vec<String>>,
    pub image: Image,
    pub env: Option<HashMap<String, String>>,
    pub resources: Resources,
    pub restart_policy: RestartPolicy,
    // pub volume: Option<HashMap<String, String>>,
    pub volume: Option<VolumeDefinition>,
    // pub volume: Option<Vec<String>>,
    pub entrypoint: Option<Vec<String>>,
}

impl CreateDeploymentConfig {
    pub fn new(
        name: &str,
        // container_strategy: ContainerStrategy,
        runtime_type: RuntimeType,
        version: &str,
        cmd: Option<Vec<String>>,
        image: Image,
        env: Option<HashMap<&str, &str>>,
        resources: Resources,
        restart_policy: RestartPolicy,
        // volume: Option<&str>,
        // TODO: check this still works
        volume: Option<VolumeDefinition>,
        entrypoint: Option<Vec<&str>>,
    ) -> CreateDeploymentConfig {

        // TODO: perform some light validation here?
        //  such as version, volume, ram size

        return CreateDeploymentConfig {
            name: name.to_owned(),
            container_strategy: ContainerStrategy::Manual,
            runtime_type: runtime_type.clone(),
            version: version.to_owned(),
            cmd,
            image,
            env: env.map(|e| e.into_iter().map(|(k, v)| (k.to_owned(), v.to_owned())).collect()),
            resources,
            restart_policy,
            /*volume: match runtime_type {
                RuntimeType::Stateful => volume.map(|v| v.to_owned()),
                _ => None,
            },*/
            // TODO: check this still works
            volume,
            // volume: volume.map(|e| e.into_iter().map(|(k, v)| (k.to_owned(), v.to_owned())).collect()),
            entrypoint: entrypoint.map(|e| e.into_iter().map(|e| e.to_owned()).collect()),
        };
    }
}

// TODO: remove option from hashmap and vec fields in struct but keep in constructor

// TODO: this is the returned config from the GET deployment endpoint, not the creation config
#[derive(Debug, Serialize, Deserialize)]
pub struct DeploymentConfig {
    // name: String,
    container_strategy: ContainerStrategy,
    #[serde(rename = "type")]
    runtime_type: RuntimeType,
    version: String,
    cmd: Option<Vec<String>>,
    image: Image,
    env: Option<HashMap<String, String>>,
    resources: Resources,
    restart_policy: RestartPolicy,
    volume: Option<HashMap<String, String>>,
    // volume: Option<Vec<String>>,
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
        // volume: Option<&str>,
        volume: Option<HashMap<&str, &str>>,
        entrypoint: Option<Vec<&str>>,
    ) -> DeploymentConfig {
        return DeploymentConfig {
            // name: name.to_owned(),
            container_strategy: ContainerStrategy::Manual,
            runtime_type: runtime_type.clone(),
            version: "12-12-2022".to_owned(),
            cmd,
            image,
            env: env.map(|e| e.into_iter().map(|(k, v)| (k.to_owned(), v.to_owned())).collect()),
            resources,
            restart_policy,
            /*volume: match runtime_type {
                RuntimeType::Stateful => volume.map(|v| v.to_owned()),
                _ => None,
            },*/
            volume: volume.map(|e| e.into_iter().map(|(k, v)| (k.to_owned(), v.to_owned())).collect()),
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

// TODO: finish this
#[derive(Debug, Serialize, Deserialize)]
pub enum HealthCheckType {
    #[serde(rename = "liveness")]
    Liveness,
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
    pub created_at: String,
    pub deployment_id: String,
    pub success_threshold: i64,
    #[serde(rename = "type")]
    pub healthcheck_type: HealthCheckType,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum HealthCheckProtocol {
    #[serde(rename = "http")]
    HTTP,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateHealthCheckConfig {
    pub protocol: HealthCheckProtocol,
    pub path: String,
    pub port: i64,
    pub interval: i64,
    pub timeout: i64,
    pub initial_delay: i64,
    pub max_retries: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateHealthCheckConfig {
    pub protocol: Option<HealthCheckProtocol>,
    pub path: Option<String>,
    pub port: Option<i64>,
    pub interval: Option<i64>,
    pub timeout: Option<i64>,
    pub initial_delay: Option<i64>,
    pub max_retries: Option<i64>,
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

// TODO: this
#[derive(Debug, Serialize, Deserialize)]
pub struct BuildCacheStats {}

#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeStats {
    pub provisioned_size: i64,
    pub used_size: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageStats {
    pub build_cache: Option<BuildCacheStats>,
    pub volume: VolumeStats,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rollout {
    pub count: i64,
    pub created_at: String,
    pub deployment_id: String,
    pub id: String,
    pub state: ContainerState,
}

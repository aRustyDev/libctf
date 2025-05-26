use kube_derive::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// IGNORE: Used to Auto-Generate ChallengeConfigs
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "ctf.rs",
    version = "v1",
    kind = "ChallengeConfig",
    namespaced,
    doc = "Custom Kubernetes resource for 'ChallengeConfig' resource"
)]
pub struct DeriveConfig {
    info: String,
    #[schemars(length(min = 3))]
    name: String,
    replicas: i32,
}

/// IGNORE: Used to Auto-Generate ChallengeSecrets
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "ctf.rs",
    version = "v1",
    kind = "ChallengeSecret",
    namespaced,
    doc = "Custom Kubernetes resource for 'ChallengeSecret' resource"
)]
pub struct DeriveSecret {
    info: String,
    #[schemars(length(min = 3))]
    name: String,
    replicas: i32,
}

/// IGNORE: Used to Auto-Generate Challenges
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "ctf.rs",
    version = "v1",
    kind = "Challenge",
    namespaced,
    doc = "Custom Kubernetes resource for 'Challenge' resource"
)]
pub struct DeriveResource {
    info: String,
    #[schemars(length(min = 3))]
    name: String,
    replicas: i32,
}

/// IGNORE: Used to Auto-Generate ChallengeConfigRequests
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "ctf.rs",
    version = "v1",
    kind = "ChallengeConfigRequest",
    namespaced,
    doc = "Custom Kubernetes resource for 'ChallengeConfigRequest' resource"
)]
#[cfg_attr(feature = "internal_doc", doc = "...")]
pub struct DeriveConfigRequest {
    info: String,
    #[schemars(length(min = 3))]
    name: String,
    replicas: i32,
}

/// IGNORE: Used to Auto-Generate ChallengeSecretRequests
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "ctf.rs",
    version = "v1",
    kind = "ChallengeSecretRequest",
    namespaced,
    doc = "Custom Kubernetes resource for 'ChallengeSecretRequest' resource"
)]
pub struct DeriveSecretRequest {
    info: String,
    #[schemars(length(min = 3))]
    name: String,
    replicas: i32,
}

/// IGNORE: Used to Auto-Generate ChallengeRequests
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "ctf.rs",
    version = "v1",
    kind = "ChallengeRequest",
    namespaced,
    doc = "Custom Kubernetes resource for 'ChallengeRequest' resource"
)]
pub struct DeriveResourceRequest {
    info: String,
    #[schemars(length(min = 3))]
    name: String,
    replicas: i32,
}

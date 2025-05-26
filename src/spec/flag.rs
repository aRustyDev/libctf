use kube_derive::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// IGNORE: Used to Auto-Generate FlagConfigs
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "ctf.rs",
    version = "v1",
    kind = "FlagConfig",
    namespaced,
    doc = "Custom Kubernetes resource for 'FlagConfig' resource"
)]
pub struct DeriveConfig {
    info: String,
    #[schemars(length(min = 3))]
    name: String,
    replicas: i32,
}

/// IGNORE: Used to Auto-Generate FlagSecrets
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "ctf.rs",
    version = "v1",
    kind = "FlagSecret",
    namespaced,
    doc = "Custom Kubernetes resource for 'FlagSecret' resource"
)]
pub struct DeriveSecret {
    info: String,
    #[schemars(length(min = 3))]
    name: String,
    replicas: i32,
}

/// IGNORE: Used to Auto-Generate Flags
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "ctf.rs",
    version = "v1",
    kind = "Flag",
    namespaced,
    doc = "Custom Kubernetes resource for 'Flag' resource"
)]
pub struct DeriveResource {
    info: String,
    #[schemars(length(min = 3))]
    name: String,
    replicas: i32,
}

/// IGNORE: Used to Auto-Generate FlagConfigRequests
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "ctf.rs",
    version = "v1",
    kind = "FlagConfigRequest",
    namespaced,
    doc = "Custom Kubernetes resource for 'FlagConfigRequest' resource"
)]
#[cfg_attr(feature = "internal_doc", doc = "...")]
pub struct DeriveConfigRequest {
    info: String,
    #[schemars(length(min = 3))]
    name: String,
    replicas: i32,
}

/// IGNORE: Used to Auto-Generate FlagSecretRequests
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "ctf.rs",
    version = "v1",
    kind = "FlagSecretRequest",
    namespaced,
    doc = "Custom Kubernetes resource for 'FlagSecretRequest' resource"
)]
pub struct DeriveSecretRequest {
    info: String,
    #[schemars(length(min = 3))]
    name: String,
    replicas: i32,
}

/// IGNORE: Used to Auto-Generate FlagRequests
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "ctf.rs",
    version = "v1",
    kind = "FlagRequest",
    namespaced,
    doc = "Custom Kubernetes resource for 'FlagRequest' resource"
)]
pub struct DeriveResourceRequest {
    info: String,
    #[schemars(length(min = 3))]
    name: String,
    replicas: i32,
}

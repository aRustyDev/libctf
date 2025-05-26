use kube_derive::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// IGNORE: Used to Auto-Generate PluginConfigs
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "ctf.rs",
    version = "v1",
    kind = "PluginConfig",
    namespaced,
    doc = "Custom Kubernetes resource for 'PluginConfig' resource"
)]
pub struct DeriveConfig {
    info: String,
    #[schemars(length(min = 3))]
    name: String,
    replicas: i32,
}

/// IGNORE: Used to Auto-Generate PluginSecrets
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "ctf.rs",
    version = "v1",
    kind = "PluginSecret",
    namespaced,
    doc = "Custom Kubernetes resource for 'PluginSecret' resource"
)]
pub struct DeriveSecret {
    info: String,
    #[schemars(length(min = 3))]
    name: String,
    replicas: i32,
}

/// IGNORE: Used to Auto-Generate Plugins
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "ctf.rs",
    version = "v1",
    kind = "Plugin",
    namespaced,
    doc = "Custom Kubernetes resource for 'Plugin' resource"
)]
pub struct DeriveResource {
    info: String,
    #[schemars(length(min = 3))]
    name: String,
    replicas: i32,
}

/// IGNORE: Used to Auto-Generate PluginConfigRequests
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "ctf.rs",
    version = "v1",
    kind = "PluginConfigRequest",
    namespaced,
    doc = "Custom Kubernetes resource for 'PluginConfigRequest' resource"
)]
#[cfg_attr(feature = "internal_doc", doc = "...")]
pub struct DeriveConfigRequest {
    info: String,
    #[schemars(length(min = 3))]
    name: String,
    replicas: i32,
}

/// IGNORE: Used to Auto-Generate PluginSecretRequests
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "ctf.rs",
    version = "v1",
    kind = "PluginSecretRequest",
    namespaced,
    doc = "Custom Kubernetes resource for 'PluginSecretRequest' resource"
)]
pub struct DeriveSecretRequest {
    info: String,
    #[schemars(length(min = 3))]
    name: String,
    replicas: i32,
}

/// IGNORE: Used to Auto-Generate PluginRequests
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "ctf.rs",
    version = "v1",
    kind = "PluginRequest",
    namespaced,
    doc = "Custom Kubernetes resource for 'PluginRequest' resource"
)]
pub struct DeriveResourceRequest {
    info: String,
    #[schemars(length(min = 3))]
    name: String,
    replicas: i32,
}

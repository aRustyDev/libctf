use kube_derive::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// IGNORE: Used to Auto-Generate VerifierConfigs
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "ctf.rs",
    version = "v1",
    kind = "VerifierConfig",
    namespaced,
    doc = "Custom Kubernetes resource for 'VerifierConfig' resource"
)]
pub struct DeriveConfig {
    info: String,
    #[schemars(length(min = 3))]
    name: String,
    replicas: i32,
}

/// IGNORE: Used to Auto-Generate VerifierSecrets
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "ctf.rs",
    version = "v1",
    kind = "VerifierSecret",
    namespaced,
    doc = "Custom Kubernetes resource for 'VerifierSecret' resource"
)]
pub struct DeriveSecret {
    info: String,
    #[schemars(length(min = 3))]
    name: String,
    replicas: i32,
}

/// IGNORE: Used to Auto-Generate Verifiers
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "ctf.rs",
    version = "v1",
    kind = "Verifier",
    namespaced,
    doc = "Custom Kubernetes resource for 'Verifier' resource"
)]
pub struct DeriveResource {
    info: String,
    #[schemars(length(min = 3))]
    name: String,
    replicas: i32,
}

/// IGNORE: Used to Auto-Generate VerifierConfigRequests
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "ctf.rs",
    version = "v1",
    kind = "VerifierConfigRequest",
    namespaced,
    doc = "Custom Kubernetes resource for 'VerifierConfigRequest' resource"
)]
#[cfg_attr(feature = "internal_doc", doc = "...")]
pub struct DeriveConfigRequest {
    info: String,
    #[schemars(length(min = 3))]
    name: String,
    replicas: i32,
}

/// IGNORE: Used to Auto-Generate VerifierSecretRequests
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "ctf.rs",
    version = "v1",
    kind = "VerifierSecretRequest",
    namespaced,
    doc = "Custom Kubernetes resource for 'VerifierSecretRequest' resource"
)]
pub struct DeriveSecretRequest {
    info: String,
    #[schemars(length(min = 3))]
    name: String,
    replicas: i32,
}

/// IGNORE: Used to Auto-Generate VerifierRequests
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "ctf.rs",
    version = "v1",
    kind = "VerifierRequest",
    namespaced,
    doc = "Custom Kubernetes resource for 'VerifierRequest' resource"
)]
pub struct DeriveResourceRequest {
    info: String,
    #[schemars(length(min = 3))]
    name: String,
    replicas: i32,
}

use crate::spec::util::{ResourceRequestSpec, ResourceSpec};
use kube_derive::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

impl ResourceRequestSpec for ChallengeRequestSpec {}
impl ResourceSpec for ChallengeSpec {}

/// IGNORE: Used to Auto-Generate Challenges
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[doc(hidden)]
#[kube(
    group = "ctf.rs",
    version = "v1",
    kind = "Challenge",
    namespaced,
    doc = "Custom Kubernetes resource for 'Challenge' resource"
)]
pub struct ChallengeSpec {
    pub info: String,
    #[schemars(length(min = 3))]
    pub name: String,
    pub replicas: i32,
}

/// IGNORE: Used to Auto-Generate ChallengeRequests
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[doc(hidden)]
#[kube(
    group = "ctf.rs",
    version = "v1",
    kind = "ChallengeRequest",
    namespaced,
    doc = "Custom Kubernetes resource for 'ChallengeRequest' resource"
)]
pub struct ChallengeRequestSpec {
    info: String,
    #[schemars(length(min = 3))]
    name: String,
    replicas: i32,
}

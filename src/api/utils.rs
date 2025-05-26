use crate::spec::challenge::{Challenge, ChallengeSpec};
// use crate::spec::util::{ResourceRequestSpec, ResourceSpec};
use context::Context;
use hyper_util::rt::TokioExecutor;
use kube::api::{DeleteParams, GetParams, Patch, PatchParams, PostParams};
use kube::{Client, Config, client::ConfigExt};
use serde::Serialize;
use tower::{BoxError, ServiceBuilder};

pub const MAX_RETRIES: u32 = 3;

/// Defines an interface for creating, reading, updating, and deleting (CRUD) resources.
pub trait Crud {
    // Challenge, Plugin, Flag, Verifier
    /// Expects same output as `kube::api::Request::create()`
    fn create(
        &self,
        ctx: &Context,
        client: Client,
        pp: &PostParams,
        data: ChallengeSpec,
    ) -> impl Future<Output = Result<Challenge, kube::Error>> + Send;
    /// Expects same output as `kube::api::Request::get()`
    fn read(
        &self,
        ctx: &Context,
        client: Client,
        name: String,
        gp: &GetParams,
    ) -> impl Future<Output = Result<Challenge, kube::Error>> + Send;
    /// Expects same output as `kube::api::Request::patch()`
    fn update<P: Serialize + std::fmt::Debug + Send + Sync + Clone + 'static>(
        &self,
        ctx: &Context,
        client: Client,
        name: String,
        pp: &PatchParams,
        patch: &Patch<P>,
    ) -> impl Future<Output = Result<Challenge, kube::Error>> + Send;
    /// Expects same output as `kube::api::Request::delete()`
    fn delete(
        &self,
        ctx: &Context,
        client: Client,
        name: String,
        dp: &DeleteParams,
    ) -> impl Future<Output = Result<Challenge, kube::Error>> + Send;
}

/// Wrapper around `kube::api::Request`, creates a 'client' for working on Kubernetes resources.
pub async fn client() -> Result<kube::Client, BoxError> {
    let config = Config::infer().await?;
    let service = ServiceBuilder::new()
        .layer(config.base_uri_layer())
        .option_layer(config.auth_layer()?)
        .map_err(BoxError::from)
        .service(hyper_util::client::legacy::Client::builder(TokioExecutor::new()).build_http());
    Ok(kube::Client::new(service, config.default_namespace))
}

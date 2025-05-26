use http::request::Request;
use kube::api::{DeleteParams, GetParams, Patch, PatchParams, PostParams};
use serde::Serialize;
use tower::BoxError;

pub trait Crud {
    fn create(
        &self,
        pp: &PostParams,
        data: Vec<u8>,
    ) -> Result<Request<Vec<u8>>, kube::core::request::Error>;
    fn read(
        &self,
        name: String,
        gp: &GetParams,
    ) -> Result<Request<Vec<u8>>, kube::core::request::Error>;
    fn update<P: Serialize>(
        &self,
        name: String,
        pp: &PatchParams,
        patch: &Patch<P>,
    ) -> Result<Request<Vec<u8>>, kube::core::request::Error>;
    fn delete(
        &self,
        name: String,
        dp: &DeleteParams,
    ) -> Result<Request<Vec<u8>>, kube::core::request::Error>;
}

pub fn client() -> Result<kube::api::Request, BoxError> {
    // Implementation of client function
    let url = "";
    Ok(kube::api::Request::new(url))
}

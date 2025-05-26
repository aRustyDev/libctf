use crate::api::utils::{Crud, client};
use http::request::Request;
use kube::api::{DeleteParams, GetParams, Patch, PatchParams, PostParams, WatchParams};
use kube::core::request::Error;
use serde::Serialize;

pub struct challenge {
    pub id: String,
    pub name: String,
    pub description: String,
    pub points: u32,
    pub category: String,
    pub flag: String,
    pub client: kube::api::Request,
}

impl Crud for challenge {
    fn create(&self, pp: &PostParams, data: Vec<u8>) -> Result<Request<Vec<u8>>, Error> {
        self.client.create(pp, data)
    }
    fn read(&self, name: String, gp: &GetParams) -> Result<Request<Vec<u8>>, Error> {
        self.client.get(&name, gp)
    }
    fn update<P: Serialize>(
        &self,
        name: String,
        pp: &PatchParams,
        patch: &Patch<P>,
    ) -> Result<Request<Vec<u8>>, Error> {
        self.client.patch(&name, pp, patch)
    }
    fn delete(&self, name: String, dp: &DeleteParams) -> Result<Request<Vec<u8>>, Error> {
        self.client.delete(&name, dp)
    }
}
impl challenge {
    pub fn new(
        id: String,
        name: String,
        description: String,
        points: u32,
        category: String,
        flag: String,
    ) -> Self {
        challenge {
            id,
            name,
            description,
            points,
            category,
            flag,
            client: client().unwrap(),
        }
    }
    fn watch(&self, wp: &WatchParams, ver: &str) -> Result<Request<Vec<u8>>, Error> {
        self.client.watch(wp, ver)
    }
    fn replace(
        &self,
        name: String,
        pp: &PostParams,
        data: Vec<u8>,
    ) -> Result<Request<Vec<u8>>, Error> {
        self.client.replace(&name, pp, data)
    }
}

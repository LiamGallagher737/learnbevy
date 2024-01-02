use std::{
    collections::HashMap,
    net::IpAddr,
    sync::{Arc, RwLock},
    time::Instant,
};

#[derive(Debug, Default, Clone)]
pub struct RateLimitMap {
    map: Arc<RwLock<HashMap<IpAddr, RateLimit>>>,
}

impl RateLimitMap {
    pub fn get(&self, addr: &IpAddr) -> Option<RateLimit> {
        self.map
            .read()
            .unwrap()
            .get(addr)
            .map(|rate_limit| rate_limit.to_owned())
    }

    pub fn insert(&self, addr: IpAddr, length: f32) {
        self.map.write().unwrap().insert(
            addr,
            RateLimit {
                length,
                start: Instant::now(),
            },
        );
    }

    pub fn remove(&self, addr: &IpAddr) {
        self.map.write().unwrap().remove(addr);
    }
}

#[derive(Debug, Clone)]
pub struct RateLimit {
    pub length: f32,
    pub start: Instant,
}

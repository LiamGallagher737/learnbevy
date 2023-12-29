use std::{
    collections::HashMap,
    net::Ipv4Addr,
    sync::{Arc, RwLock},
    time::Instant,
};

#[derive(Debug, Default, Clone)]
pub struct RateLimitMap {
    map: Arc<RwLock<HashMap<Ipv4Addr, RateLimit>>>,
}

impl RateLimitMap {
    pub fn get(&self, addr: &Ipv4Addr) -> Option<RateLimit> {
        self.map
            .read()
            .unwrap()
            .get(addr)
            .map(|rate_limit| rate_limit.to_owned())
    }

    pub fn insert(&self, addr: Ipv4Addr, length: f32) {
        self.map.write().unwrap().insert(
            addr,
            RateLimit {
                length,
                start: Instant::now(),
            },
        );
    }

    pub fn remove(&self, addr: &Ipv4Addr) {
        self.map.write().unwrap().remove(addr);
    }
}

#[derive(Debug, Clone)]
pub struct RateLimit {
    pub length: f32,
    pub start: Instant,
}

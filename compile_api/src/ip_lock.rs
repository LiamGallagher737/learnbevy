use crate::{Error, PeerAddr};
use async_std::sync::Mutex;
use std::{collections::HashSet, net::IpAddr, sync::Arc};
use tide::{Body, Middleware, Next, Request, Response, Result, StatusCode};

#[derive(Default)]
pub struct IpLockMiddleware {
    active_ips: Arc<Mutex<HashSet<IpAddr>>>,
}

impl IpLockMiddleware {
    pub fn new() -> Self {
        Self::default()
    }
}

#[tide::utils::async_trait]
impl<State: Clone + Send + Sync + 'static> Middleware<State> for IpLockMiddleware {
    async fn handle(&self, req: Request<State>, next: Next<'_, State>) -> Result {
        let PeerAddr(peer_ip) = req.ext().cloned().unwrap();
        let mut active_ips = self.active_ips.lock().await;
        if active_ips.contains(&peer_ip) {
            return Ok(Response::builder(StatusCode::TooManyRequests)
                .body(Body::from_json(&Error::ActiveRequestExists)?)
                .build());
        }
        active_ips.insert(peer_ip);
        drop(active_ips);

        let res = next.run(req).await;
        self.active_ips.lock().await.remove(&peer_ip);
        Ok(res)
    }
}

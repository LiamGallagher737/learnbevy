use crate::{Error, PeerAddr};
use async_std::sync::Mutex;
use std::{
    collections::HashMap,
    net::IpAddr,
    sync::Arc,
    time::{Duration, Instant},
};
use tide::{Body, Middleware, Next, Request, Response, Result, StatusCode};

const SUCCESS_RATE_LIMIT: Duration = Duration::from_secs(5);
const UNSUCCESS_RATE_LIMIT: Duration = Duration::from_secs(1);

#[derive(Default)]
pub struct RateLimitMiddleware {
    limits: Arc<Mutex<HashMap<IpAddr, RateLimit>>>,
}

impl RateLimitMiddleware {
    pub fn new() -> Self {
        Self::default()
    }
}

#[tide::utils::async_trait]
impl<State: Clone + Send + Sync + 'static> Middleware<State> for RateLimitMiddleware {
    async fn handle(&self, req: Request<State>, next: Next<'_, State>) -> Result {
        let PeerAddr(peer_ip) = req.ext().cloned().unwrap();
        let mut limits = self.limits.lock().await;
        if let Some(rate_limit) = limits.get(&peer_ip) {
            if rate_limit.start.elapsed() < rate_limit.duration {
                let time_left = (rate_limit.duration - rate_limit.start.elapsed())
                    .as_secs_f32()
                    .ceil();
                return Ok(Response::builder(StatusCode::TooManyRequests)
                    .body(Body::from_json(&Error::RateLimit { time_left })?)
                    .header("retry-after", time_left.to_string())
                    .build());
            }
            limits.remove(&peer_ip);
        }
        drop(limits);

        let res = next.run(req).await;
        let rate_limit_duration = match res.status() {
            StatusCode::Ok => SUCCESS_RATE_LIMIT,
            _ => UNSUCCESS_RATE_LIMIT,
        };
        self.limits.lock().await.insert(
            peer_ip,
            RateLimit {
                start: Instant::now(),
                duration: rate_limit_duration,
            },
        );
        Ok(res)
    }
}

struct RateLimit {
    start: Instant,
    duration: Duration,
}

use std::sync::Arc;
use cdrs::authenticators::NoneAuthenticator;
use cdrs::cluster::session::{new as new_session, Session};
use cdrs::cluster::{ClusterTcpConfig, NodeTcpConfigBuilder, TcpConnectionPool};
use cdrs::load_balancing::RoundRobinSync;
use failure::_core::time::Duration;
use r2d2_redis::{r2d2 as r_r2d2, RedisConnectionManager};

pub type Connection = Session<RoundRobinSync<TcpConnectionPool<NoneAuthenticator>>>;
pub type Pool = Arc<Connection>;
pub type RedisPool = r_r2d2::Pool<RedisConnectionManager>;

pub fn connect_db(node_address: &str) -> Pool {
    let authenticator = NoneAuthenticator {};
    let node = NodeTcpConfigBuilder::new(&node_address, authenticator)
        .max_size(5)
        .min_idle(Some(2))
        .max_lifetime(Some(Duration::from_secs(60)))
        .idle_timeout(Some(Duration::from_secs(60)))
        .build();

    let cluster_config = ClusterTcpConfig(vec![node]);
    let lb = RoundRobinSync::new();
    let no_compression: Arc<Connection> =
        Arc::new(new_session(&cluster_config, lb).expect("session should be created"));
    no_compression
}
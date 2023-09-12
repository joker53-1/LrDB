
use std::env;
use std::time::Duration;

#[derive(Debug)]
pub struct Config {
    store_addr: String,
    raft: bool,
    scheduler_addr: String,
    log_level: String,
    db_path: String,
    raft_base_tick_interval: Duration,
    raft_heartbeat_ticks: i32,
    raft_election_timeout_ticks: i32,
    raft_log_gc_tick_interval: Duration,
    raft_log_gc_count_limit: u64,
    split_region_check_tick_interval: Duration,
    scheduler_heartbeat_tick_interval: Duration,
    scheduler_store_heartbeat_tick_interval: Duration,
    region_max_size: u64,
    region_split_size: u64,
}

impl Config {
    pub fn new_default_config() -> Self {
        let store_addr = "127.0.0.1:20250".to_string();
        let scheduler_addr = "127.0.0.1:7897".to_string();
        let log_level = Self::get_log_level();

        Config {
            store_addr,
            raft: true,
            scheduler_addr,
            log_level,
            db_path: "/tmp/rocksE".to_string(),
            raft_base_tick_interval: Duration::from_secs(1),
            raft_heartbeat_ticks: 2,
            raft_election_timeout_ticks: 10,
            raft_log_gc_tick_interval: Duration::from_secs(10),
            raft_log_gc_count_limit: 128000,
            split_region_check_tick_interval: Duration::from_secs(10),
            scheduler_heartbeat_tick_interval: Duration::from_secs(10),
            scheduler_store_heartbeat_tick_interval: Duration::from_secs(10),
            region_max_size: 144 * 1024 * 1024, // 144 MB
            region_split_size: 96 * 1024 * 1024, // 96 MB
        }
    }

    pub fn new_test_config() -> Self {
        let log_level = Self::get_log_level();

        Config {
            store_addr: "".to_string(),
            raft: true,
            scheduler_addr: "".to_string(),
            log_level,
            db_path: "/tmp/rocksE".to_string(),
            raft_base_tick_interval: Duration::from_millis(50),
            raft_heartbeat_ticks: 2,
            raft_election_timeout_ticks: 10,
            raft_log_gc_tick_interval: Duration::from_millis(50),
            raft_log_gc_count_limit: 128000,
            split_region_check_tick_interval: Duration::from_millis(100),
            scheduler_heartbeat_tick_interval: Duration::from_millis(100),
            scheduler_store_heartbeat_tick_interval: Duration::from_millis(500),
            region_max_size: 144 * 1024 * 1024, // 144 MB
            region_split_size: 96 * 1024 * 1024, // 96 MB
        }
    }

    fn get_log_level() -> String {
        env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string())
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.raft_heartbeat_ticks == 0 {
            return Err("heartbeat tick must be greater than 0".to_string());
        }

        if self.raft_election_timeout_ticks != 10 {
            eprintln!("Election timeout ticks needs to be same across all the cluster, otherwise it may lead to inconsistency.");
        }

        if self.raft_election_timeout_ticks <= self.raft_heartbeat_ticks {
            return Err("election tick must be greater than heartbeat tick".to_string());
        }

        Ok(())
    }
}

#[test]
fn test() {
    let default_config = Config::new_default_config();
    let test_config = Config::new_test_config();

    println!("Default Config: {:?}", default_config);
    println!("Test Config: {:?}", test_config);
}

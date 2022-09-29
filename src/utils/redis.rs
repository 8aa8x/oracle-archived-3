use anyhow::Result;
use rslock::RedLock;

/// Creates a single Redis client with rslock
pub fn create_lock_client(uris: Vec<String>) -> Result<RedLock> {
    Ok(RedLock::new(uris))
}

use anyhow::{Result};

use async_trait::async_trait;

use crate::device::DevRes;

#[async_trait]
pub trait SonoffDimmable {
    /// brightness (min=0, max=100)
    async fn dim(&self, br: u8) -> Result<DevRes>;
}

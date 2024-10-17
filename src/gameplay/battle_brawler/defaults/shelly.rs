use async_trait::async_trait;

use super::super::{BrawlerExt, BrawlerInfo};

/// A structure representing Shelly.
#[derive(Clone, Debug)]
pub struct Shelly {
    pub data: BrawlerInfo,
}

#[async_trait]
impl BrawlerExt for Shelly {
    fn info(&self) -> &BrawlerInfo {
        &self.data
    }
}

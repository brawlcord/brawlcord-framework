use async_trait::async_trait;

use super::super::{BrawlerExt, BrawlerInfo};

/// A structure representing Nita.
#[derive(Clone, Debug)]
pub struct Nita {
    pub data: BrawlerInfo,
}

#[async_trait]
impl BrawlerExt for Nita {
    fn info(&self) -> &BrawlerInfo {
        &self.data
    }
}

use crate::filter::Filter;
use crate::log::Log;
use async_trait::async_trait;

pub(crate) struct BufferFilter {
    buffers: Vec<String>,
    all: bool,
}

impl BufferFilter {
    #[allow(dead_code)]
    pub(crate) fn new(buffers: Vec<String>) -> Self {
        let mut s = Self {
            buffers,
            all: false,
        };

        s.all = s.buffers.contains(&"all".to_string());

        s
    }
}

#[async_trait]
impl Filter for BufferFilter {
    async fn filter(&self, log: &Log) -> bool {
        if self.all {
            return false;
        }

        for b in &self.buffers {
            if b == &log.buffer {
                return false;
            }
        }

        true
    }
}

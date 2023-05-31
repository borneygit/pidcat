use crate::filter::Filter;
use crate::log::Log;
use async_trait::async_trait;
use std::sync::Arc;

pub(crate) struct BufferFilter {
    filter: Option<Arc<dyn Filter>>,
    buffers: Vec<String>,
    all: bool,
}

impl BufferFilter {
    #[allow(dead_code)]
    pub(crate) fn new(buffers: Vec<String>, filter: Option<Arc<dyn Filter>>) -> Self {
        let mut s = Self {
            buffers,
            filter,
            all: false,
        };

        s.all = s.buffers.contains(&"all".to_string());

        s
    }
}

#[async_trait]
impl Filter for BufferFilter {
    async fn filter(&self, mut log: Log) -> Option<Log> {
        if let Some(f) = &self.filter {
            let f = Arc::clone(f);
            if let Some(r) = f.filter(log).await {
                log = r;
            } else {
                return None;
            }
        }

        if self.all {
            return Some(log);
        }

        for b in &self.buffers {
            if b == &log.buffer {
                return Some(log);
            }
        }

        None
    }
}

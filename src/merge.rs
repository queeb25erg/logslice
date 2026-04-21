use crate::log_entry::LogEntry;
use crate::merge_config::MergeConfig;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

/// Merges multiple sorted streams of log entries into a single sorted stream.
pub struct Merger {
    config: MergeConfig,
}

impl Merger {
    pub fn new(config: MergeConfig) -> Self {
        Self { config }
    }

    /// Merge multiple sorted vectors of log entries into one sorted vector.
    /// Sorting is based on the timestamp field specified in config.
    pub fn merge(&self, streams: Vec<Vec<LogEntry>>) -> Vec<LogEntry> {
        if streams.is_empty() {
            return vec![];
        }

        // Use a min-heap: (timestamp_str, stream_index, entry_index)
        let mut heap: BinaryHeap<Reverse<(String, usize, usize)>> = BinaryHeap::new();
        let mut indices: Vec<usize> = vec![0; streams.len()];

        for (i, stream) in streams.iter().enumerate() {
            if let Some(entry) = stream.first() {
                let ts = self.get_sort_key(entry);
                heap.push(Reverse((ts, i, 0)));
            }
        }

        let mut result = Vec::new();

        while let Some(Reverse((_, stream_idx, entry_idx))) = heap.pop() {
            let entry = streams[stream_idx][entry_idx].clone();
            result.push(entry);

            let next_idx = entry_idx + 1;
            if next_idx < streams[stream_idx].len() {
                let next_entry = &streams[stream_idx][next_idx];
                let ts = self.get_sort_key(next_entry);
                heap.push(Reverse((ts, stream_idx, next_idx)));
            }
            indices[stream_idx] = next_idx;
        }

        result
    }

    fn get_sort_key(&self, entry: &LogEntry) -> String {
        entry
            .fields
            .get(&self.config.timestamp_field)
            .cloned()
            .unwrap_or_default()
    }
}

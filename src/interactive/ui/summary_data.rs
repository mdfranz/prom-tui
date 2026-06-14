use chrono::{DateTime, Local, TimeZone};

use crate::prom::{Metric, Quantile, Sample};

pub struct SummaryData {
    pub quantile_values: Vec<Quantile>,
    pub time: DateTime<Local>,
    pub count: u64,
    pub sum: f64,
}

impl SummaryData {
    pub fn parse(metric: &Metric, selected_label: &str) -> Option<Self> {
        let last_sample = metric
            .time_series
            .get(selected_label)?
            .samples
            .last()?;

        if let Sample::SummarySample(summary) = last_sample {
            let time = Local.timestamp_opt(summary.timestamp as i64, 0).unwrap();
            Some(Self {
                quantile_values: summary.quantile_values.clone(),
                time,
                count: summary.count,
                sum: summary.sum,
            })
        } else {
            None
        }
    }
}

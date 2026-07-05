use bigdecimal::BigDecimal;
use serde::Serialize;

use crate::constants::TS_RS_EXPORT_TO;

/// A bucket that holds a range of points and the number of elements
/// in that range to make up a histogram.
#[derive(ts_rs::TS, Serialize, Debug)]
#[ts(export, export_to = TS_RS_EXPORT_TO)]
pub struct PointsHistogramBucket {
    pub lower_inclusive_points_bound: BigDecimal,
    pub upper_exclusive_points_bound: BigDecimal,
    pub count: usize,
}

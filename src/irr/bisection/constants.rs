//! Constant values

/// The level of precision we require for NPVs to zero.
///
/// NPVs will be calculated for some currency or other similarly represented medium of exchange, suggesting two decimals of precision are sufficient for our purposes.
pub const NPV_PRECISION: f32 = 0.001;

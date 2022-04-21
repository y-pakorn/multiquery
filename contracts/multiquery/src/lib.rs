#[cfg(not(feature = "library"))]
pub mod contract;

pub mod msg;

#[cfg(test)]
mod testing;

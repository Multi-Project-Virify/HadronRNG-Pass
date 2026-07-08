use std::time::{SystemTime, UNIX_EPOCH};

use crate::error::HadronError;


pub struct EntropySource;


impl EntropySource {

    pub fn collect(
        size: usize
    ) -> Result<Vec<u8>, HadronError> {

        let mut entropy = Vec::new();


        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| HadronError::GenerationFailed)?;


        entropy.extend_from_slice(
            &time.as_nanos().to_le_bytes()
        );


        entropy.resize(
            size,
            0
        );


        Ok(entropy)
    }
}
use crate::{
    rng::HadronRng,
    error::HadronError,
};



impl HadronRng {


    pub fn key(
        &mut self,
        bits: usize
    )
        -> Result<Vec<u8>, HadronError>
    {

        if bits % 8 != 0 {

            return Err(
                HadronError::InvalidLength
            );
        }


        let bytes =
            bits / 8;


        self.random_bytes(bytes)

    }
}
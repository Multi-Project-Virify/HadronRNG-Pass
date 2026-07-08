use rand_core::{RngCore, OsRng};

use crate::error::HadronError;


pub struct HadronRng {

    rng: OsRng,

}



impl HadronRng {


    pub fn new()
        -> Result<Self, HadronError>
    {

        Ok(Self {

            rng: OsRng,

        })
    }



    pub fn random_bytes(
        &mut self,
        size: usize
    )
        -> Result<Vec<u8>, HadronError>
    {

        let mut buffer =
            vec![0u8; size];


        self.rng
            .fill_bytes(&mut buffer);


        Ok(buffer)
    }



    pub(crate) fn fill(
        &mut self,
        data: &mut [u8]
    )
    {

        self.rng.fill_bytes(data);

    }
}
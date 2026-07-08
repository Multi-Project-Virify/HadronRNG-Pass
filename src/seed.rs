use crate::{
    hash,
    entropy::EntropySource,
    error::HadronError,
};


pub struct Seed {

    value: Vec<u8>,

}



impl Seed {


    pub fn generate(
        size: usize
    )
        -> Result<Self, HadronError>
    {

        let entropy =
            EntropySource::collect(size)?;


        Ok(Self {

            value:
                hash::hash(&entropy)

        })
    }



    pub fn bytes(
        &self
    )
        -> &[u8]
    {

        &self.value

    }
}
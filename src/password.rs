use crate::{
    rng::HadronRng,
    error::HadronError,
};


const CHARSET:
&[u8] =
b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
abcdefghijklmnopqrstuvwxyz\
0123456789!@#$%^&*";



impl HadronRng {


    pub fn password(
        &mut self,
        length: usize
    )
        -> Result<String, HadronError>
    {

        if length == 0 {

            return Err(
                HadronError::InvalidLength
            );
        }


        let mut result =
            String::with_capacity(length);


        let bytes =
            self.random_bytes(length)?;


        for b in bytes {

            let index =
                (b as usize)
                % CHARSET.len();


            result.push(
                CHARSET[index]
                    as char
            );
        }


        Ok(result)
    }
}
#[derive(Clone)]
pub struct HadronConfig {

    pub password_length: usize,

    pub key_size: usize,

}



impl Default for HadronConfig {

    fn default()
        -> Self
    {

        Self {

            password_length: 32,

            key_size: 256,

        }
    }
}
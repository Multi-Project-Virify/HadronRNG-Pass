#[derive(Debug)]
pub enum HadronError {

    InitializationFailed,

    InvalidLength,

    GenerationFailed,

}


impl std::fmt::Display for HadronError {

    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>
    ) -> std::fmt::Result {

        write!(
            f,
            "{:?}",
            self
        )
    }
}


impl std::error::Error for HadronError {}
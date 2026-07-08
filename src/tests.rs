#[cfg(test)]
mod tests {


    use crate::HadronRng;



    #[test]
    fn create_rng() {

        let rng =
            HadronRng::new();


        assert!(
            rng.is_ok()
        );
    }



    #[test]
    fn generate_bytes() {

        let mut rng =
            HadronRng::new()
            .unwrap();


        let data =
            rng.random_bytes(64)
            .unwrap();


        assert_eq!(
            data.len(),
            64
        );
    }



    #[test]
    fn generate_password() {

        let mut rng =
            HadronRng::new()
            .unwrap();


        let pass =
            rng.password(32)
            .unwrap();


        assert_eq!(
            pass.len(),
            32
        );
    }

}
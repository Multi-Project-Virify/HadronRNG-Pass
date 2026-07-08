#[cfg(test)]
mod tests {

    use crate::HadronRng;


    #[test]
    fn create_rng() {

        let rng =
            HadronRng::new();


        assert!(
            rng.is_ok(),
            "HadronRng creation failed"
        );
    }



    #[test]
    fn generate_bytes() {

        let mut rng =
            HadronRng::new()
                .expect("failed to create rng");


        let data =
            rng.random_bytes(64)
                .expect("failed to generate bytes");


        assert_eq!(
            data.len(),
            64
        );
    }



    #[test]
    fn generate_password() {

        let mut rng =
            HadronRng::new()
                .expect("failed to create rng");


        let password =
            rng.password(32)
                .expect("failed to generate password");


        assert_eq!(
            password.len(),
            32
        );
    }



    #[test]
    fn generate_key() {

        let mut rng =
            HadronRng::new()
                .expect("failed to create rng");


        let key =
            rng.key(256)
                .expect("failed to generate key");


        assert_eq!(
            key.len(),
            32
        );
    }



    #[test]
    fn generate_many_bytes() {

        let mut rng =
            HadronRng::new()
                .expect("failed to create rng");


        for _ in 0..1000 {

            let data =
                rng.random_bytes(128)
                    .expect("generation failed");


            assert_eq!(
                data.len(),
                128
            );
        }
    }



    #[test]
    fn passwords_are_different() {

        let mut rng =
            HadronRng::new()
                .expect("failed to create rng");


        let first =
            rng.password(32)
                .expect("password failed");


        let second =
            rng.password(32)
                .expect("password failed");


        assert_ne!(
            first,
            second,
            "duplicate passwords generated"
        );
    }



    #[test]
    fn keys_are_different() {

        let mut rng =
            HadronRng::new()
                .expect("failed to create rng");


        let first =
            rng.key(256)
                .expect("key failed");


        let second =
            rng.key(256)
                .expect("key failed");


        assert_ne!(
            first,
            second,
            "duplicate keys generated"
        );
    }



    #[test]
    fn different_rng_instances() {

        let mut rng1 =
            HadronRng::new()
                .expect("failed");


        let mut rng2 =
            HadronRng::new()
                .expect("failed");


        let a =
            rng1.random_bytes(64)
                .expect("failed");


        let b =
            rng2.random_bytes(64)
                .expect("failed");


        assert_ne!(
            a,
            b,
            "two RNG instances returned same data"
        );
    }

}
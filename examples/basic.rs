use hadronrng::HadronRng;


fn main() {

    let mut rng =
        HadronRng::new()
            .expect("failed to create HadronRng");


    let bytes =
        rng.random_bytes(32)
            .expect("failed to generate bytes");


    println!("Generated {} bytes", bytes.len());

}
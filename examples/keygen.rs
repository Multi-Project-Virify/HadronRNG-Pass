use hadronrng::HadronRng;


fn main() {

    let mut rng =
        HadronRng::new()
            .expect("failed to create HadronRng");


    let key =
        rng.key(256)
            .expect("failed to generate key");


    println!(
        "Generated key size: {} bytes",
        key.len()
    );

}
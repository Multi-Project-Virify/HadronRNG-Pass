use hadronrng::HadronRng;


fn main() {

    let mut rng =
        HadronRng::new()
            .expect("failed to create HadronRng");


    let password =
        rng.password(32)
            .expect("failed to generate password");


    println!(
        "Generated password: {}",
        password
    );

}
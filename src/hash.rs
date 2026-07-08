use sha2::{
    Sha256,
    Digest
};



pub fn hash(
    input: &[u8]
) -> Vec<u8> {

    let mut hasher =
        Sha256::new();


    hasher.update(input);


    hasher.finalize()
        .to_vec()
}
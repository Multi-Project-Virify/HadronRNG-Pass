pub fn xor_mix(
    data: &mut [u8]
) {

    let len = data.len();


    for i in 0..len {

        let next =
            data[(i + 1) % len];


        data[i] ^= next;
    }
}



pub fn rotate_bytes(
    data: &mut [u8]
) {

    if data.is_empty() {
        return;
    }


    data.rotate_left(1);
}
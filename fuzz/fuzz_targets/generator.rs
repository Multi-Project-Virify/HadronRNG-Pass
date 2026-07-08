#![no_main]

use libfuzzer_sys::fuzz_target;
use hadronrng::HadronRng;


fuzz_target!(|data: &[u8]| {

    let mut rng =
        match HadronRng::new() {
            Ok(value) => value,
            Err(_) => return,
        };


    let size =
        if data.is_empty() {
            1
        } else {
            data[0] as usize
        };


    let _ =
        rng.random_bytes(size);

});
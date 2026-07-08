#![no_main]

use libfuzzer_sys::fuzz_target;
use hadronrng::HadronRng;


fuzz_target!(|length: u8| {

    let mut rng =
        match HadronRng::new() {
            Ok(value) => value,
            Err(_) => return,
        };


    let size =
        length as usize;


    let _ =
        rng.password(size);

});
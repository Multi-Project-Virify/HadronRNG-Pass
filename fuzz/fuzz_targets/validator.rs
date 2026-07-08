#![no_main]

use libfuzzer_sys::fuzz_target;
use hadronrng::validator;


fuzz_target!(|data: String| {

    let _ =
        validator::validate(&data);

});
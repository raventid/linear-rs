#![no_main]
use arbitrary::{Arbitrary, Unstructured};
use libfuzzer_sys::fuzz_target;
use linear_rs::{MustUseChannel, Undroppable};

#[derive(Arbitrary, Debug)]
struct FuzzInput {
    chan: i32,
    message: i32,
}

fuzz_target!(|data: FuzzInput| {
    let a = MustUseChannel::<Undroppable>::new(data.chan);
    let _b = a.send(data.message);
});

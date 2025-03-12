pub mod nope {
    // a collision with the name of the wasm-bindgen function in foo/lib.rs
    // appears to cause the bug to happen
    // note that this needs to have the #[no_mangle] attribute to reproduce the issue
    #[no_mangle]
    pub fn init() -> u32 {
        0
    }

    // if there isn't a second function here (the name doesn't matter)
    // that also has #[no_mangle], then wasm-bindgen will succeed
    #[no_mangle]
    pub fn why() -> u32 {
        0
    }
}

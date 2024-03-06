uniffi::setup_scaffolding!();

#[uniffi::export]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

/// refernce are not supported by uniffi v0.25.*
/// Even if we can use reference, &mut [u8] is not supported as well
/// we of course can unsafely mutate the buffer, but the buffer here is actually just a copy, not the actual buffer that we want to mutate

#[uniffi::export]
pub fn accept_mut_buf(buf: &str) {
    unsafe {
        let mut_buf = std::slice::from_raw_parts_mut(buf.as_ptr() as *mut u8, buf.len());
        for i in mut_buf.iter_mut() {
            *i = 42;
        }
    }
}


#[uniffi::export]
pub fn return_vec() -> Vec<u8> {
    let mut vec = Vec::new();
    for _ in 0..65536 {
        vec.push(42);
    }
    return vec;
}

#[uniffi::export]
pub fn push_some_data(mut buf: Vec<u8>) -> Option<u32> {
    println!("push_some_data");
    for _ in 0..65536 {
        buf.push(42);
    }
    println!("push_some_data: {}", buf.len());
    None
}
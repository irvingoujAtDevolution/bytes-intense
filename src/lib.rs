uniffi::setup_scaffolding!();

#[uniffi::export]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

// refernce are not supported by uniffi v0.25.*
// Even if we can use reference, &mut [u8] is not supported as well
// we of course can unsafely mutate the buffer, but the buffer here is actually just a copy, not the actual buffer that we want to mutate

// #[uniffi::export]
// pub fn accept_mut_buf(buf: &str) {
//     unsafe {
//         let mut_buf = std::slice::from_raw_parts_mut(buf.as_ptr() as *mut u8, buf.len());
//         for i in mut_buf.iter_mut() {
//             *i = 42;
//         }
//     }
// }


#[uniffi::export]
pub fn return_vec() -> Vec<u8> {
    let mut vec = Vec::new();
    for _ in 0..65536 {
        vec.push(42);
    }
    return vec;
}

/// This will NOT MUTATE the buffer, because the buffer is just a copy from an immutable buffer from C#
#[uniffi::export]
pub fn take_vec_and_try_mutate(mut buf: Vec<u8>) -> Option<u32> {
    println!("push_some_data");
    for _ in 0..65536 {
        buf.push(42);
    }
    println!("push_some_data: {}", buf.len());
    None
}

#[uniffi::export]
pub fn take_string(s: String) {
    println!("take_string: {}", s);
}

// Does not work in uniffi v0.25.*
// #[uniffi::export]
// pub fn take_str(s: &str) {
//     println!("take_str: {}", s);
// }


#[uniffi::export]
pub fn return_string() -> String {
    "Hello from Rust".to_string()
}

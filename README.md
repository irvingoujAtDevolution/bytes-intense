## Experiment Binding

This is an experiment for justfying the use of uniffi in IronRDP.

### Get started
install uniffi-bindgen-cs using
```
cargo install uniffi-bindgen-cs --git https://github.com/NordSecurity/uniffi-bindgen-cs --tag v0.8.0+v0.25.0
```

then just simply do 
```
cd dotnet-experiment;just csharp;dotnet run
```

### Result

uniffi does not support passing reference across FFI, all reference are simply a copy.

 1. Taking a byte slice from C# (good)

    This is possible, the type `byte[]` can be passed into Rust, for current version of uniffi-bindgen-cs, we can take it as `Vec<u8>`, for the latest version of uniffi-bindgen (which uniffi-bindgen-cs not yet supported), we can take `&[u8]` on the rust side.

2.  Returning a byte array from Rust (good)

3. Copying bytes from Rust into a buffer provided by C#

    This is impossible, as there cannot have pointers passed in and out the FFI. One could have some struct where it contains some sort of inner mutability, but at the cost of performance.

4. Taking a string by reference. (good)

5. Returning an owned string. (good)

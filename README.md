# tiledb-rust-bind
Rust Bindings for the [TileDB C API](https://tiledb-inc-tiledb.readthedocs-hosted.com/en/stable/c-api.html),
generated using bindgen.

# Usage
`Cargo.toml`
```
[dependencies]
tiledb-sys = "0.1.1"
```

`main.rs`:
```
use tiledb_sys::{tiledb_config_t, tiledb_error_t, tiledb_config_alloc};
fn main() {
    // we are calling the C API, so memory accesses may be unsafe.
    unsafe {
        /* Create a config */
        let mut config_ptr = std::ptr::null_mut();
        let config_ptr_ptr = &mut config_ptr as *mut *mut tiledb_config_t;
        let mut error_ptr = std::ptr::null_mut();
        let error_ptr_ptr = &mut error_ptr as *mut *mut tiledb_error_t;
        assert_eq!(tiledb_config_alloc(config_ptr_ptr, error_ptr_ptr), 0);
        let _config_ptr = *config_ptr_ptr;
    }
}
```

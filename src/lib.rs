#[no_mangle]
pub fn flip(a: i64, b: i64) -> (i64, i64) {
    // make the compiler bring in the libserde_json library
    // anything that touches serde_json will work 
    serde_json::Value::Bool(true);
    (b, a)
}
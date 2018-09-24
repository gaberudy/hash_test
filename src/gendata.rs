use std::ffi::CString;

extern "C" {
    pub fn srand48(__seedval: ::std::os::raw::c_long);
    pub fn drand48() -> f64;
}

pub trait BenchString<T> {
    fn as_bench_value(&self) -> T;
}

impl BenchString<String> for u32 {
    fn as_bench_value(&self) -> String {
        return format!("{:x}", self);
    }
}

impl BenchString<CString> for u32 {
    fn as_bench_value(&self) -> CString {
        return CString::new(format!("{:x}", self)).unwrap();
    }
}

impl BenchString<u32> for u32 {
    fn as_bench_value(&self) -> u32 {
        return *self;
    }
}

pub fn generate<T>(size: usize) -> Vec<T>
where
    u32: BenchString<T>,
{
    print!("[benchmark] generating data... ");
    let mut v = Vec::with_capacity(size);
    unsafe {
        srand48(11);
        for _i in 0..size {
            let d = drand48();
            let value: u32 = ((size as f64 * d / 4.) as u32).wrapping_mul(271828183u32);
            v.push(value.as_bench_value());
        }
    }
    println!("done!");
    return v;
}

#[test]
fn test_generate_int() {
    let v: Vec<u32> = generate(100000);
    assert_eq!(v.len(), 10);
    for e in v {
        println!("{}", e);
    }
}

#[test]
fn test_generate_string() {
    let v: Vec<String> = generate(100000);
    assert_eq!(v.len(), 10);
    for e in v {
        println!("{}", e);
    }
}

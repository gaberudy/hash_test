mod gendata;
pub mod mem_stats;
mod test_klib;
mod test_rust;

extern crate docopt;
use docopt::Docopt;
use std::ffi::CString;

const USAGE: &'static str = "
Usage: hast_test [options]
       hash_test --help

Options:
  -h --help     Show this screen.
  -k            Benchmark klib/kash (otherwise rust HashMap)
  -s            Benchmark strings (otherwise integers)
  -j            Use jemalloc mem printout (otherwise native resident mem diff approximation)
  -n <size>     Number of items [default: 5000000]
";

fn main() {
    let args = Docopt::new(USAGE)
        .and_then(|opt| opt.parse())
        .unwrap_or_else(|e| e.exit());
    let use_strings = args.get_bool("-s");
    let use_klib = args.get_bool("-k");
    let je_stats = args.get_bool("-j");
    let n: usize = args
        .get_str("-n")
        .parse()
        .expect("-n argument should be a number");

    let hash_name = if use_klib { "klib" } else { "rust HashMap" };
    let type_name = if use_strings { "strings" } else { "integers" };
    println!(
        "[benchmark] {} hash being tested with array of {} {}",
        hash_name, n, type_name
    );

    if use_klib {
        if use_strings {
            let v: Vec<CString> = gendata::generate(n);
            test_klib::insert_or_remove_str(&v, je_stats);
        } else {
            let v: Vec<u32> = gendata::generate(n);
            test_klib::insert_or_remove_int(&v, je_stats);
        }
    } else {
        if use_strings {
            let v: Vec<String> = gendata::generate(n);
            test_rust::insert_or_remove(&v, je_stats);
        } else {
            let v: Vec<u32> = gendata::generate(n);
            test_rust::insert_or_remove(&v, je_stats);
        }
    }
}

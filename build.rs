// build.rs

extern crate cc;

fn main() {
    cc::Build::new()
        .file("native/mem_stats.c")
        .file("native/runlib.c")
        .file("native/test.c")
        .compile("mem_stats");
}

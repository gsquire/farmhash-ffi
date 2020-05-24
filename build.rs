fn main() {
    cc::Build::new()
        .cpp(true)
        .file("include/farmhash.cc")
        .flag("-O3")
        .compile("libfarmhash.a");
}

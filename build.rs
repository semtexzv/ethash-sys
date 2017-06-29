extern crate gcc;

fn main(){

    gcc::Config::new()
        .include("native")
        .file("native/sha3.c")
        .file("native/internal.c")
        .static_flag(true)
    .compile("libethash.a");
}
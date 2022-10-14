//Following the example at https://stackoverflow.com/questions/26946646/package-with-both-a-library-and-a-binary
//the binary and the library are peers and no changes to .toml file
fn main() {
    println!("Starting use simple library");

    println!("Calling the simple library function: {}", simple_lib_usage::use_simple_library());

    println!("Done use simple library");  

}

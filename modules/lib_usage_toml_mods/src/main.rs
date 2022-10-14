//Following the example at https://stackoverflow.com/questions/26946646/package-with-both-a-library-and-a-binary
//the binary and the library are peers but I use the toml file attributes to give a specific namespace name to the 
//binary and the library that gets used in the code

use randy_lib::use_simple_library;//this brings this path into scope so that I don't have to specify
//the full path every time I want to call the function. I can just use 

fn main() {
    println!("Starting use simple library");

    println!("Calling the simple library with toml mods function: {}", use_simple_library());

    println!("Done using simple library");  

}

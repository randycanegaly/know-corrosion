//THE BOOK - Chapter 4
fn main() {
    println!("Hello,owner!");
    {   // s is not valid here; it's not yet declared
        let s = "hello";    // s is valid from this point forward

        // do stuff with s
        println!(" and {} to you too!", s);
    }   // this scope is now over, and s is no longer valid

    //println!(" and now I use {} to bust you!", s);//this doesn't compile. "not found in this scope"

    let mut s = String::from("boop");
    s.push_str(", yer nose!");//appends to s
    println!("{}",s);//prints new extended s

    //Try the above but use a String type - allocated on the heap, size is unknown at compile time
    {   // s is not valid here; it's not yet declared
        let m = String::from("fnorp grelm");    // s is valid from this point forward

        // do stuff with s
        println!(" flip your nertz and {} to you too!", m);
    }   // this scope is now over, and s is no longer valid

    //println!(" and now I use {} to bust you!", m);//this doesn't compile. "cannot find value m in this scope"
    
    let s1 = String::from("Hello Droober!");
    let s2 = s1;// Here, Rust assumes that s1 is no longer valid so as to avoid a double-free error when s1 and s2 go out of scope
    //This is called a move

    //println!("Can I use s1? s1 = {}", s1);// So this throws an E0382 error as s1 is no longer valid
    println!("Can I use s2? s2 = {}", s2);

    //But this works. integers are simple types of known size. They get put on the stack. There is no heap memory
    //to free. The double-free error is not a concern, so no move is done.
    let x = 5;
    let y = x;
    println!("x={} and y={}", x, y);

    //So, now try the above using clone. This will create a "deep" copy of s2
    let s3 = s2.clone();
    println!("Quit cloning around. {}", s3);

    println!("Ownership in a function.");
    let s = String::from("hello function");//s comes into scope

    takes_ownership(s);// s's value moves into the function and so is no longer valid here
    //can I use s here?
    //println!("{}",s);//this errors, because s is no longer valid. It moved into the takes_ownership() function.
    //String type doesn't implement the copy trait 


    let x = 5;//x comes into scope

    makes_copy(x);// x would move into the function, but i32 is Copy, 
    //it implements the copy trait, so it's okay to still use x afterward
    //can I use x here?
    println!("{}",x);//Yes. No move into the makes_copy() function. integer implements the copy trait


}


fn takes_ownership(some_string: String) {// some_string comes into scope. This was s before s's value moved into the function
println!("{}", some_string);//OK. some_string is in scope here
}// Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) {// some_integer comes into scope
    println!("{}", some_integer);
}// Here, some_integer goes out of scope. Nothing special happens.

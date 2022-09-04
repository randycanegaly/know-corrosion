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




}

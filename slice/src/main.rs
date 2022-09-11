fn main() {
    println!("Hello, slice!");

    let mut s = String::from("Slice me");
    s.push_str(" me some melon");

    println!("{}", s);

    let front = &s[..];
    println!("{}", front);

    let front1 = &s[2..7];
    println!("{}", front1);

    let front2 = &s[2..];
    println!("{}", front2);

    let front3 = &s[..7];
    println!("{}", front3);

    let first = first_word(&s);
    println!("The first word is: {}", first);

    //String literals
    let my_string_literal = "Hey, Randy lit!";//string literals are slices, of type &str

    let my_string = String::from("Hey, Randy String!");

    //let firstSlice = first_word(my_string);//this doesn't work. type mismatch, String vs. expected &str
    let second_slice = first_word(my_string_literal);
    let third_slice = first_word(&my_string_literal[..11]);
    let fourth_slice = first_word(&my_string[0..9]);

    println!("secondSlice: {}, thirdSlice: {}, fourthSlice: {}", second_slice, third_slice, fourth_slice);



    


}

//fn first_word(s: &String) -> &str{//a slice is of type $str
    fn first_word(s: &str) -> &str{//changing the parameter from &String to &str makes
        //the function more general. If we want to pass a String, take a slice of the whole
        //string and pass it
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {//walk the bytes array, pattern matching until a space is seen
        if  item == b' ' {
            return &s[0..i];//return the slice that goes from index 0 to i-1
        }
    }

    &s[..]//didn't see a space, so return the whole string.
    //This returns a slice, type &str

}

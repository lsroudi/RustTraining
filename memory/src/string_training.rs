pub fn string_example() {
    let str_1 : &str = "Hello";
    let str_2 : String = String::from(" World");

    let str_from_str_1 : String = str_1.to_string();

    println!(" str_1 is : {} ", str_1);
    println!(" str_from_str_1 is : {} ", str_from_str_1);

    let str_from_hard_coded : String = "Hello World hard coded".to_string();
    println!(" str_from_hard_coded is : {} ", str_from_hard_coded);

    let string_from_str : String = String::from(str_2);
    println!(" string_from_str is : {} ", string_from_str);


    // not a copy, just reference
    let str_3 : String = String::from("Hello World!");
    let str_from_string : &str = &str_3;
    println!(" str_3 is : {} ", str_3);
    println!(" str_from_string is : {} ", str_from_string);

    //concat
    let string_concat = ["first name"," last name"].concat();
    println!(" string_concat is : {} ", string_concat);

    let string_concat_2 = format!("{} {}","first"," second");
    println!(" string_concat_2 is : {} ", string_concat_2);



    let str_4 : &str = " World!!!";
    let str_5 : String = String::from("Hello");
    let string_plus_str = str_5 + str_4;
    println!(" string_plus_str is : {} ", string_plus_str);

    let str_6 : &str = "Hello ";
    let mut mutable_string = String::new();
    mutable_string.push_str(&str_6);
    mutable_string.push_str("World!!!");
    println!(" mutable_string is : {} ", mutable_string);


    let string_from_sub : &str = &mutable_string[5..];
    println!(" string_from_sub is : {} ", string_from_sub);

    let string_from_sub_exlude : &str = &mutable_string[5..10];
    println!(" string_from_sub_exlude is : {} ", string_from_sub_exlude);

    let string_from_sub_include : &str = &mutable_string[5..=10];
    println!(" string_from_sub_include is : {} ", string_from_sub_include);

    let char_from_string = &string_from_sub_include.chars().nth(1);

    match char_from_string {
        Some(c) =>println!(" char_from_string is : {} ", c),
        None => {}   
    }

    if let Some(c) = &string_from_sub_include.chars().nth(4) {
        println!(" c is : {} ", c);
    }
    

}
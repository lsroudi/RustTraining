mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            let mut v = vec![1, 2, 3, 4, 5];

            let _first = &v[0];
            println!("The first element is: {}", v.len());
            v.push(6);


        }
    }
}
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}





#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

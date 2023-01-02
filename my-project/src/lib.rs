 mod front_of_house{

    pub mod hosting {
        pub fn add_to_watchlist() {}
    
        fn seat_at_table()  {}
    }

    mod serving {
        fn taken_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }

 }

 pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_watchlist();

    //relative path
    front_of_house::hosting::add_to_watchlist();
 }


 fn serve_order() {}


 mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();

        // allows to specify the function that aew present in the same file by using super keyword
        super::serve_order();

    }

    fn cook_order() {} 
 }
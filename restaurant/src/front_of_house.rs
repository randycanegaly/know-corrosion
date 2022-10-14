//I don't need "pub mod front_of_house{}" here. The file serves as the module definition
pub mod hosting;

pub mod serving {
    fn take_order() {}

    pub fn serve_order() {}

    fn take_payment() {}
}
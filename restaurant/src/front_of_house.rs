
pub mod hosting;
pub mod serving {
    pub fn take_order() {
        super::hosting::add_to_waitlist();
    }

    fn serve_order() {}

    fn take_payment() {}
}

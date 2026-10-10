// Re-exporting names with pub use
// When we bring a name into scope with the use keyword , the name available in the new scope is
// private .
// To enable the code that calls our code to refer that name as if it had been defined in that
// code's scope , we can combine pub and use . This technique is called Re-exporting because we're
// bringing an item into scope but also making that item available for others to bring into their
// scope .

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

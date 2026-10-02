// Let's define some modules and function signatures in this library crate !
//
// Library section
// Frong of the house module containing other modules that then contain functions
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    // Serving Section
    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

// We define a module by starting with the mod keyword and then specify the name of the module (in
// this case , front_of_house ) and place a curly brackets around the body of the module
// Inside modules , we can have other modules , as in this case with the modules 'hosting' and
// 'serving' .
// Modules can also hold definitions for other items , such as structs , enums , constants , traits.
// PS : Definition is the logic
//
// By using modules , we can group related definitions together and name why they're related .
// Programmers using this code will have an easier time finding the definitions they wanted to use
// beacuse they could navigate the code based on the groups rather having to read through all the
// definitions.
// Programmers adding new functionality to this code would know where to place the code to keep the
// program organized .
//
// Earlier , we mentioned that src/main.rs and src/lib.rs are called crate roots .
// The reason  for their name is that the contents of either of these two files form a module named
// crate at the root of the crate's module structure , known as the module tree .
//
//
// In our last exemple it shows that some of the modules nest inside one another ( hosting nests
// inside front_of_house) .
// The tree also shows that some modules are siblings to each other , meaning they're defined in the
// same module (hosting and serving are defined within front_of_house).
// To continue the family metaphor , if module A is contained inside module B , we say that module A
// is the child of module B and that module B is the parent of module A .
//
// Notice that the entire module tree is rooter under the implicit module named crate .
// The module tree might remind us of the filesystem's directory tree on Linux , this is a very apt
// comparison ! just like directories in file system , we can use modules to organize our code .

// Paths for Referring to an Item in the Module Tree
//
// To show Rust where to find an item in module tree , we use a path in the same way we use a path
// when navigating a filesystem .
// If we want to call  a function , we need to know its path : A path can take two forms :
// An absolute path starts fron a crate root by using a crate name or a literal crate .
// A relative path starts from the current module and uses self , super , or an identifier in the
// current module .
//
// Both absolute and relative paths are followwed by one or more identifiers separated by double
// colons " :: "
//
// let's Return to our last exemple , How do we call the add_to_waitlist function ? , we simplified
// our ode a bit by removing sone of the modules and functions .
// I will show two ways to call the add_to_waitlist function from a new function eat_at_restaurant
// difned in the crate root .

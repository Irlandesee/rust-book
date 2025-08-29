fn main() {
    //[OWNERSHIP]
    //Ownership is a set of rules that govern how a rust program manages memory, which is managed
    //through a system of ownership with a set of rules that the compiler checks. If any of the
    //rules are violated, the program won't compile.

    //[THE STACK AND THE HEAP]
    //In system programming languages like rust, whether a value is on the stack or the heap
    //affects how the language behaves
    //Both the stack and the heap are parts of memory available to the code to use at runtime, but
    //they are structured in different ways
    //The stack stores values in the order it gets them, and removes the values in the opposite
    //order
    //The heap is less organized: when data is put on the heap, a certain amount of spaces is
    //requested: the memory allocator finds an empty spot in the heap that is big enough, marks it
    //as being in use, and returns a pointer, which is an address of that location (Allocating on
    //the heap).
    //Because the pointer to the heap is a known fixed size, it can be stored on the stack, but
    //when the data is actually needed, one must follow the pointer.
    //
    //Pushing to the stack is faster than allocating on the heap because the allocator never has to
    //search for a place to store new data.
    //Allocating space on the heap requires more work because the allocator must first find a big
    //enough space to hold the data and then perform bookkeeping to prepare for the next
    //allocation.
    //
    //Accessing data in the heap is generally slower than accessing data on the stack (one needs a
    //pointer)
    //
    //[OWNERSHIP RULES]
    //Each value in rust has a owner
    //There can only be one owner at a time
    //When the owner goes out of scope, the value will be dropped.
    //
    //[VARIABLE SCOPE]

    //This variable is valid from the point at which it's declared, until the end of the current
    //scope.
    let s = "hello";

    // :: allows to namespace this particular from function under the String type.
    let x = String::from("hello");

    //A mutable string
    let mut mut_string = String::from("Hello");
    mut_string.push_str(", world!");
    println!("{mut_string}");

    //Memory and allocation
    // In the case of a string literal, the contents are known at compile time so that the text
    // is hardcoded into the final string.
    // With the String type, in order to support a mutable, we need to allocate an amount of memory
    // on the heap, unknown at compile time:
    //  * The memory must be requested from the memory allocator at runtime.
    //  * The memory must be returned to the allocator when no longer needed.
    //
    //In rust the memory is automatically returned once the variable that owns it goes out of
    //scope. For example:
    {
        let s = String::from("Wow"); // s is valid from this point forward
    } //This scope is no longer valid, and is no longer valid
    //When a variable goes out of scope, rust calls a special function called drop. (RAII pattern)

    // Variables and DAta interacting with move
}

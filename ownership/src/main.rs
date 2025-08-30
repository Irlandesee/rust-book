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

    // Variables and Data interacting with move
    let x1 = 5;
    let y1 = x1; //Bind the value 5 to x, then make a copy of the value in x and bind it to y
    //
    // A String is made up of three parts, a pointer to the memory that holds the contents of the
    // string, a length, and a capacity. This group of data is stored on the stack.
    // The lenght is how much memory, in bytes, the contents of the String are currently using.
    // The capacity is the total amount of memory, in bytes, that the String has received from the
    // allocator.
    //
    let s1 = String::from("Hello");
    let s2 = s1; //When s1 is assigned to s2, the String data is copied, meaning the pointer, the length,
    //and the capacity that are on the stack. The data on the heapthat the pointer refers to is not copied.

    //When a varible goes out of scope, rust automatically calls the drop function and cleans up
    //the heap memory for that variable: This is a problem, because s2 and s1 go out of scope they
    //will both try to free the same memory. This is known as a doable free error: Freeing memory
    //twice can lead to memory corruption, which can potentially lead to security vulnerabilities.
    //
    //To ensure memory safety, after the line
    //let s2 = s1;
    //Rust consides s1 no longer valid. Therefore, Rust does not need to free anything when s1
    //goes out of scope.
    //
    //This concept is called move: s1 was moved to s2
    //Rust will never automatically create deep copies of the data, any automatic copying can be
    //assumed to be inexpensive in terms of runtime performance.
    //
    //Scope and Assignment
    //The inverse of this is true for the relationship between scoping, ownership and memory being
    //freed via the drop function as well: when an existing variable is assigned to a completely
    //new variable, Rust will call drop and free the original value's memory immediately
    let mut s2 = String::from("Hello");
    s2 = String::from("ahoy"); //At this point, nothing is referring to the original value on the
    //heap at all
    //Thus the original string goes immediately out of scope.
    println!("{s2}, world"); //Ahoy World is printed
    //
    //Variables and data interacting with clone
    //When a deep copy of the heap data is needed, a method called clone can be used.
    //
    let u = String::from("hello");
    let u2 = u.clone(); //The heap data is actually copied
    //
    // Stack only data: Copy
    // Types such as integers have a known size at compile time are stored entirely on the stack,
    // so copies of the actual values are quick to make. There is no difference between deep and
    // shallow copying here, so calling clone wouldn't do anything different from the usual shallow
    // copying.
    // If a type implements the Copy trait, variables that use it do not move, but rather are
    // trivially copied, making them still valid after assignment to another value
    //
    // Rust won't let us annotate a type with Copy if the type, or any of its parts, has
    // implemented the Drop trait. Nothing that requires allocation or is some form of resource can
    // implement Copy.
    //
    // Ownership and function
    // The mechanics of passing a value to a function are similar to those when assigning a value
    // to a variable. Passing a variable to a function will move or copy, just as assignment does.
    //
    let s3 = String::from("HelloWorld!");
    takes_ownership(s3); // s's value moves into the function, and is no longer valid here

    let x3 = 5; //x comes into scope
    makes_copy(x3); //i32 implements the Copy trait, x3 does NOT move into the function, its okay
    //to use x afterward
} // x3 goes out of scope, then s. S's value was moved, nothing special happens

fn takes_ownership(some_string: String) {
    //some_string comes into scope
    println!("{some_string}");
} //some_string goes out of scope and the drop is called. The backing memory is freed

fn makes_copy(some_integer: i32) {
    //some_integer comes into scope
    println!("{some_integer}");
} // Some integer goes out of scope, nothing special happnes

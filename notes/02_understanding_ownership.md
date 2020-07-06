# Understanding Ownership

- ownership is meant to make memory safe without having a garbage collector
- this chapter will cover ownership, borrowing, slices, data in memory layouts

## What is Ownership

- _ownership_ is central to the way Rust works and it's simple to explain
- all programs have to manage a computer's memory for running
- some use garbage collectors that constantly check for unused memory, some
need the programmer to manually allocate memory
- rust uses a system that checks rules at compile time and thus does not slow
down the program when it is running
- this chapter will cover strings as an example

### The Stack and the Heap

- in many programming scenarios the stack and heap are not that important, but
for systems programming and rust they are very important
- where data is stored influences the behavior of the language as well as its
speed
- stack: memory that stores data in order and returns them in the opposite
order, last int, first out
- data stored on the stack must have a known size at compile time, unknown or
changing sizes must be stored on the heap
- heap: less organized, a certain amount of space is requested to store data,
OS finds the space and returns a pointer (address of its location) to it
- pushing to the stack is faster than allocating on the heap because for the
stack no location large enough has to be found and then kept in order
- accessing data on the heap is slower and jumping between data is also slower
than working on one piece of data at a time
- when a function is called, the values passed to the function are all pushed
onto the stack -- to return the values they are popped off the stack
- ownership addresses what code is using data on the heap, cleaning up unused
data on the heap etc

### Ownership Rules

- each value in Rust has a variable that's called its _owner_
- there can only be one owner at a time
- when the owner goes out of scope, the value will be dropped

### Variable Scope

- range in a program for which an item is valid
- when a variable comes _into scope_ it is valid, when is goes _out of scope_
it becomes invalid
- scopes are generally encapsulated by or related to curly brackets
    ```
    {                       // s comes into scope
        let s = "hello";

                            // s is valid

    }                       // s goes out of scope
    ```

### The `String` Type

- simple data types are stored on the stack and popped off when they go out of
scope
- more complex data types are stored on the heap and must be cleaned up after
use
- `String` will be the example used here insofar as it relates to ownership
- string literals are not always convenient because they are immutable and hard
coded
- `String` is allocated on the heap and can change at runtime, they can be
created from string literals
    ```
    let s = String::from("hello");
    ```
- the resulting type can be modified:
    ```
    let mut s = String::from("hello");
    s.push_str(", world!");         // appends to s
    ```
- the difference between `String` and string literals is the way they deal with
memory

### Memory and Allocation

- string literals are hardcoded into the program because they are known at
compile time -- they are fast efficient
- it is not possible to reserve blobs of memory at compile time for each string
that might change
- `String` is growable, so: its memory must be requested from the OS at
runtime; the memory must be returned to the OS when the `String` is done
- the programmer does the allocation manually 
    ```
    String::from
    ```
- normally memory is either freed by a garbage collector or manually by the
programmer, in Rust it is freed when the variable goes out of scope
- when `s` goes out of scope the `drop` function associated with it is
automatically called by Rust to free the memory
- this seems simple now, but it can be more complicated in more complicated
code

#### Ways Variables and Data Interact: Move

- if two primitive data types are set equal, the data is copied and then there
are two variables with two copies of the same data, both are on the stack
    ```
    let x = 5;
    let y = x;
    ```
- for `String` this is different
    ```
    let s1 = String::from("hello");
    let s2 = s1;
    ```
- `s1` is made up of a `ptr`, `len`, and `capacity`, the pointer points to the
first element of the string in memory, `len` is the amount of bytes of memory
that the string is currently using and `capacity` is the total amount of
memory allocated by the OS
- when `s1` is assigned to `s2`, the three pieces of data are copied, but the
data on the stack remains the same, it is not copied and the two pointers point
to the same place in memory
- in the example above Rust moves the data from `s1` to `s2` and invalidates
`s1` so it is no longer valid
- invalidating `s1` will mean that when `s2` goes out of scope the memory is
only freed once and thus does not generate a double free error
- additionally, Rust will never automatically make deep and expensive copies of
anything -- it will be fast by default

#### Ways Variables and Data Interact: Clone

- if we do want a deep copy of the data on the heap we use `clone`
    ```
    let s1 = String::from("hello");
    let s2 = s1.clone();
    ```
- `clone` is something that is expensive to call

#### Stack-Only Data: Copy

- if a type has the `copy` trait, an older version of the variable is still
valid after copying, like with integers
    ```
    let x = 5;
    let y = x;
    ```
- a type can't have the `copy` trait if any of its parts implement `drop`
- all simple or primitive types are copy

### Ownership and Functions

- passing a variable to a function is similar to assigning values to variables,
thus the same rules apply
    ```
    fn main() {
        let s = String::from("hello");      // s comes into scope
        takes_ownership(s);                 // value of s moves into 
                                            // function
                                            // it's no longer valid

        let x = 5;                          // x comes into scope
        makes_copy(x);                      // x is Copy and is thus 
                                            // still valid
    }   // x and then s go out of scope
        // nothing special happes to s because it is already invalid
    
    fn takes_ownership(s: String) {         // s comes into scope
        println!("{}", s);
    }   // s goes out of scope and drop is called, memory is freed

    fn makes_copy(i: i32) {                 // i comes into scope
        println!("{}", i);
    }   // i goes out of scope, not affecting x
    ```
- if `s` were to be used after the `takes_ownership(s)` was called, a compile
time error would happen

### Return Values and Scope

- returning values can also transfer ownership
    ```
    fn main() {
        let s1 = give_ownership();          // fn moves its return
                                            // value to s1

        let s2 = String::from("hello");     // s2 comes into scope

        let s3 = takes_and_gives_back(s2);  // s2 moved into fn
                                            // return value moved to s3
    }   // s3 goes out of scope and is dropped, so does s1.
        // s2 is already out of scope, so nothing happens

    fn gives_ownership() -> String {        // will move return value
                                            // into calling fn
        let s = String::from("hello");      // s comes into scope
        s                                   // s is returned and moves
                                            // to the calling function
    }   // nothing goes out of scope

    fn takes_and_gives_back(s: String) -> String {
                                            // s comes into scope
        s                                   // s is returned and moves
                                            // to the calling fn
    }   // nothing goes out of scope
    ```
- assigning the value of a variable to another moves it
- when an active variable goes out of scope, it is dropped
- one option for returning ownership of the argument plus a result is to return
a tuple from a function -- a better way to do it is to use _references_

## References and Borrowing

- if one uses a function that takes ownership and then has to return ownership
so the argument can be used afterwards
- passing references to functions instead of taking ownership is the solution
to that
    ```
    fn main() {
        let s1 = String::from("hello");
        let len = calculate_length(&s1);
        println!("The length of '{}' is {}.", s1, len);
    }

    fn calculate_length(s: &String) -> usize {
        s.len()
    }
    ```
- ampersands '`&`' are _references_ and enable referring to values without
taking ownership 
- above, `s` points to `s1` which points to the actual value
- dereferencing is done with `*`
- `&s1` refers to the value of `s1` but does now own it -- the value will not
be dropped when `s` goes out of scope
- when functions have references as parameters it is called _borrowing_
- references are immutable by default

### Mutable References

- creating a mutable string and then passing a mutable reference to a function
allows variables to be modified using their references
    ```
    fn main() {
        let mut s = String::from("hello");
        change(&mut s);
    }

    fn change(s: &mut String) {
        s.push_str(", world");
    }
    ```
- big restriction: there can only be one mutable reference to a particular
piece of data in a particular scope
- this only allows restricted mutation -- less than most other languages
- Rust can thus prevent _data races_ at compile time -- these three things need
to be true: two or more pointers access data at the same time, at least one of
the pointers is used to write to the data, there are no mechanisms to
synchronize the access to the data
- data races are undefined and difficult to diagnose
- new scopes allow for more mutable references, just not simultaneous ones
- we also cannot borrow data as mutable if it is also borrowed as immutable
- if an immutable reference is being used it is not expected that the value
changes at the same time
- multiple immutable references are ok because nobody can change any of the
data
- some intricacies are: if immutable references are no longer used a mutable
one can be created even if the other ones are technically still in scope
- borrowing errors are annoying, but they prevent bugs at compile time

### Dangling References

- these are created when a reference to a non-existent memory exists, producing
undefined behavior
    ```
    fn main() {
        let ref_to_nothing = dangle();
    }

    fn dangle() -> &String {                // returns ref to str
        let s = String::from("hello");      // s is new string

        &s                                  // return ref to str
    }   // s goes out of scope here and is dropped
        // the reference now refers to nothing
    ```
- the simple solution here is to return the string with ownership instead

### The Rules of References

- at any given time, you can have _either_ one mutable reference _or_ any
number of immutable references
- references must always be valid

## The Slice Type

- slices do not have ownership
- slices reference a contiguous sequence of elements in a collection rather
than the whole collection

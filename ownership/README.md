# Ownership

## A Rust unique feature

`Ownership` is Rust most unique feature. It enable Rust to make memory safety guarantees without the need of a garbage collector.

`Ownership` is based on a game of rule that the compiler will verify at compilation time. If one of the rules has been violated, the program will not compile.

## (ASIDE) Stack and Heap

### What is the stack ? 

Both the stack and the heap are parts of memory available to your code to use at runtime, but they are structured in different ways. The stack stores values in the order it gets them and removes the values in the opposite order. This is referred to as last in, first out. Adding data is called `pushing` onto the stack, and removing data is called `popping` off the stack. **All data stored on the stack must have a known, fixed size.** Data with an unknown size at compile time or a size that might change must be stored on the heap instead.

### What is the heap ?

The heap is less organized: when you put data on the heap, you request a certain amount of space. The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location. This process is called `allocating` on the heap (pushing values onto the stack is not considered allocating). Because the pointer to the heap is a known, fixed size, you can store the pointer on the stack, but when you want the actual data, you must follow the pointer. 

### Why allocating on the stack is faster than on the heap ? 

Pushing to the stack is faster than allocating on the heap because the allocator never has to search for a place to store new data; that location is always at the top of the stack. Comparatively, allocating space on the heap requires more work because the allocator must first find a big enough space to hold the data and then perform bookkeeping to prepare for the next allocation.

### Local variables and arguments on a function are allocated on the stack 

When your code calls a function, the values passed into the function (including, potentially, pointers to data on the heap) and the function’s local variables get pushed onto the stack. When the function is over, those values get popped off the stack.

## Rules of ownership

- Each value in Rust has an `owner`.
- **There can only be one owner at a time.**
- When the owner goes out of scope, the value will be dropped.


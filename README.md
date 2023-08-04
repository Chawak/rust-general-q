# rust-general-q
**1. heap vs stack; Which types rely on which?**

- Stack rely on heap because stack keep the pointer point to memory in heap.

**2. ownership & move**
- Ownership
  - Rust concept to bind variable and data
  - Every data must have exactly one owner at a time
  - When owner is gone, data is drop

- Move
  - When the ownership is transfer to other variable
    
        s1 = String::from("Hello World");
        s2 = s1;
    S1 is no longer valid after s2 declaration.

**3. Copy vs Clone**
- Copy: Refer to shallow copy
- Clone: Deep copy. Duplicate data in heap

**4. & vs &mut; Why you can't return reference of the variable inside the function?**

- When we don’t want to pass ownership due to calling fn, Reference params can solve this 
  - `&` : immutable reference
  - `&mut` : mutable reference 

- When a mutable reference is in used, other references aren’t allow.

- We can’t return reference of the variable declare inside function because when the function is done, variable’s value is drop so the reference to the address of memory is no longer valid.

**5. &str vs String vs &String**
- `String` : std class to own string data
- `str` : immutable sequence of string
- `&String` : Reference of 
- `&str` : Reference of string slice

**6. Vec<A> vs &Vec<A> vs &[A] vs [A; 10]**

- `Vec<A>` : Vector of generic type A
- `&Vec<A>` : Immutable Reference to vector of generic type A
- `&[A]` : Immutable Reference of slice
- `[A; 10]` : Array of A with fixed-length 10

**7. Pattern matching with match and if let**

- `match`
  
        match x {
            "some" => some_fn(),
            SomeEnum::Value => println!("Match enum"),
            _ => println!("Default match for exhautive check"),
        }

    **Benefits over if**
  - Let us check if the type is match for enum rather than compare bare value
  - Exhuastive check all possible case

- `if let` : Another writing style for match

        if let MyEnum::ValA(num) == myValue {
            some_fn();
        }


**8. Error handling && Null handling**

- Error handling : mostly Result<R,E> or panic!
- Null handling : Option<T> {Some(T), None}

**9. generic**

- A concept that let us refer to group of type instead of one specific type Ex. Option<T>

**10. trait; What does impl mean? What is the difference between fn f(x: impl A, y: impl A) and fn f<T: A>(x: T, y: T)**

- `trait` : A concept that let us define behaviors that some types may have in common.
- `impl` : A keyword to implement method

- `fn f(x: impl A, y: impl A)` : `x` and `y` must be any variable that have definition of trait `A`
- `fn f<T: A>(x: T, y: T)` : Same as above but `x` and `y` must be the same types

**11. lifetime parameter; how to declare and use it?**

- Use `&'a` to declare lifetime parameters

        fn longest<'a> (&'a, ...){
        
        }

- Use when we want to return reference of the parameter 
- It is like the guaranteed that the return value will have shorter lifetime than the params (if num params>1, lifetime of return values will be consider from the min lifetime of param)


**12. closure; capturing references vs moving ownership; fn() -> A vs Fn() -> A vs FnMut() -> A vs FnOnce() -> A; what is the difference between closure and normal function?**

- capturing references vs moving ownership
  - A closure can capture ref like | | {println!("{}",some_list)}
  - Or moving ownership like move | | {println!("{}",some_list)}

- fn() -> A vs Fn() -> A vs FnMut() -> A vs FnOnce() -> A 
  - fn() -> A : Function pointer. A pointer to closure that contains nothing or function 
  - FnOnce() -> A : Trait. Apply to func/closure that will be call once including the one that move ownership of data.
  - Fn() -> A : Trait. Apply to immutable func/closures.
  - FnMut() -> A : Trait. Apply to mutable func/closures.

- closure vs func

    Closure may contains data (by capturing/moving) but func contain no data.

**13. into_iter vs iter vs iter_mut; how iterator works?**

- `v1.into_iter()` => the iterator that return will take ownership of values in v1

- `v1.iter()` => the iterator that return will contain the immutable ref of values in v1

- `v1.iter_mut()` => the iterator that return will contain the mutable ref of values in v1

- collection vs iterator

    Iterator seems to have better performance because it is zero-cost abstraction and it do the unrolling in low=level.

**14.Smart Pointers; Box vs Rc vs RefCell; What are the pros and cons of using one**

- Box
  - Pros: Provide indirection (storing pointer instead of the actual data) and storing data in heap.
  - Cons: Overhead in accessing data since it store data in heap.

- Rc : Reference count. 
  - Pros: Allow multiple owner to the value.
  - Cons: Might cause the memory leak due to reference cycle (Weak reference can solve this!). Also the overhead from storing the count of reference together with data.

- RefCell (keywords: interior mutability)
  - Pros: Allow editing values when there are immutable ref to the values since it check borrowing rule on runtime.
  - Cons: 
    - Might cause panic at runtime.
    - worse runtime performance due to checking borrowing rule at runtime.
  - RefCell is usually used with Rc.

**15. mpsc (and many other concurrency tools)**
- mpsc stands for multiple producer and single consumer

        use std::sync::mpsc;
        (tx, rx) = mpsc::channel();

- We can create multiple producer by cloning tx

        tx1 = tx.clone();

- And we can use this to send data between thread like moving the tx/rx to different threads.

        thread::spawn(move || tx.send("hello"));

**16. Arc, Mutex, RwLock**

- Arc
    - Atomic reference counted
    - Similar to rc but can be used between threads.

- Mutex

    - We use it to have a mutable reference in multiple threads.
    - Similar to RefCell but can be used between threads.

- RwLock
    - Allows either multiple readers or one writer at a time.


**17. Send and Sync (trait name)**

- `Send` 

    - A marker traits that indicates if the ownership of type can be transferred between threads.
    - Rc does not implement this

- `Sync`
    - A marker traits that indicates if it safe to reference to the type from multiple threads.
    - Rc and RefCell don't implement this.

- Basically, these trait is implemented autonomously if subtype of the type already implement these two traits.


**17. dyn; What is it?; dyn A vs impl A**

- It is a prefix to use a trait object to make it object safe.

- Since Rust don't know the actual return type at the compile time if we want to return type that implement specific traits, we have to use keyword dyn to make Rust determine return type at the runtime. This is call dynamically dispatched.

- `dyn A` vs `impl A`
    - `impl` only allows monomorphic return while `dyn` allow polymorphic return 

            fn f(a: bool) -> impl Bar {
                if a {
                    Foo { ... }
                } else {
                    Baz { ... }
                }
            }

        even though Foo and Baz implement Bar, this code will be error.

    - dyn have to be used with pointer since we do not know the size of types at compile time.
    
    - impl allows mutiple trait bound like impl (Foo + Bar) but dyn does not.

**18. how to return polymorphic return type that has the same trait?**

- Declare the return type as a trait object using dyn. `Box<dyn Trait>`

# Post Book
**19. Future, async, await; how async/await differ from thread?**

- `Future` 
    - A trait which will be a trait bound to return value of asynchronous function.
    - Have Pending or Success status to indicate if the result is ready to use.
    - Similar to Promise in many language
    - To get a value inside we have to await or call poll

- `async`
    - Use it in a function signature to declare the asynchronous function.
    - Asynchronous function will return Future type and run asychronously with part of program (Continuously switch control between function).

- `await`
    - Use it to get the value from asynchronous function
    - It will call the poll method of the Future trait to get the value.

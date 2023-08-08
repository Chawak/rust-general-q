**1) ทำความเข้าใจกับ  The following pointer types: &, &mut, Box, Rc, Arc, const,mut  ว่าต่างกันอย่างไร**

`&` : Immutable Referencing 
  - Read-only value of the data
  - Doesn't take the ownership of the data
`&mut` => Mutable Referencing
- Editable value of the data.
- Doesn't take the ownership of the data
- Can be borrow once in the scope.
  
`const` => Constant.

`mut` => mutable. Indicate that the value can be change.

`Box` 
  - Smart pointer 
  - Take ownership of data
  - Move data from stack to heap

`Rc` : Reference-counted
  - Smart pointer
  - Allow the data to have multiple owner.
  - Consider dropping data on its strong reference.
  - Move data from stack to heap.
  - Immutablle borrow.
  - Can cause the memory leaks due to the reference cycle.
  - Allow each elements in data structure that construct by it to be independent from others.

`Arc` : Atomic reference-counter
  - Smart pointer
  - Similar to Rc but used when we want to share value across thread.

**2) Rust slice คืออะไร มาใช้ manipulate อะไรได้บ้าง**

- `str`, `[T]`
- Slice return subsequence of string, vector, array (collection).
- And can only exists in pointer since its unknown sized at compile time.

- **From P'Rew answer** : &[T] is more preferable than &Vec<T> as parameter because the &[T] will accept the partial vector

**3) trait คืออะไร ใช้งานตอนไหน มีผลกับการออกแบบ architecture ยังไง**

- The concept that we can shared method and abstraction on different type of data.

- Being used when we want to have a function/method that accept any types that provide method A. For example, trait Display to print the value.

**4) Iterator คืออะไร ใช้ยังไง เกี่ยวพันกับ Rust standard collection ยังไง**

- A trait that allow borrowing and accesing the value sequences. 
- Use `.iter()`, `.iter_mut()`, `.to_iter()` to get the iterator.
- Can also be converted to collection by `.collect`.

- It has better performance than collections due to it performs unrolling in the low-level.

**5) Rust standard collection มีอะไรบ้าง มีอะไรที่เหมือนกันและต่างกันบ้าง**

- Vector
  - append in average O(1)
  - access value by index in O(1)

- VecDeque
  - double-end queue
  - use when you want to append the value in both side of sequence in O(1)

- HashMap 
  - hash the key and associated value
  - lookup the value in O(1)
  - check if the value exists O(1)

- BTreeMap 
  - Map implementation using binary tree.
  - sort the key of the key-value pairs
  - map:unordered_map => BTreeMap:HashMap

- HashSet/BTreeSet 
  - Store only key

- BinaryHeap 
  - PriorityQueue

**6) Closures คืออะไร เอามาใช้ทำอะไรได้บ้าง เกี่ยวพันกับ Iterators อย่างไร**

- A function-like types. We can used it like function.

- It also can contains the value by referencing or moving the data inside.

**7) Module คืออะไร เราสามารถสร้าง nest modules ได้มั้ย และถ้าทำ ทำยังไง**

- A subset of code that we seperate according to its functionality and its publicity.

- A module can be declare like:

        mod some_module1
        {
            fn some_fn1(){
                ...
            }
        
            mod some_module2{
                ...
            }
        }

- Or like `mod some_module1` in `main.rs` and `mod some_module2` in `some_module1.rs` or `some_moduel1/mod.rs`

**8) เราใช้ "as" กับ "transmute" ต่างกันอย่างไร**
- `as` and `transmute` is the way to cast type in rust
  - `as`
    
        let x : i32 = 5;
        let y = x as i64;

  - transmute
    
        let a = [0u8, 1u8, 0u8, 0u8];
        let b = mem::transmute::<[u8; 4], u32>(a);
        let c: u32 = mem::transmute(a);

- `as` is safe while `transmute` isn't. 

- **From P'Rew's answer:**
  - transmute doesn't really convert a type, it just re-interpret a type as a new type.
  - notable difference is when casting between u32 => f32 or the like. as will just change the type while trying to maintain the same value ex. 8 => 8.0. However, transmute will just reinterpret those 32 bits with the format according to IEEE754.


**9) Self, &Self, &mut Self  ต่างกันยังไง   &self กับ &Self เท่ากันมั้ย ความหมายคืออะไร?**

- `Self` -> Refer to struct type, can be used as return type
- `&Self` -> Reference to the struct type
- `&mut Self` -> Mutable reference to the struc type.

- `&self` vs `&Self`

  - &self is the short for self : &Self

- Example of usage
  
        impl Person {
        fn me(self) -> Self {
        ....
        }
        }

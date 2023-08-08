**1) อธิบายความแตกต่างของ str and String? และถ้าเราต้องการ access Slice ที่ชี้ไปยัง string จะต้องทำยังไง**

- `str` : Slice of string which is mostly seen as &str.
- `String` : The String type
- Accessing slice => `s[1..4]`

**2) [T] กับ Vec<T> มีความสัมพันธ์กันอย่างไร**

- `Vec<T>` vector of generic type T
- `[T]` Slice type (which is unsized) of vector with generic type T

- **From P'Rew's answer**
  - Unsized type can only be used with the function with trait bound !Sized or ?Sized.


**3) Static lifetime หมายความว่าอะไร**

- The variable will be valid until the end of program.

**4) เราจะ define explicit lifetime ได้อย่างไร และมี ประโยชน์ หรือ scope การใช้งานอย่างไร**

    fn some_fn<'a> (some_var: &'a some_type) -> &'a some_type
    
    struct SomeStruct<'a>{
      x: &'a str
    }

    impl<'a> SomeTrait<'a> for SomeStruct<'a>{
      fn some_fn(&self, some_arg:&'a str) -> &'a str
    }

- It is the explicit declaration that ensure the return value which is a reference will be valid as long as the parameter (which is also a reference) is valid.

**5) เล่าปัญหาที่สามารถเกิดขี้นกับ code นี้แล้วเสนอวิธีแก้ไข**

    struct Plusplus {
      value: &mut i32
    }
    
    impl Plusplus {
      pub fn plusplus (&mut self) -> i32 {
        *self.value += 1;
        *self.value
      }
    }

- Problem: Did not declare lifetime.
- Solution:

      struct Plusplus<'a> {
          value: &'a mut i32
        }
        
      impl<'a> Plusplus<'a> {
      pub fn plusplus (&mut self) -> i32 {
          *self.value += 1;
          *self.value
      }
      }


**6) อธิบายความแตกต่าง และ behavior ของ panic,  และการ propagate error โดยใช้ Result และ Option  ยกตัวอย่าง usecase ที่ เหมาะสมสำหรับแต่ละกรณี และวิธี handling จาก upstream calling function**

- Panic `panic!` will stop the program in case of unrecoverable problems while using Result with won't interrupt the program but return `Result<T,E>`. `Ok(T)` in cases of success and `Err()` in cases that error occured.

- `Option<T>`: A way to wrap nullable value (`Some(T)` and `None`)

- We can use match to extract value inside Option and Result.

      match val {
          Ok(T) => T,
          Err() => ...
      }

- Or we can use `?` operator to handle return value

      fn some_fn() -> Result<i32,String>{
          let x = some_other_fn()?;
          // some_other_fn() have to return Err(String) too to match the function definition
          // If error occur in calling some_other_fn(), some_fn() will return the error early
          // otherwise, ? will return the value inside Ok()
      
      }

- The best practice is to use Result and manually handle the error.

**7) เปรียบเทียบความแตกต่างของ  as,  Into<>, From<> และ Transmute**
- transmute try to reinterpret data
- as is used to transform some set of primitive data types
- into and from convert value to another type using trait. Implement from infer into


**7) Enum ใน Rust มีข้อที่เหมือน และ แตกต่างจากภาษาอื่นๆ อย่างใรบ้าง**

- Rust enum doesn't require a associate primitive value to each enum value.
- It can also store value inside enum. For example.

      enum IP{
          V4(i32,i32,i32,i32),
          V6(String),
          V11 {x:i32, y:i32},
      }


**8) Rust Attributes คืออะไร เอามาใช้ประโยชน์ได้อย่างไรบ้าง**

- Interpreted metadatum. For example #[test], #[derive(SomeTrait)]

**9) ใน std::thread จงอธิบายความแตกต่างของการ pass Closure เทียบกับ pass Function ในการเรียก spawn ในการสร้าง Thread**

- Passing closure can capture the value like Mutex, or Arc to another threads while passing function cannot do it directly.

**10) Send trait marker กับ Sync trait marker ต่างกันอย่างไร และมีข้อยกเว้นอะไรบ้าง**

- `Send` 

  - A marker traits that indicates if the ownership of type can be transferred between threads.
  - Rc does not implement this

- `Sync`
  - A marker traits that indicates if it safe to reference to the type from multiple threads.
  - Rc and RefCell don't implement this.

- Manually implement these trait is unsafe, so basically Rust automatically do this for programmers.
- Raw pointer does not implement these traits.

**11) อธิบาย concept ความต่าง ของ Rc กับ Arc**
- `Rc` : Reference count. 
  - Pros : Allow multiple owner to the value.
  - Cons : Might cause the memory leak due to reference cycle (Weak reference can solve this!). Also the overhead from storing the count of reference together with data.

- `Arc` : Atomic reference counted
  - Similar to `Rc` but can be used between threads.

**12) อธิบายว่า Rust handle race condition อย่างไรได้บ้าง**

- **Value can have only one mutable reference** : ensure that no editing from many threads. Also check inner mutability and multiple ownership by Send and Sync trait.

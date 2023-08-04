1) ใน modern Operating System ที่อยู่บน Modern CPU hardware สามารถมี Race Condition ได้จากสถานการณ์ใหนได้บ้าง 
2) Weak pointer คืออะไร ปรกติเราใช้กับสถานการณ์แบบไหน

A type of pointer that won't increasing Rc::strong_count.
Use it with Rc to avoid possible memory leaks from reference cycle.
Dropping the data will consider from strong reference not weak reference.

3) อธิบาย Nullable Pointer  และโอกาสที่จะเกิด รวมถึงวิธีการ handle Nullable Pointer

The raw pointer can point to address that is null. So dereferencing it is unsafe.

You need to check dangling and make sure the pointer is valid for derefencing.

4) ถ้าเราต้องการจะ ทำ Binding C++ library from Rust จะมีวิธี implement อย่างไรได้บ้าง และเราต้องระวังการ handle data type ใหนบ้าง 
5) ยกตัวอย่าง Use case ของ Local Thread Variable และถ้าเราไม่ใช้ Local Thread Variable แล้ว เราสามารถใช้อะไรเป็น alternative ได้บ้าง บอกช้อดีและเสียของ Alternative Strategy แต่ละอย่าง
6) ถ้ามี CPU 8 core จะมีการ run concurrent thread ได้กี่ Thread?
7) ถ้ามี thread สองชุด โดย Thread (1) พยายาม acquire mutex ของ A, then B.  Thread (2) พยายาม acquire mutex ของ B, then A  โดย system underneath ที่ execute ทั้งสอง thread เป็น CPU single core ระบบของเราจะมีโอกาสเกิด Deadlock ได้หรือไม่ ทำไม?

Possible if the order of execution is
- Thread 1 acquire A 
- Thread 2 acquire B 
- Thread 1 try to acquire B 
- Thread 2 try to acquire A 


8) จะมี strategy อย่างไรที่จะทำให้ระบบที่ เขียน cross order lock แบบข้อ 7 ไม่เกิด Deadlock แม้ว่าจะเป็น multi core CPU?
Using try_lock of different order of the accessing

9) อธิบาย enum Ordering ในการใช้ Atomic primitive type
10) Mutex lock behavior ใน Rust ต่างกับภาษาอื่นอย่างไรบ้าง
11) System interrupt มีผลต่อ การทำ ordering อย่างไรบ้าง และเราจะป้องกันได้แบบใหน
12) อธิบาย Poison algorithm ของ Rust Mutex

A mutex value will be poisoned if any thread panic while acquire a lock.
Use into_inner() to obtain the data inside.

13) ยกตัวอย่าง valid use case ของ Condition Variable 
Use when we need to block other threads from accessing data 

14) เปรียบเทียบ RwLock กับ mpsc  ในเชิงของ use case และ cost การใช้งาน
Since RwLock allows multiple read-only reference the cost is lower than mpsc which has multiple producer and require buffer allocation for queue.


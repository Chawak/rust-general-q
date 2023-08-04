use std::alloc::{self, Layout};
use std::ptr::{self, NonNull};
use std::ops::{ Index, IndexMut};

pub struct MyVec<T> {
    data: NonNull<T>,
    len: usize,
    cap: usize,
}

impl<T> MyVec<T> {
    pub fn new()-> Self{
        MyVec {data: NonNull::dangling(),len: 0,cap:0}
    }
    fn expand(&mut self) {
        let (new_cap, new_layout) = if self.cap == 0 {
            (1, Layout::array::<T>(1).unwrap())
        } else {
            let new_cap = self.cap * 2;
            let new_layout = Layout::array::<T>(new_cap).unwrap();
            (new_cap, new_layout)
        };

        let new_ptr = if new_cap==1 {
            unsafe{
                alloc::alloc(new_layout)
            }
        } else {
            let old_ptr = self.data.as_ptr() as *mut u8;
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            unsafe{
                alloc::realloc(old_ptr, old_layout, new_layout.size())
            }
        };

        self.data = match NonNull::new(new_ptr as *mut T){
            Some(p) => p,
            None => alloc::handle_alloc_error(new_layout),
        };
        self.cap = new_cap;
    }
    pub fn push(&mut self, item: T) {
        if self.len == self.cap{
            self.expand();
        }

        unsafe {
            ptr::write(self.data.as_ptr().add(self.len), item);
        }
        self.len+=1;
    }

    pub fn pop(&mut self) -> Option<T>{
        if self.len == 0{
            return None
        }

        self.len -= 1;
        
        unsafe {
            Some(ptr::read(self.data.as_ptr().add(self.len)))
        }
    }

    pub fn len(&self) -> usize{
        self.len
    }
    pub fn capacity(&self) -> usize{
        self.cap
    }
}

impl<T> Index<usize> for MyVec<T>{
    type Output = T;

    fn index(&self, idx: usize) -> &Self::Output {
        assert!(idx<self.len);        
        unsafe {
            &*self.data.as_ptr().add(idx)
        }

    }
}
impl<T> IndexMut<usize> for MyVec<T> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        assert!(idx<self.len);        
        unsafe {
            &mut *self.data.as_ptr().add(idx)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::MyVec;
    #[test]
    fn test_new_vec() {
        let vec : MyVec<i32>= MyVec::new();
        assert_eq!(vec.len(),0);
        assert_eq!(vec.capacity(),0);
        
    }
    #[test]
    fn test_immutable_index() {
        let mut vec : MyVec<usize> = MyVec::new();
        for i in 1..=5{
            vec.push(i);
        }
        assert_eq!(vec[0],1);
        assert_eq!(vec[1],2);
        assert_eq!(vec[2],3);
        assert_eq!(vec[3],4);
        assert_eq!(vec[4],5);
    }
    #[test]
    fn test_mutable_index() {
        let mut vec : MyVec<usize> = MyVec::new();
        for i in 1..=5{
            vec.push(i);
        }
        vec[0]= 8;
        assert_eq!(vec[0],8);
        assert_eq!(vec[1],2);
        assert_eq!(vec[2],3);
        assert_eq!(vec[3],4);
        assert_eq!(vec[4],5);
    }

    #[test]
    fn test_expand_size() {
        let mut vec : MyVec<usize> = MyVec::new();
        let cap_arr = [1,2,4,4,8];
        for i in 1..=5{
            vec.push(i);
            assert_eq!(vec.len(), i);
            assert_eq!(vec.capacity(), cap_arr[i-1]);
        }
        
    }

    #[test]
    fn vec_of_str_ref() {
        let string = String::from("scope string");
        let mut vec: MyVec<&str> = MyVec::new();
        vec.push(&string);
        vec.push("Static string");
        assert_eq!(2, vec.len());
        assert_eq!(2, vec.capacity());
        assert_eq!(Some("Static string"), vec.pop());
        assert_eq!(Some("scope string"), vec.pop());
        assert_eq!(0, vec.len());
        assert_eq!(2, vec.capacity());
    }

    #[test]
    fn test_pop() {
        let mut vec : MyVec<usize> = MyVec::new();
        for i in 1..=5{
            vec.push(i);
        }
        let expected_pop_result = [5,4,3,2,1];
        let mut idx =0;
        while let Some(elem) = vec.pop() {
            assert_eq!(expected_pop_result[idx],elem);
            idx+=1
        }
        assert_eq!(vec.len(),0);
    }
}

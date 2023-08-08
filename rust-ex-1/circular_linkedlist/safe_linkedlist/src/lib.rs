
use core::panic;
use std::{sync::{Arc,Mutex}, borrow::BorrowMut};
struct Node<T>{
    value : T,
    next_node : Option<Arc<Mutex<Node<T>>>>,
    is_tail: bool
}

struct IterNode<T>{
    next: Option<Arc<Mutex<Node<T>>>>,
}

pub struct CircularLinkedlist<T>{
    start_node : Option<Arc<Mutex<Node<T>>>>,
    len : usize
}

impl<T> Iterator for IterNode<T> {
    type Item = Arc<Mutex<Node<T>>>;
    fn next(&mut self) -> Option<Self::Item> {
        if let None = self.next {
            None
        } else {
            let res = Arc::clone(&self.next.as_ref().unwrap());
            self.next = Some(Arc::clone(&res.lock().unwrap().next_node.as_ref().unwrap()));
            Some(res)
        }
    }
}

impl<T : std::fmt::Display> CircularLinkedlist<T> {
    pub fn new() -> Self {
        CircularLinkedlist{start_node: None, len : 0}
    }
    fn get_tail_node(&mut self) -> Arc<Mutex<Node<T>>>{

        let mut tail = Arc::clone(&self.start_node.as_ref().unwrap());
        while !tail.lock().unwrap().is_tail{
            let new_tail = Arc::clone(&tail.lock().unwrap().next_node.as_ref().unwrap());
            tail = new_tail;
        }
        tail

    }
    pub fn push(&mut self, elem : T){

        if let None = self.start_node{

            let new_node = Node{value : elem , next_node: None, is_tail : true };

            self.start_node = Some(Arc::new(Mutex::new(new_node)));

            (*self.start_node.as_mut().unwrap().borrow_mut()).lock().unwrap().next_node = Some(Arc::clone(&self.start_node.as_ref().unwrap()));

        }
        else {

            let tail = self.get_tail_node();

            let new_node = Node{value: elem, next_node: Some(Arc::clone(&self.start_node.as_ref().unwrap())), is_tail : true};

            tail.lock().unwrap().next_node = Some(Arc::new(Mutex::new(new_node)));
            tail.lock().unwrap().is_tail = false;
        }
        self.len+=1;
    }

    pub fn pop(&mut self) -> Option<T>{
        if let None = self.start_node{
            None
        }
        else {
            let mut last: Arc<Mutex<Node<T>>>;

            println!("Enter block 1");
            {

            
            let mut before_tail = Arc::clone(&self.start_node.as_ref().unwrap());
            last = Arc::clone(&self.start_node.as_ref().unwrap());

            while !last.lock().unwrap().is_tail{
                let new_tail = Arc::clone(&last.lock().unwrap().next_node.as_ref().unwrap());
                before_tail = last;
                last = new_tail;
            }

            before_tail.lock().unwrap().next_node = Some(Arc::clone(&self.start_node.as_ref().unwrap()));
            before_tail.lock().unwrap().is_tail = true;
            }

            println!("Arc strong count {}", Arc::strong_count(&last));
            
            if Arc::strong_count(&last)>1{
                self.start_node.as_mut().unwrap().lock().unwrap().next_node = None;
                self.start_node = None;
            }

            let last_refcell = match Arc::try_unwrap(last){
                    Ok(t) => t,
                    Err(_) => panic!("Should not be possible"),
            };
            self.len -= 1;
            println!("End pop");
            Some(last_refcell.into_inner().unwrap().value)
        }
        
    }

    fn iter_mut(&mut self) -> IterNode<T> {

        if let None = self.start_node{
            return IterNode{next: None}
        }
        IterNode { next:  Some(Arc::clone(&self.start_node.as_ref().unwrap()))}
        
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circular() {
        let mut my_list: CircularLinkedlist<i32> = CircularLinkedlist::new();

        let ans = [3,4,5];
        my_list.push(3);
        my_list.push(4);
        my_list.push(5);

        {
            for (i, val) in my_list.iter_mut().take(3).enumerate() {
                assert_eq!(ans[i], val.lock().unwrap().value);
                // println!("val lock unwrap {}",val.lock().unwrap().value);
            }
        }

        assert_eq!(my_list.pop(), Some(ans[2]));
        assert_eq!(my_list.pop(), Some(ans[1]));
        assert_eq!(my_list.pop(), Some(ans[0]));


    }
    #[test]
    fn test_string_circular() {
        let mut my_list: CircularLinkedlist<String> = CircularLinkedlist::new();

        let ans = [1,2,3];
        my_list.push(ans[0].to_string());
        my_list.push(ans[1].to_string());
        my_list.push(ans[2].to_string());

        for (i, val) in my_list.iter_mut().take(6).enumerate() {
            val.lock().unwrap().value.push_str(&i.to_string());
        }
        assert_eq!(my_list.pop().unwrap(),"325".to_string());
        assert_eq!(my_list.pop().unwrap(),"214".to_string());
        assert_eq!(my_list.pop().unwrap(),"103".to_string());
    }
}

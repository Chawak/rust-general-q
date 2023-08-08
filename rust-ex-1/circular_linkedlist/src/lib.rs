
use core::panic;
use std::{rc::Rc, cell::RefCell, collections::binary_heap::Iter};
struct Node<T>{
    value : T,
    next_node : Option<Rc<RefCell<Node<T>>>>,
    is_tail: bool
}

struct IterNode<T>{
    next: Option<Rc<RefCell<Node<T>>>>,
}

pub struct CircularLinkedlist<T>{
    start_node : Option<Rc<RefCell<Node<T>>>>,
    len : usize
}

impl<T> Iterator for IterNode<T> {
    type Item = Rc<RefCell<Node<T>>>;
    fn next(&mut self) -> Option<Self::Item> {
        if let None = self.next {
            None
        } else {
            let res = Rc::clone(&self.next.as_ref().unwrap());
            self.next = Some(Rc::clone(&res.borrow().next_node.as_ref().unwrap()));
            Some(res)
        }
    }
}

impl<T : std::fmt::Display> CircularLinkedlist<T> {
    pub fn new() -> Self {
        CircularLinkedlist{start_node: None, len : 0}
    }
    fn get_tail_node(&mut self) -> Rc<RefCell<Node<T>>>{

        let mut tail = Rc::clone(&self.start_node.as_ref().unwrap());
        while !tail.borrow().is_tail{
            let new_tail = Rc::clone(&tail.borrow().next_node.as_ref().unwrap());
            tail = new_tail;
        }
        tail

    }
    pub fn push(&mut self, elem : T){

        if let None = self.start_node{

            let new_node = Node{value : elem , next_node: None, is_tail : true };

            self.start_node = Some(Rc::new(RefCell::new(new_node)));

            (*self.start_node.as_mut().unwrap().borrow_mut()).next_node = Some(Rc::clone(&self.start_node.as_ref().unwrap()));

        }
        else {

            let tail = self.get_tail_node();

            let new_node = Node{value: elem, next_node: Some(Rc::clone(&self.start_node.as_ref().unwrap())), is_tail : true};

            tail.borrow_mut().next_node = Some(Rc::new(RefCell::new(new_node)));
            tail.borrow_mut().is_tail = false;
        }
        self.len+=1;
    }

    pub fn pop(&mut self) -> Option<T>{
        if let None = self.start_node{
            None
        }
        else {
            let last: Rc<RefCell<Node<T>>>;

            {
            let mut before_tail = Rc::clone(&self.start_node.as_ref().unwrap());

            while !before_tail.borrow().next_node.as_ref().unwrap().borrow().is_tail{
                let new_btail = Rc::clone(&before_tail.borrow().next_node.as_ref().unwrap());
                before_tail = new_btail;
            }

            last = Rc::clone(&before_tail.borrow().next_node.as_ref().unwrap());

            before_tail.borrow_mut().next_node = Some(Rc::clone(&self.start_node.as_ref().unwrap()));
            before_tail.borrow_mut().is_tail = true;
            }
            
            if Rc::strong_count(&last)>1{
                self.start_node.as_mut().unwrap().borrow_mut().next_node = None;
                self.start_node = None;
            }

            let last_refcell = match Rc::try_unwrap(last){
                    Ok(t) => t,
                    Err(_) => panic!("Should not be possible"),
            };
            self.len -= 1;
            Some(last_refcell.into_inner().value)
        }
        
    }

    fn iter_mut(&mut self) -> IterNode<T> {

        if let None = self.start_node{
            return IterNode{next: None}
        }
        IterNode { next:  Some(Rc::clone(&self.start_node.as_ref().unwrap()))}
        
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
        let mut tail = Rc::clone(&my_list.start_node.as_ref().unwrap());
        let mut idx = 0;
        while !tail.borrow().is_tail{
            assert_eq!(tail.borrow().value,ans[idx]);
            idx+=1;
            let new_tail = Rc::clone(&tail.borrow().next_node.as_ref().unwrap());
            tail = new_tail;
        }
        assert_eq!(tail.borrow().value,ans[idx]);
        let new_tail = Rc::clone(&tail.borrow().next_node.as_ref().unwrap());
        tail = new_tail;
        assert_eq!(tail.borrow().value,ans[0]);
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
            val.borrow_mut().value.push_str(&i.to_string());
            
        }
        assert_eq!(my_list.pop().unwrap(),"325".to_string());
        assert_eq!(my_list.pop().unwrap(),"214".to_string());
        assert_eq!(my_list.pop().unwrap(),"103".to_string());
    }
}

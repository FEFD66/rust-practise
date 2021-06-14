use std::ptr::{null, NonNull};

pub struct UnsafeLinkedList<T>{
    head:Option<NonNull<Node<T>>>,
    tail:Option<NonNull<Node<T>>>,
    length:usize
}
#[derive(Debug)]
struct Node<T>{
    data:T,
    next:Option<NonNull<Node<T>>>,
    prev:Option<NonNull<Node<T>>>
}
impl<T> UnsafeLinkedList<T>{
    pub fn new() ->Self{
        UnsafeLinkedList{ head:None,tail:None,length:0 }
    }
    pub fn push_back(&mut self,val:T){
        let node=Box::new(Node{ data:val,next:None,prev:None });
        if let Some(mut tail)=self.tail{
            unsafe {
                tail.as_mut().next=Some(Box::leak(node).into());
                self.tail=tail.as_mut().next;
                self.length=self.length+1
            }
        }else {
            self.head=Some(Box::leak(node).into());
            self.tail=self.head;
            self.length=1;
        }
    }
}
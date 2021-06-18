use std::rc::{Weak, Rc};
use std::cell::RefCell;

pub struct SafeLinkedList<T:Sized>{
    chain:Option<Rc<RefCell<MyLinkedNode<T>>>>,

    first:Option<Weak<RefCell<MyLinkedNode<T>>>>,
    last:Option<Weak<RefCell<MyLinkedNode<T>>>>,
    length:usize
}
impl<T:Sized> SafeLinkedList<T>{
    pub fn new()->Self{
        SafeLinkedList {
            chain:None,
            first:None,
            last:None,
            length:0
        }
    }
    pub fn push_back(mut self, val:T){
        if let None=self.chain{
            self.chain=Some(Rc::new(RefCell::new(MyLinkedNode::new(val))));
            self.last=Some(Rc::downgrade(self.chain.as_ref().unwrap()));
            self.first=Some(Rc::downgrade(self.chain.as_ref().unwrap()));
            self.length=1;
            return;
        }
        let last=self.last.unwrap().upgrade().unwrap();
        let new=Rc::new(RefCell::new(MyLinkedNode::behind(val,&last)));
        let mut last=last.borrow_mut();
        last.set_next(new);
        self.length=self.length+1;
    }
}

struct MyLinkedNode<T:Sized>{
    data:T,
    next:Option<Rc<RefCell<MyLinkedNode<T>>>>,
    prev:Option<Weak<RefCell<MyLinkedNode<T>>>>
}
impl<T:Sized> MyLinkedNode<T>{
    pub fn new(val:T)->Self{
        MyLinkedNode{
            data:val,
            next:None,
            prev:None,
        }
    }

    pub fn behind(val:T,prev:&Rc<RefCell<MyLinkedNode<T>>>)->Self{
        MyLinkedNode{
            data:val,
            next:None,
            prev:Some(Rc::downgrade(prev))
        }
    }
    pub fn set_next(&mut self,next:Rc<RefCell<MyLinkedNode<T>>>){
        self.next=Some(next);
    }
}
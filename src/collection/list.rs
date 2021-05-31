use std::rc::{Weak, Rc};

pub struct MyLinkedList<T:Sized>{
    chain:Option<Rc<MyLinkedNode<T>>>,
    first:Option<Weak<MyLinkedNode<T>>>,
    last:Option<Weak<MyLinkedNode<T>>>,
    length:usize
}
impl<T:Sized> MyLinkedList<T>{
    pub fn new()->Self{
        MyLinkedList{
            chain:None,
            first:None,
            last:None,
            length:0
        }
    }
    pub fn push_back(mut self, val:T){
        if let None=self.chain{
            self.chain=Some(Rc::new(MyLinkedNode::new(val)));
            self.last=Some(Rc::downgrade(self.chain.as_ref().unwrap()));
            self.first=Some(Rc::downgrade(self.chain.as_ref().unwrap()));
            self.length=1;
            return;
        }
        let mut last:Rc<MyLinkedNode<T>>=self.last.unwrap().upgrade().unwrap();
        let new=Rc::new(MyLinkedNode::behind(val,&last));
        last.set_next(new);
        self.length=self.length+1;
    }
}
struct MyLinkedNode<T:Sized>{
    data:T,
    next:Option<Rc<MyLinkedNode<T>>>,
    prev:Option<Weak<MyLinkedNode<T>>>
}
impl<T:Sized> MyLinkedNode<T>{
    pub fn new(val:T)->Self{
        MyLinkedNode{
            data:val,
            next:None,
            prev:None,
        }
    }

    pub fn behind(val:T,prev:&Rc<Self>)->Self{
        MyLinkedNode{
            data:val,
            next:None,
            prev:Some(Rc::downgrade(prev))
        }
    }
    pub fn set_next(&mut self,next:Rc<Self>){
        self.next=Some(next);
    }
}
mod safe_linklist;
mod unsafe_linkist;

#[cfg(test)]
mod tests{
    use crate::collection::safe_linklist::SafeLinkedList;
    use std::collections::LinkedList;
    use std::fmt::Error;
    use crate::collection::unsafe_linkist::UnsafeLinkedList;

    #[test]
    fn list_linked()->(){
        let mut ll=LinkedList::new();
        ll.push_back(1);

        let list: SafeLinkedList<i32>= SafeLinkedList::new();
        let x="yesa";
        let x=String::from("yea");

        let mut un=UnsafeLinkedList::new();
        un.push_back(1);
        un.push_back(1);

    }

}
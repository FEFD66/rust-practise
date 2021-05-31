mod list;

#[cfg(test)]
mod tests{
    use crate::collection::list::MyLinkedList;

    #[test]
    fn list_linked(){
        let list:MyLinkedList<i32>=MyLinkedList::new();
    }

}
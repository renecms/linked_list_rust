#![allow(dead_code)]
use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

type NodeRc<T> = Rc<RefCell<LinkedListNode<T>>>;

#[derive(Debug, Clone)]
struct LinkedListNode<T: Display + Clone> {
    data: T,
    next: Option<NodeRc<T>>,
}

#[derive(Debug, Clone)]
struct LinkedList<T: Display + Clone> {
    head: Option<NodeRc<T>>,
    tail: Option<NodeRc<T>>,
    count: usize,
}

#[derive(Debug, Clone)]
struct LinkedListIterator<T: Display + Clone> {
    next_node: Option<NodeRc<T>>,
}

impl<T> LinkedList<T>
where
    T: Display + Clone,
{
    pub fn new() -> LinkedList<T> {
        LinkedList::<T> {
            head: None,
            tail: None,
            count: 0,
        }
    }

    pub fn insert(&mut self, data: T) {
        match &self.head {
            Some(node) => {
                self.head = Self::create_node(data, Some(node.clone()));
            }
            None => {
                self.head = Self::create_node(data, None);
                self.tail = self.head.clone();
            }
        }
        self.count += 1;
    }

    fn create_node(data: T, next: Option<NodeRc<T>>) -> Option<NodeRc<T>> {
        Some(Rc::new(RefCell::from(LinkedListNode { data, next })))
    }

    pub fn push(&mut self, data: T) {
        let new_node = Rc::new(RefCell::new(LinkedListNode { data, next: None }));
        if let Some(tail) = &self.tail {
            let mut node = tail.borrow_mut();
            node.next = Some(new_node.clone());
        } else {
            self.head = Some(new_node.clone());
        }
        self.tail = Some(new_node);
        self.count += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.clone() {
            Some(node) => {
                self.head = node.borrow().next.as_ref().cloned();
                self.count -= 1;
                Some(node.borrow().data.clone())
            }
            None => None,
        }
    }

    pub fn display(&self) {
        let mut node: Option<NodeRc<T>> = self.head.clone();
        while let Some(current) = node {
            println!("node {}", current.borrow().data);
            node = current.borrow().next.clone();
        }
    }

    pub fn iter(&self) -> LinkedListIterator<T> {
        LinkedListIterator {
            next_node: self.head.clone(),
        }
    }
}

impl<T> Iterator for LinkedListIterator<T>
where
    T: Display + Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.next_node.clone() {
            let current_data: T = node.borrow().data.clone();
            self.next_node = node.borrow().next.clone();
            Some(current_data)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut list: LinkedList<String> = LinkedList::new();
        list.insert(String::from("teste"));
        list.insert(String::from("teste2"));
        list.insert(String::from("teste3"));

        assert_eq!(3, list.count)
    }

    #[test]
    fn test_iter() {
        let mut list: LinkedList<String> = LinkedList::new();
        list.insert(String::from("teste"));
        list.insert(String::from("teste2"));
        list.insert(String::from("teste3"));

        let mut iter = list.iter();

        assert_eq!("teste3".to_string(), iter.next().unwrap());
        assert_eq!("teste2".to_string(), iter.next().unwrap());
        assert_eq!("teste".to_string(), iter.next().unwrap());
        assert_eq!(None, iter.next());

        assert_eq!(3, list.count)
    }

    #[test]
    fn test_pop() {
        let mut list: LinkedList<String> = LinkedList::new();
        list.insert(String::from("teste"));
        list.insert(String::from("teste2"));
        list.insert(String::from("teste3"));

        assert_eq!("teste3".to_string(), list.pop().unwrap());
        assert_eq!("teste2".to_string(), list.pop().unwrap());
        assert_eq!("teste".to_string(), list.pop().unwrap());
        assert_eq!(None, list.pop());

        assert_eq!(0, list.count)
    }

    #[test]
    fn test_push() {
        let mut list: LinkedList<String> = LinkedList::new();
        list.push(String::from("teste"));
        list.push(String::from("teste2"));
        list.push(String::from("teste3"));

        let mut iter = list.iter();

        assert_eq!("teste".to_string(), iter.next().unwrap());
        assert_eq!("teste2".to_string(), iter.next().unwrap());
        assert_eq!("teste3".to_string(), iter.next().unwrap());
        assert_eq!(None, iter.next());

        assert_eq!(3, list.count)
    }
}

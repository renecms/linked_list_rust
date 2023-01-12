#![allow(dead_code)]
use std::{fmt::Display, rc::Rc, marker::PhantomData};

#[derive(Debug)]
struct LinkedListNode<T: Display> {
    data: T,
    next: Option<Rc<LinkedListNode<T>>>,
}

#[derive(Debug)]
struct LinkedList<T: Display> {
    head: Option<Rc<LinkedListNode<T>>>,
    tail: Option<Rc<LinkedListNode<T>>>,
    count: usize,
}

struct LinkedListIterator<'a, T: Display> {
    current_node: Option<Rc<LinkedListNode<T>>>,
    marker: PhantomData<&'a LinkedListNode<T>>
}

impl<T> LinkedList<T>
where
    T: Display,
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
                self.head = Some(Rc::new(LinkedListNode {
                    data,
                    next: Some(node.clone()),
                }));
                self.count += 1;
            }
            None => {
                self.head = Some(Rc::new(LinkedListNode {
                    data,
                    next: None,
                }));
                self.tail = self.head.clone();
                self.count += 1;
            }
        }
    }

    pub fn display(&self) {
        let mut node = &self.head;
        while let Some(current) = node {
            println!("node {}", current.data);
            node = &current.next;
        }
    }

    pub fn iter(&self) -> LinkedListIterator<T> {
        LinkedListIterator {
            current_node: self.head.clone(),
            marker: PhantomData
        }
    }
}

impl<'a, T> Iterator for LinkedListIterator<'a, T>
where
    T: Display,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.current_node.clone() {
            unsafe {
                let cur_node: &LinkedListNode<T> = &*Rc::<LinkedListNode<T>>::as_ptr(&node);
                self.current_node = node.next.clone();
                Some(&cur_node.data)
            }    
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
        assert_eq!("teste3".to_string(), *iter.next().unwrap());
        assert_eq!("teste2".to_string(), *iter.next().unwrap());
        assert_eq!("teste".to_string(), *iter.next().unwrap());
        assert_eq!(None, iter.next());
        assert_eq!(3, list.count)
    }
}

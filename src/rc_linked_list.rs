#![allow(dead_code)]
use std::fmt::Display;
use std::rc::Rc;

#[derive(Debug, Clone)]
struct LinkedListNode<T: Display + Clone> {
    data: T,
    next: Option<Rc<LinkedListNode<T>>>,
}

#[derive(Debug, Clone)]
struct LinkedList<T: Display + Clone> {
    head: Option<Rc<LinkedListNode<T>>>,
    tail: Option<Rc<LinkedListNode<T>>>,
    count: usize,
}

#[derive(Debug, Clone)]
struct LinkedListIterator<T: Display + Clone> {
    next_node: Option<Rc<LinkedListNode<T>>>,
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
                self.head = Some(Rc::new(LinkedListNode {
                    data,
                    next: Some(node.clone()),
                }));
                self.count += 1;
            }
            None => {
                self.head = Some(Rc::new(LinkedListNode { data, next: None }));
                self.tail = self.head.clone();
                self.count += 1;
            }
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        let result;
        match self.head.clone() {
            Some(node) => {
                let new_head = node.next.clone();
                result = Some(node.data.clone());
                self.head = new_head;
                self.count -= 1;
            }
            None => result = None,
        }
        result
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
        if let Some(node) = &self.next_node {
            let current_data = node.data.clone();
            self.next_node = node.next.clone();
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
        
        assert_eq!("teste3".to_string(), *iter.next().unwrap());
        assert_eq!("teste2".to_string(), *iter.next().unwrap());
        assert_eq!("teste".to_string(), *iter.next().unwrap());
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
}

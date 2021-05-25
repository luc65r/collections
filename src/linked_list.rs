use core::ptr::NonNull;
use alloc::boxed::Box;

struct Node<T> {
    element: T,
    next: Option<NonNull<Node<T>>>,
}

pub struct LinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn front(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &unsafe { node.as_ref() }.element)
    }

    pub fn front_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut unsafe { node.as_mut() }.element)
    }

    pub fn back(&self) -> Option<&T> {
        self.tail.as_ref().map(|node| &unsafe { node.as_ref() }.element)
    }

    pub fn back_mut(&mut self) -> Option<&mut T> {
        self.tail.as_mut().map(|node| &mut unsafe { node.as_mut() }.element)
    }

    pub fn push_front(&mut self, element: T) {
        let node = Box::new(Node {
            element,
            next: self.head,
        });
        let node = Some(Box::leak(node).into());

        if self.tail.is_none() {
            self.tail = node;
        }

        self.head = node;
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.map(|node| {
            let node = unsafe { Box::from_raw(node.as_ptr()) };
            self.head = node.next;

            if self.head.is_none() {
                self.tail = None;
            }

            self.len -= 1;
            node.element
        })
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut node = self.head;
        while let Some(n) = node {
            node = unsafe { Box::from_raw(n.as_ptr()) }.next;
        }
    }
}

#[cfg(test)]
mod tests {
    use core::mem;
    use super::*;

    #[test]
    fn empty() {
        let list: LinkedList<u32> = LinkedList::new();
        assert_eq!(list.len(), 0);
        assert_eq!(list.front(), None);
        assert_eq!(list.back(), None);
    }

    #[test]
    fn push_front() {
        let mut list: LinkedList<u32> = LinkedList::new();

        list.push_front(4);
        assert_eq!(list.len(), 1);
        assert_eq!(list.front(), Some(&4));
        assert_eq!(list.back(), Some(&4));

        list.push_front(6);
        assert_eq!(list.len(), 2);
        assert_eq!(list.front(), Some(&6));
        assert_eq!(list.back(), Some(&4));
    }

    #[test]
    fn pop_front() {
        let mut list: LinkedList<u32> = LinkedList::new();

        assert_eq!(list.pop_front(), None);

        list.push_front(12);
        list.push_front(7);

        assert_eq!(list.pop_front(), Some(7));
        assert_eq!(list.pop_front(), Some(12));
        assert_eq!(list.pop_front(), None);

        mem::forget(list);
    }
}

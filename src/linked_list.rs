//! Implementations of linked lists with owned nodes.

use core::{
    ptr::NonNull,
    marker::PhantomData,
};
use alloc::boxed::Box;

/// A singly-linked list with owned nodes.
///
/// The `LinkedList` allows pushing at either end and popping
/// at the front elements in constant time.
pub struct LinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,

    /// Indicates that `LinkedList` owns some `Box<Node>`
    marker: PhantomData<Box<Node<T>>>,
}

struct Node<T> {
    element: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> LinkedList<T> {
    /// Creates an empty `LinkedList`.
    ///
    /// # Examples
    ///
    /// ```
    /// use collections::linked_list::LinkedList;
    ///
    /// let list: LinkedList<u32> = LinkedList::new();
    /// ```
    #[inline]
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
            marker: PhantomData,
        }
    }

    /// Returns the length of the `LinkedList°.
    ///
    /// This operation should compute in *O*(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use collections::linked_list::LinkedList;
    ///
    /// let mut list = LinkedList::new();
    ///
    /// list.push_front(2);
    /// assert_eq!(list.len(), 1);
    ///
    /// list.push_front(1);
    /// assert_eq!(list.len(), 2);
    ///
    /// list.push_back(3);
    /// assert_eq!(list.len(), 3);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Provides a reference to the front element,
    /// or `None` if the list is empty.
    ///
    /// This operation should compute in *O*(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use collections::linked_list::LinkedList;
    ///
    /// let mut list = LinkedList::new();
    /// assert_eq!(list.front(), None);
    ///
    /// list.push_front(1);
    /// assert_eq!(list.front(), Some(&1));
    /// ```
    #[inline]
    pub fn front(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &unsafe { node.as_ref() }.element)
    }

    /// Provides a mutable reference to the front element,
    /// or `None` if the list is empty.
    ///
    /// This operation should compute in *O*(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use collections::linked_list::LinkedList;
    ///
    /// let mut list = LinkedList::new();
    /// assert_eq!(list.front(), None);
    ///
    /// list.push_front(1);
    /// assert_eq!(list.front(), Some(&1));
    ///
    /// match list.front_mut() {
    ///     None => {},
    ///     Some(x) => *x = 5,
    /// }
    /// assert_eq!(list.front(), Some(&5));
    /// ```
    #[inline]
    pub fn front_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut unsafe { node.as_mut() }.element)
    }

    /// Provides a reference to the back element,
    /// or `None` if the list is empty.
    ///
    /// This operation should compute in *O*(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use collections::linked_list::LinkedList;
    ///
    /// let mut list = LinkedList::new();
    /// assert_eq!(list.back(), None);
    ///
    /// list.push_back(1);
    /// assert_eq!(list.back(), Some(&1));
    /// ```
    #[inline]
    pub fn back(&self) -> Option<&T> {
        self.tail.as_ref().map(|node| &unsafe { node.as_ref() }.element)
    }

    /// Provides a mutable reference to the back element,
    /// or `None` if the list is empty.
    ///
    /// This operation should compute in *O*(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use collections::linked_list::LinkedList;
    ///
    /// let mut list = LinkedList::new();
    /// assert_eq!(list.back(), None);
    ///
    /// list.push_back(1);
    /// assert_eq!(list.back(), Some(&1));
    ///
    /// match list.back_mut() {
    ///     None => {},
    ///     Some(x) => *x = 5,
    /// }
    /// assert_eq!(list.back(), Some(&5));
    /// ```
    #[inline]
    pub fn back_mut(&mut self) -> Option<&mut T> {
        self.tail.as_mut().map(|node| &mut unsafe { node.as_mut() }.element)
    }

    /// Adds an element first in the list.
    ///
    /// This operation should compute in *O*(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use collections::linked_list::LinkedList;
    ///
    /// let mut list = LinkedList::new();
    ///
    /// list.push_front(2);
    /// assert_eq!(list.front().unwrap(), &2);
    ///
    /// list.push_front(1);
    /// assert_eq!(list.front().unwrap(), &1);
    /// ```
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

    /// Appends an element to the back of a list.
    ///
    /// This operation should compute in *O*(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use collections::linked_list::LinkedList;
    ///
    /// let mut list = LinkedList::new();
    /// list.push_back(1);
    /// list.push_back(3);
    /// assert_eq!(3, *list.back().unwrap());
    /// ```
    pub fn push_back(&mut self, element: T) {
        let node = Box::new(Node {
            element,
            next: None,
        });
        let node = Some(Box::leak(node).into());

        if let Some(tail) = self.tail {
            unsafe { (*tail.as_ptr()).next = node; }
        } else {
            self.head = node;
        }

        self.tail = node;
        self.len += 1;
    }

    /// Removes the first element and returns it,
    /// or `None` if the list is empty.
    ///
    /// This operation should compute in *O*(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use collections::linked_list::LinkedList;
    ///
    /// let mut list = LinkedList::new();
    /// assert_eq!(list.pop_front(), None);
    ///
    /// list.push_front(1);
    /// list.push_front(3);
    /// assert_eq!(list.pop_front(), Some(3));
    /// assert_eq!(list.pop_front(), Some(1));
    /// assert_eq!(list.pop_front(), None);
    /// ```
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

    /// Removes the last element from a list and returns it,
    /// or `None` if it is empty.
    ///
    /// This operation should compute in *O*(*n*) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use collections::linked_list::LinkedList;
    ///
    /// let mut list = LinkedList::new();
    /// assert_eq!(list.pop_back(), None);
    /// list.push_back(1);
    /// list.push_back(3);
    /// assert_eq!(list.pop_back(), Some(3));
    /// ```
    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.map(|node| {
            let node = unsafe { Box::from_raw(node.as_ptr()) };

            if let Some(penultimate) = {
                let mut last = self.head.unwrap();
                let mut penultimate = None;
                while let Some(l) = unsafe { last.as_ref() }.next {
                    penultimate = Some(last);
                    last = l;
                }
                penultimate
            } {
                unsafe { (*penultimate.as_ptr()).next = None; }
                self.tail = Some(penultimate);
            } else {
                self.head = None;
                self.tail = None;
            }

            self.len -= 1;
            node.element
        })
    }

    /// Removes all elements from the `LinkedList`.
    ///
    /// This operation should compute in *O*(*n*) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use collections::linked_list::LinkedList;
    ///
    /// let mut list = LinkedList::new();
    ///
    /// list.push_front(2);
    /// list.push_front(1);
    /// assert_eq!(list.len(), 2);
    /// assert_eq!(list.front(), Some(&1));
    ///
    /// list.clear();
    /// assert_eq!(list.len(), 0);
    /// assert_eq!(list.front(), None);
    /// ```
    #[inline]
    pub fn clear(&mut self) {
        *self = Self::new();
    }

    /// Insert an element at position `index` within the LinkedList.
    ///
    /// This operation should compute in *O*(*n*) time.
    ///
    /// # Panics
    ///
    /// Panics if `index` > `len`.
    ///
    /// # Examples
    ///
    /// ```
    /// use collections::linked_list::LinkedList;
    ///
    /// let mut list = LinkedList::new();
    ///
    /// list.insert(0, 4);
    /// assert_eq!(list.len(), 1);
    /// assert_eq!(list.front(), Some(&4));
    ///
    /// list.insert(1, 2);
    /// list.insert(1, 7);
    /// assert_eq!(list.len(), 3);
    /// assert_eq!(list.front(), Some(&4));
    /// assert_eq!(list.back(), Some(&2));
    /// ```
    ///
    /// ```should_panic
    /// use collections::linked_list::LinkedList;
    ///
    /// let mut list = LinkedList::new();
    ///
    /// list.insert(5, 6);
    /// ```
    pub fn insert(&mut self, index: usize, element: T) {
        let mut before = None;
        let mut after = self.head;
        for _ in 0..index {
            before = after;
            after = unsafe { after.unwrap().as_ref() }.next;
        }

        let node = Box::new(Node {
            element,
            next: after,
        });
        let node = Some(Box::leak(node).into());

        if let Some(b) = before {
            unsafe { (*b.as_ptr()).next = node; }
        } else {
            self.head = node;
        }
        if after.is_none() {
            self.tail = node;
        }

        self.len += 1;
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
    fn push_back() {
        let mut list: LinkedList<u32> = LinkedList::new();

        list.push_back(4);
        assert_eq!(list.len(), 1);
        assert_eq!(list.front(), Some(&4));
        assert_eq!(list.back(), Some(&4));

        list.push_back(6);
        assert_eq!(list.len(), 2);
        assert_eq!(list.front(), Some(&4));
        assert_eq!(list.back(), Some(&6));
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

    #[test]
    fn pop_back() {
        let mut list: LinkedList<u32> = LinkedList::new();

        assert_eq!(list.pop_back(), None);

        list.push_back(12);
        list.push_back(7);

        assert_eq!(list.pop_back(), Some(7));
        assert_eq!(list.pop_back(), Some(12));
        assert_eq!(list.pop_back(), None);

        mem::forget(list);
    }
}

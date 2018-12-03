use std::marker::PhantomData;
use std::mem;
use std::ptr::NonNull;

pub struct UnsafeLinkedList<T> {
    head: Option<NonNull<Node<T>>>,
}

pub struct Node<T> {
    next: Option<NonNull<Node<T>>>,
    prev_next: NonNull<Option<NonNull<Node<T>>>>,
    element: T,
}

pub struct NodeHandle<T> {
    ptr: NonNull<Node<T>>,
}

pub struct Iter<'a, T: 'a> {
    node: Option<NonNull<Node<T>>>,
    marker: PhantomData<&'a Node<T>>,
}

impl<T> UnsafeLinkedList<T> {
    pub fn new() -> Self {
        UnsafeLinkedList { head: None }
    }

    pub fn push_front(&mut self, elt: T) -> NodeHandle<T> {
        self.push_front_node(Box::new(Node::new(elt)))
    }

    pub fn push_front_node(&mut self, mut node: Box<Node<T>>) -> NodeHandle<T> {
        node.next = self.head;
        node.prev_next = NonNull::from(&mut self.head);
        if let Some(mut head) = self.head {
            unsafe {
                head.as_mut().prev_next = NonNull::from(&mut node.next);
            }
        }

        let head = NonNull::from(&mut *node);
        Box::into_raw(node);
        self.head = Some(head);
        NodeHandle { ptr: head }
    }

    pub unsafe fn remove_node(&mut self, node: NodeHandle<T>) -> Box<Node<T>> {
        let mut node = node.ptr;
        *node.as_mut().prev_next.as_mut() = node.as_ref().next;
        if let Some(mut next) = node.as_ref().next {
            next.as_mut().prev_next = node.as_ref().prev_next;
        }
        Box::from_raw(node.as_ptr())
    }

    pub unsafe fn get(&self, node: &NodeHandle<T>) -> &T {
        let ret = &node.ptr.as_ref().element;

        mem::transmute::<&T, &T>(ret)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            node: self.head,
            marker: PhantomData,
        }
    }
}

impl<T> Drop for UnsafeLinkedList<T> {
    fn drop(&mut self) {
        while let Some(head) = self.head {
            unsafe {
                self.head = Box::from_raw(head.as_ptr()).next;
            }
        }
    }
}

unsafe impl<T: Send> Send for UnsafeLinkedList<T> {}
unsafe impl<T: Sync> Sync for UnsafeLinkedList<T> {}

impl<T> Node<T> {
    pub fn new(element: T) -> Self {
        Node {
            next: None,
            prev_next: NonNull::dangling(),
            element,
        }
    }

    pub fn as_ref(&self) -> &T {
        &self.element
    }

    pub fn into_element(self) -> T {
        self.element
    }
}

unsafe impl<T: Send> Send for NodeHandle<T> {}
unsafe impl<T: Sync> Sync for NodeHandle<T> {}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        self.node.map(|node| {
            unsafe {
                let node = &*node.as_ptr();
                self.node = node.next;
                node.as_ref()
            }
        })
    }
}
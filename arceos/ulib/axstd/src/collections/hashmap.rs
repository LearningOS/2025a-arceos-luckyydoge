use alloc::vec;
use alloc::{
    boxed::Box,
    string::{String, ToString},
    vec::Vec,
};
use core::{hash::Hasher, slice::Iter as SliceIter};

use crate::collections::hash::Fnv1aHasher;

pub struct HashMap {
    bucket: Vec<Node>,
}

#[derive(Clone)]
struct Node {
    key: String,
    val: u32,
    hash: u64,
    next: Option<Box<Node>>,
}

impl Node {
    pub fn new() -> Self {
        Node {
            key: "".to_string(),
            val: 0,
            hash: 0,
            next: None,
        }
    }
}

pub struct Iter<'a> {
    bucket_iter: SliceIter<'a, Node>,
    current_node: Option<&'a Node>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = (&'a str, &'a u32);
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.current_node {
            self.current_node = node.next.as_deref();
            return Some((&node.key, &node.val));
        }
        while let Some(head) = self.bucket_iter.next() {
            if let Some(node) = head.next.as_deref() {
                self.current_node = node.next.as_deref();
                return Some((&node.key, &node.val));
            }
        }
        None
    }
}

impl HashMap {
    pub fn new() -> Self {
        HashMap {
            bucket: vec![Node::new(); 1001],
        }
    }

    pub fn iter(&self) -> Iter {
        Iter {
            bucket_iter: self.bucket.iter(),
            current_node: None,
        }
    }

    pub fn insert(&mut self, k: String, v: u32) -> () {
        let mut hasher = Fnv1aHasher::new();
        hasher.write(k.as_bytes());
        let hash = hasher.finish();
        let index = hash % 1001;
        let mut head = &mut self.bucket[index as usize];
        {
            while let Some(node) = head.next.as_mut().map(|box_node| &mut **box_node) {
                if node.hash == hash {
                    node.val = v;
                    return;
                }
                head = node;
            }
        }
        let mut head = &mut self.bucket[index as usize];
        let new_node = Node {
            key: k,
            val: v,
            hash,
            next: head.next.take(),
        };
        head.next = Some(Box::new(new_node));
    }
}

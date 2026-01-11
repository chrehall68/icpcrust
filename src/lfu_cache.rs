use std::{cell::RefCell, collections::HashMap, hash::Hash, rc::Rc};

type Node<T> = Rc<RefCell<LinkedListNode<T>>>;
struct LinkedListNode<T> {
    next: Option<Node<T>>,
    prev: Option<Node<T>>,
    // unless it is the sentinel head, it will have Some
    value: Option<T>,
}
impl<T> LinkedListNode<T> {
    pub fn new(val: T) -> Self {
        LinkedListNode {
            next: None,
            prev: None,
            value: Some(val),
        }
    }
}
impl<T: Eq> PartialEq for LinkedListNode<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
// only distinct values
struct LinkedList<T> {
    // head is a sentinel value
    head: Node<T>,
    length: usize,
}
impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: Rc::new(RefCell::new(LinkedListNode {
                next: None,
                prev: None,
                value: None,
            })),
            length: 0,
        }
    }
    pub fn push_front_val(&mut self, val: T) -> Node<T> {
        let node = Rc::new(RefCell::new(LinkedListNode::new(val)));
        self.push_front_node(node)
    }
    pub fn push_front_node(&mut self, node: Node<T>) -> Node<T> {
        let prev = self.head.clone();
        self.insert_after(prev, node)
    }
    pub fn insert_after(&mut self, a: Node<T>, b: Node<T>) -> Node<T> {
        // returns b
        let next = a.borrow().next.clone();
        // attach to a
        a.borrow_mut().next = Some(b.clone());
        b.borrow_mut().prev = Some(a);
        // attach to next
        b.borrow_mut().next = next.clone();
        if let Some(next) = next.clone() {
            next.borrow_mut().prev = Some(b.clone());
        }
        self.length += 1;
        b
    }
    pub fn remove_node(&mut self, node: Node<T>) {
        // doesn't immediately drop the value, but removes other references to it
        // doesn't remove connections
        if let Some(next) = node.borrow().next.clone() {
            next.borrow_mut().prev = node.borrow().prev.clone();
        }
        if let Some(prev) = node.borrow().prev.clone() {
            prev.borrow_mut().next = node.borrow().next.clone();
        }
        self.length -= 1;
    }
    pub fn len(&self) -> usize {
        self.length
    }
}
#[derive(PartialEq, Eq, Clone, Hash)]
struct LFUCacheInfo<K, V> {
    use_count: usize,
    k: K,
    v: V,
}

pub struct LFUCache {
    ll: LinkedList<LFUCacheInfo<i32, i32>>,
    capacity: usize,
    // maps from use_count: last node with that use count
    lasts: HashMap<usize, Node<LFUCacheInfo<i32, i32>>>,
    // maps from k: node
    nodes: HashMap<i32, Node<LFUCacheInfo<i32, i32>>>,
}
// idea is just maintain a linkedlist ordered by
// (use_count, recently_used)
impl LFUCache {
    pub fn new(capacity: i32) -> Self {
        LFUCache {
            ll: LinkedList::new(),
            capacity: capacity as usize,
            lasts: HashMap::new(),
            nodes: HashMap::new(),
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(node) = self.nodes.get(&key).cloned() {
            let result = node.borrow().value.as_ref().unwrap().v;
            self.use_node(node);
            result
        } else {
            -1
        }
    }

    fn update_lasts(&mut self, to_remove: Node<LFUCacheInfo<i32, i32>>) {
        let use_count = to_remove.borrow().value.clone().unwrap().use_count;
        if self.lasts.get(&use_count).cloned().unwrap() == to_remove {
            // then either we update it or remove it
            let prev = to_remove.borrow().prev.clone().unwrap();
            let prev_info = prev.borrow().value.clone();
            if let Some(info) = prev_info
                && info.use_count == use_count
            {
                // then prev is our new last
                self.lasts.insert(use_count, prev);
            } else {
                // either this was the first thing or the prev has a different use count
                // either way, need to get rid of this entry in self.lasts
                self.lasts.remove(&use_count);
            }
        }
    }
    fn use_node(&mut self, node: Node<LFUCacheInfo<i32, i32>>) {
        // handle when this was the last of something
        // since it no longer has this use count, we need to set our new lasts
        self.update_lasts(node.clone());
        let use_count = node.borrow().value.clone().unwrap().use_count;
        // then now move it to either after the last of the next
        // or after the last of the cur (if no next exists)
        // or keep it here (if it's already the last of the cur and no next exists)
        let last = if let Some(next_last) = self.lasts.get(&(use_count + 1)).cloned() {
            Some(next_last)
        } else if let Some(cur_last) = self.lasts.get(&use_count).cloned() {
            Some(cur_last)
        } else {
            None
        };
        if let Some(last) = last {
            self.ll.remove_node(node.clone());
            self.ll.insert_after(last, node.clone());
        }
        // and update node's use count
        self.lasts.insert(use_count + 1, node.clone());
        node.borrow_mut().value.as_mut().unwrap().use_count = use_count + 1;
    }

    pub fn put(&mut self, key: i32, value: i32) {
        // two cases:
        // exists or doesn't
        if let None = self.nodes.get(&key)
            && self.ll.len() == self.capacity
        {
            // will need to evict the least frequently used thing
            // and in cases of ties, the least recently used
            // which is always just the front of the linkedlist
            let front = self.ll.head.borrow().next.clone().unwrap();
            // but first update our lasts
            self.update_lasts(front.clone());
            self.ll.remove_node(front.clone());
            self.nodes.remove(&front.borrow().value.clone().unwrap().k);
        }
        // now we definitely have space, no matter what we're doing
        // first, remove the node if it already exists
        let node;
        match self.nodes.get(&key).cloned() {
            None => {
                node = self.ll.push_front_val(LFUCacheInfo {
                    use_count: 0,
                    k: key,
                    v: value,
                });
                self.nodes.insert(key, node.clone());
                self.lasts.insert(0, node.clone());
            }
            Some(existing) => {
                node = existing;
                node.borrow_mut().value.as_mut().unwrap().v = value;
            }
        };
        self.use_node(node);
    }
}

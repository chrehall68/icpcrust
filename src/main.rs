mod lfu_cache;
use lfu_cache::LFUCache;
pub fn main() {
    let mut obj = LFUCache::new(3);
    obj.put(1, 3);
    println!("got {}", obj.get(1));
    obj.put(1, 4);
    println!("got {}", obj.get(1));
    obj.put(2, 5);
    println!("got {}", obj.get(2));
    obj.put(3, 2);
    println!("got {}", obj.get(3));
    obj.put(4, 17);
    println!("got {}", obj.get(4));
    println!("got {}", obj.get(2));
}

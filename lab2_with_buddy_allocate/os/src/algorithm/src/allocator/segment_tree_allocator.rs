
use super::Allocator;
use alloc::{vec, vec::Vec};
use bit_field::BitArray;


pub struct SegmentTreeAllocator {
    segment: Vec<u8>,
    capacity:usize,
    leaf_count:usize,
}

impl Allocator for SegmentTreeAllocator {
    fn new(capacity: usize) -> Self {
        let leaf_count = capacity.next_power_of_two();
        let mut tree =vec![0u8;(2*leaf_count/8)];
        for i in capacity..leaf_count{
            tree.set_bit(leaf_count+i,false);
        }
        for i in (1..leaf_count).rev(){
            let v = tree.get_bit(i * 2) && tree.get_bit(i * 2 + 1);
            tree.set_bit(i,v);
        }
        Self{segment:tree,capacity:capacity,leaf_count:leaf_count}
    }

    fn alloc(&mut self) -> Option<usize> {
        if self.segment.get_bit(1){
            None
        }else{
            let mut node =1;
            while node<self.leaf_count{
                if !self.segment.get_bit(node*2) {
                    node *= 2;
                }else if !self.segment.get_bit(node*2+1){
                    node = node*2+1;
                }else{
                    panic!("tree damaged");
                }
            }
            self.update_node(node, true);
            Some(node-self.leaf_count)
        }
    }

    fn dealloc(&mut self, index: usize) {
        let node = index + self.leaf_count;
        assert!(self.segment.get_bit(node));
        self.update_node(node, false);
    }
}

impl SegmentTreeAllocator{
    fn update_node(&mut self, mut index: usize, value: bool) {
        self.segment.set_bit(index,value);
        while index>1{
            index/=2;
            let v = self.segment.get_bit(index * 2) && self.segment.get_bit(index * 2 + 1);
            self.segment.set_bit(index,v);
        }
    }
}
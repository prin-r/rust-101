use bit_vec::BitVec;
use std::collections::HashMap;
use tiny_keccak::{Hasher, Sha3};

#[derive(Clone)]
pub enum Node {
    Leaf(Box<[u8]>),
    Branch([u8; 32]),
}

pub struct Tree {
    root: [u8; 32],
    size: u32,
    map: HashMap<[u8; 32], (Node, Node)>,
}

fn _extract_key(node: &Node) -> [u8; 32] {
    match node {
        Node::Branch(h) => *h,
        _ => [0u8; 32],
    }
}

pub fn hash(val: &[u8]) -> [u8; 32] {
    let mut sha3 = Sha3::v256();
    let mut output = [0u8; 32];
    sha3.update(val);
    sha3.finalize(&mut output);
    output
}

pub fn to_bitvec(val: &[u8]) -> BitVec {
    BitVec::from_bytes(val)
}

impl Tree {
    pub fn new() -> Tree {
        let mut tree = Tree {
            root: [0u8; 32],
            size: 0u32,
            map: HashMap::new(),
        };
        tree.map.insert(
            [0u8; 32],
            (
                Node::Leaf(Box::new([0u8; 32])),
                Node::Leaf(Box::new([0u8; 32])),
            ),
        );
        tree
    }

    pub fn size(self: &Self) -> u32 {
        self.size
    }

    pub fn root(self: &Self) -> [u8; 32] {
        self.root
    }

    fn _get_with_default(self: &Self, _key: &[u8; 32]) -> (Node, Node) {
        let key;
        if self.map.contains_key(_key) {
            key = _key;
        } else {
            key = &[0u8; 32]
        }
        match &self.map[key] {
            (Node::Branch(l), Node::Branch(r)) => (Node::Branch(*l), Node::Branch(*r)),
            (Node::Leaf(l), Node::Branch(r)) => (Node::Leaf((&**l).into()), Node::Branch(*r)),
            (Node::Branch(l), Node::Leaf(r)) => (Node::Branch(*l), Node::Leaf((&**r).into())),
            (Node::Leaf(l), Node::Leaf(r)) => {
                (Node::Leaf((&**l).into()), Node::Leaf((&**r).into()))
            }
        }
    }

    fn _insert(
        self: &mut Self,
        current_hash: &[u8; 32],
        _key: &BitVec,
        _val: &[u8],
        bit_index: &mut usize,
    ) -> Node {
        if *bit_index >= 255 {
            return Node::Leaf(_val.into());
        }

        let (mut left, mut right) = self._get_with_default(current_hash);

        match _key[*bit_index] {
            false => {
                *bit_index += 1;
                left = self._insert(&_extract_key(&left), _key, _val, bit_index);
            }
            true => {
                *bit_index += 1;
                right = self._insert(&_extract_key(&right), _key, _val, bit_index);
            }
        }

        let new_hash;
        let clone_of_left_right;
        match (left, right) {
            (Node::Branch(l), Node::Branch(r)) => {
                new_hash = hash(&[l, r].concat());
                clone_of_left_right = (Node::Branch(l), Node::Branch(r));
            }
            (Node::Leaf(l), Node::Branch(r)) => {
                new_hash = hash(&[&*l, &r].concat());
                clone_of_left_right = (Node::Leaf(l), Node::Branch(r));
            }
            (Node::Branch(l), Node::Leaf(r)) => {
                new_hash = hash(&[&l, &*r].concat());
                clone_of_left_right = (Node::Branch(l), Node::Leaf(r));
            }
            (Node::Leaf(l), Node::Leaf(r)) => {
                new_hash = hash(&[&*l, &*r].concat());
                clone_of_left_right = (Node::Leaf(l), Node::Leaf(r));
            }
        }
        self.map.insert(new_hash, clone_of_left_right);
        Node::Branch(new_hash)
    }

    pub fn contain(self: &Self) -> bool {
        false
    }

    pub fn insert(self: &mut Self, val: &[u8]) -> bool {
        let mut key_bits = BitVec::from_bytes(&hash(val));
        let root = self.root;
        let new_root = self._insert(&root, &mut key_bits, val, &mut 0);
        match new_root {
            Node::Branch(new_root_hash) => {
                self.root = new_root_hash;
                self.size = self.size + 1;
                return true;
            }
            _ => false,
        }
    }
}

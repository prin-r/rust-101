pub mod smt;

#[cfg(test)]
mod tests {
    use crate::smt;
    use smt::Tree;

    #[test]
    fn test_creation() {
        let t = Tree::new();
        assert_eq!(t.root(), [0u8; 32]);
    }

    #[test]
    fn test_hashing() {
        assert_eq!(
            smt::hash(b"test_hashing"),
            [
                83, 245, 113, 136, 13, 233, 74, 228, 56, 211, 72, 122, 8, 136, 123, 9, 50, 136,
                201, 199, 47, 196, 128, 249, 118, 195, 210, 171, 212, 92, 3, 185
            ]
        );

        let v1 = b"some message 1";
        let v2 = b"some message 2";
        let bv1 = smt::to_bitvec(&smt::hash(v1));
        let bv2 = smt::to_bitvec(&smt::hash(v2));
        let mut r1 = v1.to_vec();
        let mut r2 = v2.to_vec();
        let mut root = Default::default();
        let concat = |arr: std::vec::Vec<u8>, flag| match flag {
            false => [arr, (&[0u8; 32]).to_vec()].concat(),
            true => [(&[0u8; 32]).to_vec(), arr].concat(),
        };
        for i in (0..255).rev() {
            match (bv1[i], bv2[i], i) {
                (_, _, 0) => {
                    root = (&smt::hash(&concat(root, false))).to_vec();
                }
                (_, _, 1) => {
                    root = (&smt::hash(&concat(root, false))).to_vec();
                }
                (_, _, 2) => {
                    root = (&smt::hash(&[&r1[..], &r2[..]].concat())).to_vec();
                }
                (l, r, _) => {
                    r1 = (&smt::hash(&concat(r1, l))).to_vec();
                    r2 = (&smt::hash(&concat(r2, r))).to_vec();
                }
            }
        }

        assert_eq!(
            root,
            [
                221, 124, 131, 18, 165, 4, 117, 72, 97, 9, 22, 94, 242, 153, 180, 25, 154, 145, 8,
                233, 220, 145, 255, 1, 30, 113, 176, 140, 200, 202, 90, 62
            ]
        );
    }

    #[test]
    fn test_insert() {
        let mut t = Tree::new();
        assert_eq!(t.root(), [0u8; 32]);

        t.insert(b"some message 1");
        let new_root1 = t.root();
        assert_ne!(new_root1, [0u8; 32]);

        t.insert(b"some message 1");
        assert_eq!(new_root1, t.root());

        t.insert(b"some message 2");
        let new_root2 = t.root();
        assert_ne!(new_root2, new_root1);
        assert_eq!(
            new_root2,
            [
                221, 124, 131, 18, 165, 4, 117, 72, 97, 9, 22, 94, 242, 153, 180, 25, 154, 145, 8,
                233, 220, 145, 255, 1, 30, 113, 176, 140, 200, 202, 90, 62
            ]
        );

        t.insert(b"some message 1");
        assert_eq!(t.root(), new_root2);

        t.insert(b"some message 2");
        assert_eq!(t.root(), new_root2);

        t.insert(b"some message 3");
        let new_root3 = t.root();
        assert_ne!(new_root3, new_root2);
        assert_ne!(new_root3, new_root1);

        t.insert(b"some message 1");
        assert_eq!(t.root(), new_root3);

        t.insert(b"some message 2");
        assert_eq!(t.root(), new_root3);

        t.insert(b"some message 3");
        assert_eq!(t.root(), new_root3);
    }
}

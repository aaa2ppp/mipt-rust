#![forbid(unsafe_code)]
use bst::NodeRef;

fn make_tree<K: Ord + Clone>(keys: &[K]) -> NodeRef<K, ()> {
    let mut root = NodeRef::Nil;
    for k in keys {
        let k = k.clone();
        (root, _) = root.insert(k, ())
    }
    root
}

#[test]
fn node_insert() {
    let t = make_tree(&[4, 6, 2, 5, 1, 7, 3]);
    assert_eq!(t.lnr(Vec::new()), &[&1, &2, &3, &4, &5, &6, &7]);
    assert_eq!(t.nlr(Vec::new()), &[&4, &2, &1, &3, &6, &5, &7]);
    assert_eq!(t.size(), 7);
    assert_eq!(t.height(), 3);

    let t = make_tree(&[4, 2, 6, 1, 3, 5, 7]);
    assert_eq!(t.lnr(Vec::new()), &[&1, &2, &3, &4, &5, &6, &7]);
    assert_eq!(t.nlr(Vec::new()), &[&4, &2, &1, &3, &6, &5, &7]);
    assert_eq!(t.size(), 7);
    assert_eq!(t.height(), 3);

    let t = make_tree(&[1, 2, 3, 4, 5, 6, 7]);
    assert_eq!(t.lnr(Vec::new()), &[&1, &2, &3, &4, &5, &6, &7]);
    assert_eq!(t.nlr(Vec::new()), &[&4, &2, &1, &3, &6, &5, &7]);
    assert_eq!(t.size(), 7);
    assert_eq!(t.height(), 3);

    let t = make_tree(&[7, 6, 5, 4, 3, 2, 1]);
    assert_eq!(t.lnr(Vec::new()), &[&1, &2, &3, &4, &5, &6, &7]);
    assert_eq!(t.nlr(Vec::new()), &[&4, &2, &1, &3, &6, &5, &7]);
    assert_eq!(t.size(), 7);
    assert_eq!(t.height(), 3);
}

#[test]
fn node_remove() {
    let mut t = make_tree(&[2, 1, 3]);
    (t, _) = t.remove(&1);
    assert_eq!(t.lnr(Vec::new()), &[&2, &3]);
    assert_eq!(t.nlr(Vec::new()), &[&2, &3]);
    assert_eq!(t.size(), 2);
    assert_eq!(t.height(), 2);

    let mut t = make_tree(&[2, 1, 3]);
    (t, _) = t.remove(&3);
    assert_eq!(t.lnr(Vec::new()), &[&1, &2]);
    assert_eq!(t.nlr(Vec::new()), &[&2, &1]);
    assert_eq!(t.size(), 2);
    assert_eq!(t.height(), 2);

    let mut t = make_tree(&[2, 1, 3]);
    (t, _) = t.remove(&2);
    assert_eq!(t.lnr(Vec::new()), &[&1, &3]);
    assert_eq!(t.nlr(Vec::new()), &[&3, &1]);
    assert_eq!(t.size(), 2);
    assert_eq!(t.height(), 2);

    let mut t = make_tree(&[4, 2, 6, 1, 3, 5, 7]);
    (t, _) = t.remove(&5);
    assert_eq!(t.lnr(Vec::new()), &[&1, &2, &3, &4, &6, &7]);
    assert_eq!(t.nlr(Vec::new()), &[&4, &2, &1, &3, &6, &7]);
    assert_eq!(t.size(), 6);
    assert_eq!(t.height(), 3);
}

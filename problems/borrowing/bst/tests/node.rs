#![forbid(unsafe_code)]
use bst::Node;

fn make_tree<K: Ord + Clone>(keys: &[K]) -> Option<Box<Node<K, ()>>> {
    let mut root = None;
    for k in keys {
        let k = k.clone();
        (root, _) = Node::insert(root, k, ())
    }
    root
}

#[test]
fn node_insert() {
    let t = make_tree(&[4, 6, 2, 5, 1, 7, 3]);
    assert_eq!(Node::lnr(t.as_ref(), Vec::new()), &[&1, &2, &3, &4, &5, &6, &7]);
    assert_eq!(Node::nlr(t.as_ref(), Vec::new()), &[&4, &2, &1, &3, &6, &5, &7]);
    assert_eq!(Node::size(t.as_ref()), 7);
    assert_eq!(Node::height(t.as_ref()), 3);

    let t = make_tree(&[4, 2, 6, 1, 3, 5, 7]);
    assert_eq!(Node::lnr(t.as_ref(), Vec::new()), &[&1, &2, &3, &4, &5, &6, &7]);
    assert_eq!(Node::nlr(t.as_ref(), Vec::new()), &[&4, &2, &1, &3, &6, &5, &7]);
    assert_eq!(Node::size(t.as_ref()), 7);
    assert_eq!(Node::height(t.as_ref()), 3);

    let t = make_tree(&[1, 2, 3, 4, 5, 6, 7]);
    assert_eq!(Node::lnr(t.as_ref(), Vec::new()), &[&1, &2, &3, &4, &5, &6, &7]);
    assert_eq!(Node::nlr(t.as_ref(), Vec::new()), &[&4, &2, &1, &3, &6, &5, &7]);
    assert_eq!(Node::size(t.as_ref()), 7);
    assert_eq!(Node::height(t.as_ref()), 3);

    let t = make_tree(&[7, 6, 5, 4, 3, 2, 1]);
    assert_eq!(Node::lnr(t.as_ref(), Vec::new()), &[&1, &2, &3, &4, &5, &6, &7]);
    assert_eq!(Node::nlr(t.as_ref(), Vec::new()), &[&4, &2, &1, &3, &6, &5, &7]);
    assert_eq!(Node::size(t.as_ref()), 7);
    assert_eq!(Node::height(t.as_ref()), 3);
}

#[test]
fn node_remove() {
    let mut t = make_tree(&[2, 1, 3]);
    (t, _) = Node::remove(t, &1);
    assert_eq!(Node::lnr(t.as_ref(), Vec::new()), &[&2, &3]);
    assert_eq!(Node::nlr(t.as_ref(), Vec::new()), &[&2, &3]);
    assert_eq!(Node::size(t.as_ref()), 2);
    assert_eq!(Node::height(t.as_ref()), 2);

    let mut t = make_tree(&[2, 1, 3]);
    (t, _) = Node::remove(t, &3);
    assert_eq!(Node::lnr(t.as_ref(), Vec::new()), &[&1, &2]);
    assert_eq!(Node::nlr(t.as_ref(), Vec::new()), &[&2, &1]);
    assert_eq!(Node::size(t.as_ref()), 2);
    assert_eq!(Node::height(t.as_ref()), 2);

    let mut t = make_tree(&[2, 1, 3]);
    (t, _) = Node::remove(t, &2);
    assert_eq!(Node::lnr(t.as_ref(), Vec::new()), &[&1, &3]);
    assert_eq!(Node::nlr(t.as_ref(), Vec::new()), &[&3, &1]);
    assert_eq!(Node::size(t.as_ref()), 2);
    assert_eq!(Node::height(t.as_ref()), 2);

    let mut t = make_tree(&[4, 2, 6, 1, 3, 5, 7]);
    (t, _) = Node::remove(t, &5);
    assert_eq!(Node::lnr(t.as_ref(), Vec::new()), &[&1, &2, &3, &4, &6, &7]);
    assert_eq!(Node::nlr(t.as_ref(), Vec::new()), &[&4, &2, &1, &3, &6, &7]);
    assert_eq!(Node::size(t.as_ref()), 6);
    assert_eq!(Node::height(t.as_ref()), 3);

}
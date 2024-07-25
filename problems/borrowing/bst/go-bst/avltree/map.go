package avltree

import (
	"cmp"
)

type Map[K cmp.Ordered, V any] struct {
	root *Node[K, V]
}

func (t *Map[K, V]) Len() int {
	return t.root.Size()
}

func (t *Map[K, V]) Empty() bool {
	return t.Len() == 0
}

func (t *Map[K, V]) Get(key K) (V, bool) {
	node := t.root.Get(key)
	return node.Val(), node != nil
}

func (t *Map[K, V]) GetKeyValue(key K) (K, V, bool) {
	node := t.root.Get(key)
	return node.Key(), node.Val(), node != nil
}

func (t *Map[K, V]) NthKeyValue(i int) (K, V, bool) {
	node := t.root.GetNth(i)
	return node.Key(), node.Val(), node != nil
}

func (t *Map[K, V]) ContainsKey(key K) bool {
	return t.root.Get(key) != nil
}

func (t *Map[K, V]) Insert(key K, val V) (V, bool) {
	new_root, old_val, ok := t.root.Insert(key, val)
	t.root = new_root
	return old_val, ok
}

func (t *Map[K, V]) Remove(key K) (V, bool) {
	new_root, old_node := t.root.Remove(key)
	t.root = new_root
	return old_node.Val(), old_node != nil
}

func (t *Map[K, V]) RemoveEntry(key K) (K, V, bool) {
	new_root, old_node := t.root.Remove(key)
	t.root = new_root
	return old_node.Key(), old_node.Val(), old_node != nil
}

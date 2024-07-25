package avltree

import (
	"cmp"
)

type Node[K cmp.Ordered, V any] struct {
	key   K
	val   V
	left  *Node[K, V]
	right *Node[K, V]
	size  int
	hght  int
}

func (n *Node[K, V]) Key() K {
	if n == nil {
		return *new(K)
	}
	return n.key
}

func (n *Node[K, V]) Val() V {
	if n == nil {
		return *new(V)
	}
	return n.val
}

func (n *Node[K, V]) Size() int {
	if n == nil {
		return 0
	}
	return n.size
}

func (n *Node[K, V]) height() int {
	if n == nil {
		return 0
	}
	return n.hght
}

func (n *Node[K, V]) Get(key K) *Node[K, V] {
	if n == nil {
		return nil
	}
	if key < n.key {
		return n.left.Get(key)
	}
	if key > n.key {
		return n.right.Get(key)
	}
	return n
}

func (n *Node[K, V]) GetNth(i int) *Node[K, V] {
	if n == nil {
		return nil
	}
	left_size := n.left.Size()
	if i < left_size {
		return n.left.GetNth(i)
	}
	if i > left_size {
		return n.right.GetNth(i - left_size - 1)
	}
	return n
}

func (n *Node[K, V]) Insert(key K, val V) (*Node[K, V], V, bool) {
	if n == nil {
		return &Node[K, V]{
			key:  key,
			val:  val,
			size: 1,
			hght: 1,
		}, *new(V), false
	}

	if key < n.key {
		new_left, old_val, ok := n.left.Insert(key, val)
		n.left = new_left
		return n.repair(), old_val, ok
	}

	if key > n.key {
		new_right, old_val, ok := n.right.Insert(key, val)
		n.right = new_right
		return n.repair(), old_val, ok
	}

	old_val := n.val
	n.val = val
	return n, old_val, true
}

func (n *Node[K, V]) Remove(key K) (*Node[K, V], *Node[K, V]) {
	if n == nil {
		return nil, nil
	}

	if key < n.key {
		new_left, old_node := n.left.Remove(key)
		n.left = new_left
		return n.repair(), old_node
	}

	if key > n.key {
		new_right, old_node := n.right.Remove(key)
		n.right = new_right
		return n.repair(), old_node
	}

	left, right := n.untie()

	if left == nil {
		return right, n
	}

	if right == nil {
		return left, n
	}

	new_right, min_node := right.removeMin()
	min_node.right = new_right
	min_node.left = left
	return min_node.repair(), n
}

func (n *Node[K, V]) removeMin() (*Node[K, V], *Node[K, V]) {
	if n.left == nil {
		_, right := n.untie()
		return right, n
	}

	new_left, min_node := n.left.removeMin()
	n.left = new_left
	return n.repair(), min_node
}

func (n *Node[K, V]) untie() (*Node[K, V], *Node[K, V]) {
	left, right := n.left, n.right
	n.left, n.right = nil, nil
	n.update()
	return left, right
}

func (n *Node[K, V]) update() {
	n.size = n.left.Size() + n.right.Size() + 1
	n.hght = max(n.left.height(), n.right.height()) + 1
}

func (n *Node[K, V]) repair() *Node[K, V] {
	d := n.left.height() - n.right.height()
	if d < -1 {
		return n.leftRotate()
	}
	if d > 1 {
		return n.rightRotate()
	}
	n.update()
	return n
}

func (n *Node[K, V]) leftRotate() *Node[K, V] {
	al := n
	bt := al.right

	if bt.right.height()-bt.left.height() > 0 {
		al.right = bt.left
		al.update()
		bt.left = al
		bt.update()
		return bt
	}

	ga := bt.left
	al.right = ga.left
	al.update()
	bt.left = ga.right
	bt.update()
	ga.left = al
	ga.right = bt
	ga.update()
	return ga
}

func (n *Node[K, V]) rightRotate() *Node[K, V] {
	al := n
	bt := al.left

	if bt.left.height()-bt.right.height() > 0 {
		al.left = bt.right
		al.update()
		bt.right = al
		bt.update()
		return bt
	}

	ga := bt.right
	al.left = ga.right
	al.update()
	bt.right = ga.left
	bt.update()
	ga.right = al
	ga.left = bt
	ga.update()
	return ga
}

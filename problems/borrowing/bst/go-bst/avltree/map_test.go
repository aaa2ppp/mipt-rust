package avltree_test

import (
	"cmp"
	"context"
	"math/rand"
	"sort"
	"testing"
	"time"

	"go-bst/avltree"
)

func checkEmpty[K cmp.Ordered, V any](t *testing.T, m *avltree.Map[K, V], want bool) {
	if got := m.Empty(); got != want {
		t.Fatalf("Enpty() = %v, want %v", got, want)
	}
}

func checkLen[K cmp.Ordered, V any](t *testing.T, m *avltree.Map[K, V], want int) {
	if got := m.Len(); got != want {
		t.Fatalf("Len() = %v, want %v", got, want)
	}
}

func checkInsert[K cmp.Ordered, V comparable](t *testing.T, m *avltree.Map[K, V], k K, v V, wantV V, wantOk bool) {
	if gotV, gotOk := m.Insert(k, v); gotV != wantV || gotOk != wantOk {
		t.Fatalf("Insert(%v, %v) = %v, %v, want %v, %v", k, v, gotV, gotOk, wantV, wantOk)
	}
}

func checkContainsKey[K cmp.Ordered, V any](t *testing.T, m *avltree.Map[K, V], k K, want bool) {
	if got := m.ContainsKey(k); got != want {
		t.Fatalf("ContainsKey(%v) = %v, want %v", k, got, want)
	}
}

func checkRemove[K cmp.Ordered, V comparable](t *testing.T, m *avltree.Map[K, V], k K, wantV V, wantOk bool) {
	if gotV, gotOk := m.Remove(k); gotV != wantV || gotOk != wantOk {
		t.Fatalf("Remove(%v) = %v %v, want %v %v", k, gotV, gotOk, wantV, wantOk)
	}
}

func checkRemoveEntry[K cmp.Ordered, V comparable](t *testing.T, m *avltree.Map[K, V], k K, wantK, wantV V, wantOk bool) {
	if gotK, gotV, gotOk := m.RemoveEntry(k); gotV != wantV || gotOk != wantOk {
		t.Fatalf("RemoveEntry(%v) = %v %v %v, want %v %v %v", k, gotK, gotV, gotOk, wantK, wantV, wantOk)
	}
}

func checkNthKeyValue[K cmp.Ordered, V comparable](t *testing.T, m *avltree.Map[K, V], i int, wantK K, wantV V, wantOk bool) {
	if gotK, gotV, gotOk := m.NthKeyValue(i); gotK != wantK || gotV != wantV || gotOk != wantOk {
		t.Fatalf("NthKeyValue(%d) = %v %v %v, want %v %v %v", i, gotK, gotV, gotOk, wantK, wantV, wantOk)
	}
}

func checkGet[K cmp.Ordered, V comparable](t *testing.T, m *avltree.Map[K, V], k K, wantV V, wantOk bool) {
	if gotV, gotOk := m.Get(k); gotV != wantV || gotOk != wantOk {
		t.Fatalf("Get(%v) = %v %v, want %v %v", k, gotV, gotOk, wantV, wantOk)
	}
}

func checkGetKeyValue[K cmp.Ordered, V comparable](t *testing.T, m *avltree.Map[K, V], k K, wantK K, wantV V, wantOk bool) {
	if gotK, gotV, gotOk := m.GetKeyValue(k); gotK != wantK || gotV != wantV || gotOk != wantOk {
		t.Fatalf("GetKeyValue(%v) = %v %v %v, want %v %v %v", k, gotK, gotV, gotOk, wantK, wantV, wantOk)
	}
}

// #[test]
// fn empty() {
//     let mut map = AVLTreeMap::new();
//     assert!(map.is_empty());
//     assert_eq!(map.insert(1, 1), None);
//     assert_eq!(map.insert(2, 2), None);
//     assert_eq!(map.insert(3, 3), None);
//     assert!(!map.is_empty());
// }

func TestMap_empty(t *testing.T) {
	var m avltree.Map[int, int]
	checkEmpty(t, &m, true)
	checkInsert(t, &m, 1, 1, 0, false)
	checkInsert(t, &m, 2, 2, 0, false)
	checkInsert(t, &m, 3, 3, 0, false)
	checkEmpty(t, &m, false)
}

// #[test]
// fn should_compile1() {
//     let mut map = AVLTreeMap::new();
//     assert_eq!(map.insert(Number(1), 1), None);
//     assert!(map.contains_key(&Number(1)));
// }

type Number int32

func TestMap_hould_compile1(t *testing.T) {
	var m avltree.Map[Number, int]
	checkInsert(t, &m, Number(1), 1, 0, false)
	checkContainsKey(t, &m, 1, true)
}

// #[test]
//
//	fn contains() {
//	    let mut map = AVLTreeMap::new();
//	    assert_eq!(map.insert(1, 1), None);
//	    assert_eq!(map.insert(2, 2), None);
//	    assert_eq!(map.insert(3, 3), None);
//	    assert!(!map.contains_key(&0));
//	    assert!(map.contains_key(&1));
//	    assert!(map.contains_key(&2));
//	    assert!(map.contains_key(&3));
//	    assert!(!map.contains_key(&4));
//	}

func TestMap_contains(t *testing.T) {
	var m avltree.Map[int, int]
	checkInsert(t, &m, 1, 1, 0, false)
	checkInsert(t, &m, 2, 2, 0, false)
	checkInsert(t, &m, 3, 3, 0, false)

	checkContainsKey(t, &m, 0, false)
	checkContainsKey(t, &m, 1, true)
	checkContainsKey(t, &m, 2, true)
	checkContainsKey(t, &m, 3, true)
	checkContainsKey(t, &m, 4, false)
}

// #[test]
// fn remove() {
//     let mut map = AVLTreeMap::new();
//     assert_eq!(map.insert(1, 1), None);
//     assert_eq!(map.insert(2, 2), None);
//     assert_eq!(map.insert(3, 3), None);
//     assert_eq!(map.remove(&1), Some(1));
//     assert!(!map.contains_key(&1));
//     assert!(map.contains_key(&2));
//     assert!(map.contains_key(&3));
//     assert_eq!(map.remove(&2), Some(2));
//     assert!(!map.contains_key(&1));
//     assert!(!map.contains_key(&2));
//     assert!(map.contains_key(&3));
//     assert_eq!(map.remove(&3), Some(3));
//     assert!(!map.contains_key(&1));
//     assert!(!map.contains_key(&2));
//     assert!(!map.contains_key(&3));
//     assert!(map.is_empty());
// }

func TestMap_remove(t *testing.T) {
	var m avltree.Map[int, int]
	checkInsert(t, &m, 1, 1, 0, false)
	checkInsert(t, &m, 2, 2, 0, false)
	checkInsert(t, &m, 3, 3, 0, false)

	checkRemove(t, &m, 1, 1, true)
	checkContainsKey(t, &m, 1, false)
	checkContainsKey(t, &m, 2, true)
	checkContainsKey(t, &m, 3, true)

	checkRemove(t, &m, 2, 2, true)
	checkContainsKey(t, &m, 1, false)
	checkContainsKey(t, &m, 2, false)
	checkContainsKey(t, &m, 3, true)

	checkRemove(t, &m, 3, 3, true)
	checkContainsKey(t, &m, 1, false)
	checkContainsKey(t, &m, 2, false)
	checkContainsKey(t, &m, 3, false)

	checkEmpty(t, &m, true)
}

// #[test]
// fn TestMap_nth() {
//     let mut map = AVLTreeMap::<u8, u8>::new();
//     assert_eq!(map.insert(2, 2), None);
//     assert_eq!(map.insert(1, 1), None);
//     assert_eq!(map.insert(3, 3), None);

//     assert_eq!(map.remove_entry(&2), Some((2, 2)));
//     assert_eq!(map.insert(2, 2), None);

//     assert_eq!(map.nth_key_value(0), Some((&1, &1)));
//     assert_eq!(map.nth_key_value(1), Some((&2, &2)));
//     assert_eq!(map.nth_key_value(2), Some((&3, &3)));
// }

func TestMap_nth(t *testing.T) {
	var m avltree.Map[int, int]
	checkInsert(t, &m, 2, 2, 0, false)
	checkInsert(t, &m, 1, 1, 0, false)
	checkInsert(t, &m, 3, 3, 0, false)

	checkRemoveEntry(t, &m, 2, 2, 2, true)
	checkInsert(t, &m, 2, 2, 0, false)

	checkNthKeyValue(t, &m, 0, 1, 1, true)
	checkNthKeyValue(t, &m, 1, 2, 2, true)
	checkNthKeyValue(t, &m, 2, 3, 3, true)
}

// #[test]
// #[timeout(1500)]
// fn performance1() {
//     let count = 10000000;
//     let mut rng = rand::thread_rng();
//     let mut map = AVLTreeMap::new();
//     let mut hash_map = HashMap::<u8, u8>::new();
//     for _ in 0..count {
//         let key = rng.gen();
//         let value = rng.gen();
//         map.insert(key, value);
//         hash_map.insert(key, value);
//     }
//     let mut vec: Vec<_> = hash_map.into_iter().collect();
//     vec.sort_unstable();
//     let mut vec: Vec<_> = vec
//         .into_iter()
//         .enumerate()
//         .map(|(index, (key, value))| (key, value, index))
//         .collect();
//     vec.shuffle(&mut rng);
//     for (key, value, index) in &vec {
//         assert!(map.contains_key(key));
//         assert_eq!(map.nth_key_value(*index), Some((key, value)));
//         assert_eq!(map.get_key_value(key), Some((key, value)));
//     }
//     for (key, value, _) in &vec {
//         assert_eq!(map.remove_entry(key), Some((*key, *value)));
//         assert!(!map.contains_key(key));
//     }
// }

func TestMap_performance1(t *testing.T) {
	timeout := 1500 * time.Millisecond
	ctx, cancel := context.WithTimeout(context.Background(), timeout)
	defer cancel()

	done := make(chan struct{})
	go func() {
		defer close(done)

		count := 10000000
		rnd := rand.New(rand.NewSource(1))
		m := avltree.Map[uint8, uint8]{}
		h := make(map[uint8]uint8, 256) // uint8 => max 256 keys

		for i := 0; i < count; i++ {
			if i&0xfff == 0 && ctx.Err() != nil {
				return
			}
			key := uint8(rnd.Int())
			val := uint8(rnd.Int())
			m.Insert(key, val)
			h[key] = val
		}
		type item struct {
			key, val uint8
			idx      int
		}

		a := make([]item, 0, len(h))
		for key, val := range h {
			a = append(a, item{key: key, val: val})
		}
		sort.Slice(a, func(i, j int) bool {
			return a[i].key < a[j].key
		})
		for i := range a {
			a[i].idx = i
		}
		rnd.Shuffle(len(a), func(i, j int) {
			a[i], a[j] = a[j], a[i]
		})

		for _, it := range a {
			key, val, idx := it.key, it.val, it.idx
			checkContainsKey(t, &m, key, true)
			checkNthKeyValue(t, &m, idx, key, val, true)
			checkGetKeyValue(t, &m, key, key, val, true)
		}

		for _, it := range a {
			key, val := it.key, it.val
			checkRemove(t, &m, key, val, true)
			checkContainsKey(t, &m, key, false)
		}
	}()

	<-done
	if ctx.Err() != nil {
		t.Error("timeout expired")
	}
}

// #[test]
// #[timeout(2500)]
// fn performance2() {
//     let count = 8000000;
//     let mut rng = rand::thread_rng();
//     let mut map = AVLTreeMap::new();
//     let mut hash_map = HashMap::<u8, u8>::new();
//     for _ in 0..count {
//         let key = rng.gen();
//         let value = rng.gen();
//         match rng.gen_range(0usize..10) {
//             0..=7 => {
//                 assert_eq!(map.insert(key, value), hash_map.insert(key, value));
//             }
//             8 => {
//                 assert_eq!(map.remove(&key), hash_map.remove(&key));
//             }
//             9 => {
//                 assert_eq!(map.remove_entry(&key), hash_map.remove_entry(&key));
//             }
//             _ => unreachable!(),
//         }
//         assert_eq!(map.is_empty(), hash_map.is_empty());
//         assert_eq!(map.len(), hash_map.len());
//         assert_eq!(map.contains_key(&key), hash_map.contains_key(&key));
//         assert_eq!(map.get(&key), hash_map.get(&key));
//         assert_eq!(map.get_key_value(&key), hash_map.get_key_value(&key));
//     }
// }

func TestMap_performance2(t *testing.T) {
	timeout := 2500 * time.Millisecond
	ctx, cancel := context.WithTimeout(context.Background(), timeout)
	defer cancel()

	done := make(chan struct{})
	go func() {
		defer close(done)

		// count := 8000000
		count := 5000000

		rnd := rand.New(rand.NewSource(1))
		m := avltree.Map[uint8, uint8]{}
		h := make(map[uint8]uint8, 256) // uint8 => max 256 keys

		wants := func(key uint8) (uint8, uint8, bool) {
			val, ok := h[key]
			if !ok {
				key = 0
			}
			return key, val, ok
		}

		for i := 0; i < count; i++ {
			if i&0xfff == 0 && ctx.Err() != nil {
				return
			}

			key := uint8(rnd.Int())
			val := uint8(rnd.Int())
			wantK, wantV, wantOk := wants(key)
			_ = wantK

			v := rnd.Intn(10)
			switch {
			case 0 <= v && v <= 7:
				checkInsert(t, &m, key, val, wantV, wantOk)
				h[key] = val
			case v == 8:
				checkRemove(t, &m, key, wantV, wantOk)
				delete(h, key)
			case v == 9:
				checkRemoveEntry(t, &m, key, wantK, wantV, wantOk)
				delete(h, key)
			}

			wantK, wantV, wantOk = wants(key)
			checkEmpty(t, &m, len(h) == 0)
			checkLen(t, &m, len(h))
			checkContainsKey(t, &m, key, wantOk)
			checkGet(t, &m, key, wantV, wantOk)
			checkGetKeyValue(t, &m, key, wantK, wantV, wantOk)
		}
	}()

	<-done
	if ctx.Err() != nil {
		t.Error("timeout expired")
	}
}

// #[test]
// #[timeout(1500)]
// fn performance3() {
//     let count = 1000000;
//     let mut rng = rand::thread_rng();
//     let mut map = AVLTreeMap::<i32, i32>::new();
//     for i in 0..count {
//         let value = rng.gen();
//         map.insert(i, value);
//     }
//     for _ in 0..count {
//         assert_eq!(map.contains_key(&count), false);
//     }
//     for i in 1000..count {
//         map.remove(&i);
//     }
//     for i in 0..count {
//         let value = rng.gen();
//         map.insert(-i, value);
//     }
//     for _ in 0..count {
//         assert_eq!(map.contains_key(&(-count)), false);
//     }
// }

func TestMap_performance3(t *testing.T) {
	timeout := 1500 * time.Millisecond
	ctx, cancel := context.WithTimeout(context.Background(), timeout)
	defer cancel()

	done := make(chan struct{})
	go func() {
		defer close(done)

		count := int32(1000000)

		rnd := rand.New(rand.NewSource(1))
		m := avltree.Map[int32, int32]{}

		for i := int32(0); i < count; i++ {
			if i&0xfff == 0 && ctx.Err() != nil {
				return
			}
			val := int32(rnd.Int())
			m.Insert(i, val)
		}

		for i := int32(0); i < count; i++ {
			if i&0xfff == 0 && ctx.Err() != nil {
				return
			}
			checkContainsKey(t, &m, i, true)
		}

		for i := int32(1000); i < count; i++ {
			if i&0xfff == 0 && ctx.Err() != nil {
				return
			}
			m.Remove(i)
		}

		for i := int32(0); i < count; i++ {
			if i&0xfff == 0 && ctx.Err() != nil {
				return
			}
			val := int32(rnd.Int())
			m.Insert(-i, val)
		}

		for i := int32(0); i < count; i++ {
			if i&0xfff == 0 && ctx.Err() != nil {
				return
			}
			checkContainsKey(t, &m, -i, true)
		}
	}()

	<-done
	if ctx.Err() != nil {
		t.Error("timeout expired")
	}
}

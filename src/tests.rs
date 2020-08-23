/// Test for node

#[allow(unused_imports)]
mod node_test {
	
	use crate::node::Node;
	use crate::branch::Branch;
	
	#[test]
	fn node_default_test() {
		let test_node = Node::<i32>::default();
		assert_eq!(test_node, Node::Empty);
	}
	
	#[test]
	fn node_new_test() {
		let test_node = Node::<i32>::new();
		assert_eq!(test_node, Node::Empty)
	}
	
	#[test]
	fn node_ignores_test() {
		let mut test_node = Node::<i32>::new();
		test_node.insert(&1);
		
		assert_eq!(*test_node.ignore(), Box::new(Branch {
			key: 1,
			right: Node::Empty,
			left: Node::Empty,
		}));
		
		assert_eq!(*test_node.ignore_mut(), Box::new(Branch {
			key: 1,
			right: Node::Empty,
			left: Node::Empty,
		}));
	}
	
	#[test]
	fn node_get_key_test() {
		let test_node: Node<i32> = Node::NonEmpty(Box::new(Branch {
			key: 32,
			left: Node::Empty,
			right: Node::Empty,
		}));
		assert_eq!(32, *test_node.get_key());
	}
	
	#[test]
	fn node_insert_test() {
		let mut test_node = Node::new();
		test_node.insert(&3);
		assert_eq!(test_node, Node::NonEmpty(Box::new(Branch {
			key: 3,
			right: Node::Empty,
			left: Node::Empty,
		})));
	}
	
	#[test]
	fn node_insert_full_test() {
		let mut test_node = Node::new();
		
		test_node.insert(&3);
		assert_eq!(test_node, Node::NonEmpty(Box::new(
			Branch {
				key: 3,
				right: Node::Empty,
				left: Node::Empty,
			})));
		assert_eq!(*test_node.get_key(), 3);
		
		test_node.insert(&3);
		assert_eq!(test_node, Node::NonEmpty(Box::new(
			Branch {
				key: 3,
				left: Node::Empty,
				right: Node::NonEmpty(Box::new(
					Branch {
					key: 3,
					right: Node::Empty,
					left: Node::Empty,
				})),
			})));
		assert_eq!(*test_node.ignore().right.get_key(), 3);
		
		test_node.insert(&2);
		assert_eq!(test_node, Node::NonEmpty(Box::new(
			Branch {
				key: 3,
				right: Node::NonEmpty(Box::new(
					Branch {
					key: 3,
					right: Node::Empty,
					left: Node::Empty,
				})),
				left: Node::NonEmpty(Box::new(
					Branch {
					key: 2,
					right: Node::Empty,
					left: Node::Empty,
				})),
			})));
		assert_eq!(*test_node.ignore().left.get_key(), 2);
	}
	
	#[test]
	fn node_find_test() {
		let mut node_test = Node::new();
		assert_eq!(*node_test.find(&3), Node::Empty);
		
		node_test.insert(&3);
		node_test.insert(&4);
		
		assert_eq!(*node_test.find(&3), Node::NonEmpty(Box::new(
			Branch {
			key: 3,
			left: Node::Empty,
			right: Node::NonEmpty(Box::new(
				Branch {
				key: 4,
				left: Node::Empty,
				right: Node::Empty,
			})),
		})));
		
		assert_eq!(*node_test.find(&4), Node::NonEmpty(Box::new(
			Branch {
			key: 4,
			left: Node::Empty,
			right: Node::Empty,
		})));
		
		assert_eq!(*node_test.find(&2), Node::Empty);
	}
	
	#[test]
	fn node_min_max_test() {
		let mut node_test = Node::new();
		
		node_test.insert(&3);
		node_test.insert(&3);
		node_test.insert(&4);
		node_test.insert(&2);
		node_test.insert(&2);
		
		assert_eq!(*node_test.max(), Node::NonEmpty(Box::new(
			Branch {
			key: 4,
			left: Node::Empty,
			right: Node::Empty,
		})));
		
		assert_eq!(*node_test.min(), Node::NonEmpty(Box::new(
			Branch {
			key: 2,
			left: Node::Empty,
			right: Node::NonEmpty(Box::new(
				Branch {
				key: 2,
				left: Node::Empty,
				right: Node::Empty,
			})),
		})));
	}
	
	#[test]
	fn node_walk_test() {
		let mut node_test = Node::new();
		node_test.insert(&5);
		node_test.insert(&6);
		node_test.insert(&4);
		node_test.insert(&7);
		node_test.insert(&3);
		node_test.insert(&8);
		node_test.insert(&2);
		node_test.insert(&9);
		node_test.insert(&1);
		
		assert_eq!(node_test.walk(), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
	}
	
	#[test]
	fn node_drop_test() {
		let mut node_test = Node::new();
		
		node_test.insert(&3);
		node_test.insert(&1);
		node_test.insert(&4);
		node_test.insert(&3);
		node_test.insert(&5);
		node_test.insert(&6);
		
		//          3
		//        /  \
		//       1   4
		//          /  \
		//         3   5
		//             \
		//             6
		
		assert_eq!(node_test, Node::NonEmpty(Box::new(Branch {
			key: 3,
			left: Node::NonEmpty(Box::new(Branch {
				key: 1,
				right: Node::Empty,
				left: Node::Empty,
			})),
			right: Node::NonEmpty(Box::new(Branch {
				key: 4,
				left: Node::NonEmpty(Box::new(Branch {
					key: 3,
					right: Node::Empty,
					left: Node::Empty,
				})),
				right: Node::NonEmpty(Box::new(Branch {
					key: 5,
					left: Node::Empty,
					right: Node::NonEmpty(Box::new(Branch {
						key: 6,
						right: Node::Empty,
						left: Node::Empty,
					})),
				})),
			})),
		})));
		
		 node_test.ignore_mut().right.ignore_mut().right.rec_drop();
		
		//          3
		//        /  \
		//       1   4
		//          /
		//         3
		
		assert_eq!(node_test, Node::NonEmpty(Box::new(Branch {
			key: 3,
			left: Node::NonEmpty(Box::new(Branch {
				key: 1,
				right: Node::Empty,
				left: Node::Empty,
			})),
			right: Node::NonEmpty(Box::new(Branch {
				key: 4,
				right: Node::Empty,
				left: Node::NonEmpty(Box::new(Branch {
					key: 3,
					right: Node::Empty,
					left: Node::Empty,
				})),
			})),
		})));
	}
	
	#[test]
	fn node_remove_test() {
		let mut node_test = Node::new();
		
		node_test.insert(&3);
		node_test.insert(&1);
		node_test.insert(&4);
		node_test.insert(&3);
		node_test.insert(&5);
		node_test.insert(&6);
		
		//          3
		//        /  \
		//       1   4
		//          /  \
		//         3    5
		//               \
		//                6
		
		assert_eq!(node_test, Node::NonEmpty(Box::new(Branch {
			key: 3,
			left: Node::NonEmpty(Box::new(Branch {
				key: 1,
				right: Node::Empty,
				left: Node::Empty,
			})),
			right: Node::NonEmpty(Box::new(Branch {
				key: 4,
				left: Node::NonEmpty(Box::new(Branch {
					key: 3,
					right: Node::Empty,
					left: Node::Empty,
				})),
				right: Node::NonEmpty(Box::new(Branch {
					key: 5,
					left: Node::Empty,
					right: Node::NonEmpty(Box::new(Branch {
						key: 6,
						right: Node::Empty,
						left: Node::Empty,
					})),
				})),
			})),
		})));
		
		let safe = node_test.remove(&5);
		assert_eq!(safe.0, true);
		assert_eq!(safe.1, vec![6]);
		
		//          3
		//        /  \
		//       1   4
		//          /
		//         3
		
		assert_eq!(node_test, Node::NonEmpty(Box::new(Branch {
			key: 3,
			left: Node::NonEmpty(Box::new(Branch {
				key: 1,
				right: Node::Empty,
				left: Node::Empty,
			})),
			right: Node::NonEmpty(Box::new(Branch {
				key: 4,
				left: Node::NonEmpty(Box::new(Branch {
					key: 3,
					right: Node::Empty,
					left: Node::Empty,
				})),
				right: Node::Empty,
			})),
		})));
	}
}

/// Tests for Iterator

#[allow(unused_imports)]
mod iter_test {
	use crate::iter::TreeIter;
	use std::iter::FromIterator;
	
	#[test]
	fn iter_capacity_test() {
		let mut iter = TreeIter::with_capacity(10);
		
		iter.extend_from_slice(&[5; 4]);
		assert!(iter.capacity() >= 10);
		let cap = iter.capacity();
		
		iter.push_back(&0);
		assert_eq!(iter.capacity(), cap);
	}
	
	#[test]
	fn iter_reserve_test() {
		let mut iter = TreeIter::<i32>::with_capacity(500);
		iter.reserve(10);
		assert!(iter.capacity() >= 510);
	}
	
	#[test]
	fn iter_append_test() {
		let mut iter1 = TreeIter::from_iter(1..1500);
		let iter2 = TreeIter::from_iter(500..2000);
		
		iter1.append(iter2);
		
		let mut check = (1..1500).collect::<Vec<i32>>();
		check.extend(500..2000);
		
		assert_eq!(iter1.collect::<Vec<i32>>(), check);
	}
	
	#[test]
	fn iter_clear_test() {
		let mut iter = TreeIter::from_iter(1..1500);
		iter.clear();
		assert_eq!(iter.clone().collect::<Vec<i32>>(), vec![]);
		
		iter.extend(500..2000);
		iter.clear();
		assert_eq!(iter.collect::<Vec<i32>>(), vec![]);
	}
	
	#[test]
	fn iter_from_iterator_len_test() {
		let iter = TreeIter::from_iter((0..100).step_by(2));
		assert_eq!(iter.len(), 50);
	}
	
	#[test]
	fn iter_empty_test() {
		let not_empty_iter = TreeIter::from_iter(1..1000);
		assert_eq!(not_empty_iter.is_empty(), false);

		let empty_iter = TreeIter::<i32>::new();
		assert_eq!(empty_iter.is_empty(), true);
	}
	
	#[test]
	fn iter_dedup_test() {
		let mut iter = TreeIter::new();
		iter.extend(1..1500);
		iter.extend(500..2000);
		iter.extend(1000..2500);
		iter.full_dedup();
		assert_eq!(iter.collect::<Vec<i32>>(), (1..2500).collect::<Vec<i32>>());
	}
	
	#[test]
	fn iter_drain_test() {
		let mut iter = TreeIter::from_iter((0..2000).step_by(2));
		iter.drain(0..500);
		assert_eq!(iter.clone().collect::<Vec<i32>>(), ((1000..2000).step_by(2)).collect::<Vec<i32>>());
		
		iter.drain(..);
		assert_eq!(iter.collect::<Vec<i32>>(), vec![]);
	}
	
	#[test]
	fn iter_drain_filter_test() {
		let mut iter = TreeIter::from_iter(1..1501);
		
		iter.drain_filter(|x| *x % 2 == 0);
		assert_eq!(iter.len(), 750);
		
		iter.drain_filter(|x| *x % 3 == 0);
		assert_eq!(iter.len(), 500);
		
		iter.drain_filter(|x| *x % 5 == 0);
		assert_eq!(iter.len(), 400);
		
		iter.drain_filter(|x| *x % 7 == 0);
		assert_eq!(iter.len(), 343);
		
		iter.drain_filter(|x| *x % 11 == 0);
		assert_eq!(iter.len(), 313);
		
		iter.drain_filter(|x| *x % 13 == 0);
		assert_eq!(iter.len(), 287);
		
		iter.drain_filter(|x| *x % 17 == 0);
		assert_eq!(iter.len(), 269);
		
		iter.drain_filter(|x| *x % 19 == 0);
		assert_eq!(iter.len(), 254);
		
		iter.drain_filter(|x| *x % 23 == 0);
		assert_eq!(iter.len(), 243);
		
		iter.drain_filter(|x| *x % 29 == 0);
		assert_eq!(iter.len(), 236);
		
		iter.drain_filter(|x| *x % 31 == 0);
		assert_eq!(iter.len(), 230);
		
		iter.drain_filter(|x| *x % 37 == 0);
		assert_eq!(iter.len(), 228);
		
		assert_eq!(iter.collect::<Vec<i32>>(), vec![1, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103,
			107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193, 197, 199, 211, 223,
			227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347,
			349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463,
			467, 479, 487, 491, 499, 503, 509, 521, 523, 541, 547, 557, 563, 569, 571, 577, 587, 593, 599, 601, 607,
			613, 617, 619, 631, 641, 643, 647, 653, 659, 661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743,
			751, 757, 761, 769, 773, 787, 797, 809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863, 877, 881, 883,
			887, 907, 911, 919, 929, 937, 941, 947, 953, 967, 971, 977, 983, 991, 997, 1009, 1013, 1019, 1021, 1031,
			1033, 1039, 1049, 1051, 1061, 1063, 1069, 1087, 1091, 1093, 1097, 1103, 1109, 1117, 1123, 1129, 1151, 1153,
			1163, 1171, 1181, 1187, 1193, 1201, 1213, 1217, 1223, 1229, 1231, 1237, 1249, 1259, 1277, 1279, 1283, 1289,
			1291, 1297, 1301, 1303, 1307, 1319, 1321, 1327, 1361, 1367, 1373, 1381, 1399, 1409, 1423, 1427, 1429, 1433,
			1439, 1447, 1451, 1453, 1459, 1471, 1481, 1483, 1487, 1489, 1493, 1499]);  // all remain numbers
	}
	
	#[test]
	fn iter_extend_slice_test() {
		let mut iter = TreeIter::from_iter(1..1500);
		iter.extend_from_slice(&[500; 500]);
		
		let mut check = (1..1500).collect::<Vec<i32>>();
		check.extend_from_slice(&[500; 500]);
		
		assert_eq!(iter.collect::<Vec<i32>>(), check);
	}
	
	#[test]
	fn iter_insert_test() {
		let mut iter = TreeIter::new();
		iter.insert(0, &1);
		iter.insert(0, &2);
		iter.insert(0, &3);
		assert_eq!(iter.collect::<Vec<i32>>(), vec![3, 2, 1]);
	}
	
	#[test]
	fn iter_push_front_test() {
		let mut iter = TreeIter::from_iter(1..500);
		iter.push_front(&0);
		assert_eq!(iter.collect::<Vec<i32>>(), (0..500).collect::<Vec<i32>>());
	}
	
	#[test]
	fn iter_pop_front_test() {
		let mut iter = TreeIter::from_iter(1..500);
		for i in 1..500 {
			assert_eq!(iter.pop_front().unwrap(), i);
		}
		assert_eq!(iter.collect::<Vec<i32>>(), vec![]);
	}
	
	#[test]
	fn iter_push_back_test() {
		let mut iter = TreeIter::from_iter(1..500);
		iter.push_back(&500);
		assert_eq!(iter.collect::<Vec<i32>>(), (1..= 500).collect::<Vec<i32>>());
	}
	
	#[test]
	fn iter_pop_back_test() {
		let mut iter = TreeIter::from_iter(1..500);
		for i in 1..500 {
			assert_eq!(iter.pop_back().unwrap(), 500 - i);
		}
		assert_eq!(iter.collect::<Vec<i32>>(), vec![]);
	}
	
	#[test]
	fn iter_remove_test() {
		let mut iter = TreeIter::new();
		iter.extend_from_slice(&[4, 3, 2, 1]);
		assert_eq!(iter.remove(1), Some(3));
		assert_eq!(iter.remove(1), Some(2));
		assert_eq!(iter.remove(1), Some(1));
		assert_eq!(iter.collect::<Vec<i32>>(), vec![4]);
	}
	
	#[test]
	fn iter_retain_test() {
		let mut iter = TreeIter::from_iter(1..1501);
		
		iter.retain(|x| *x % 2 != 0);
		assert_eq!(iter.len(), 750);
		
		iter.retain(|x| *x % 3 != 0);
		assert_eq!(iter.len(), 500);
		
		iter.retain(|x| *x % 5 != 0);
		assert_eq!(iter.len(), 400);
		
		iter.retain(|x| *x % 7 != 0);
		assert_eq!(iter.len(), 343);
		
		iter.retain(|x| *x % 11 != 0);
		assert_eq!(iter.len(), 313);
		
		iter.retain(|x| *x % 13 != 0);
		assert_eq!(iter.len(), 287);
		
		iter.retain(|x| *x % 17 != 0);
		assert_eq!(iter.len(), 269);
		
		iter.retain(|x| *x % 19 != 0);
		assert_eq!(iter.len(), 254);
		
		iter.retain(|x| *x % 23 != 0);
		assert_eq!(iter.len(), 243);
		
		iter.retain(|x| *x % 29 != 0);
		assert_eq!(iter.len(), 236);
		
		iter.retain(|x| *x % 31 != 0);
		assert_eq!(iter.len(), 230);
		
		iter.retain(|x| *x % 37 != 0);
		assert_eq!(iter.len(), 228);
		
		assert_eq!(iter.collect::<Vec<i32>>(), vec![1, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103,
			107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193, 197, 199, 211, 223,
			227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347,
			349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463,
			467, 479, 487, 491, 499, 503, 509, 521, 523, 541, 547, 557, 563, 569, 571, 577, 587, 593, 599, 601, 607,
			613, 617, 619, 631, 641, 643, 647, 653, 659, 661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743,
			751, 757, 761, 769, 773, 787, 797, 809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863, 877, 881, 883,
			887, 907, 911, 919, 929, 937, 941, 947, 953, 967, 971, 977, 983, 991, 997, 1009, 1013, 1019, 1021, 1031,
			1033, 1039, 1049, 1051, 1061, 1063, 1069, 1087, 1091, 1093, 1097, 1103, 1109, 1117, 1123, 1129, 1151, 1153,
			1163, 1171, 1181, 1187, 1193, 1201, 1213, 1217, 1223, 1229, 1231, 1237, 1249, 1259, 1277, 1279, 1283, 1289,
			1291, 1297, 1301, 1303, 1307, 1319, 1321, 1327, 1361, 1367, 1373, 1381, 1399, 1409, 1423, 1427, 1429, 1433,
			1439, 1447, 1451, 1453, 1459, 1471, 1481, 1483, 1487, 1489, 1493, 1499]);  // all remain numbers
	}
	
	#[test]
	fn iter_shrink_test() {
		let mut iter = TreeIter::with_capacity(10);

		iter.extend_from_slice(&[1, 2, 3]);
		assert!(iter.capacity() >= 10);

		iter.shrink_to_fit();
		assert!(iter.capacity() >= 3);
	}
	
	#[test]
	fn iter_split_off_test() {
		let mut iter1 = TreeIter::from_iter(0..5000);
	    let iter2 = iter1.split_off(2500);
	
		assert_eq!(iter1.collect::<Vec<i32>>(), (0..2500).collect::<Vec<i32>>());
		assert_eq!(iter2.collect::<Vec<i32>>(), (2500..5000).collect::<Vec<i32>>());
	}
	
	#[test]
	fn iter_swap_back() {
		let mut iter = TreeIter::from_iter(1..5);
		
		assert_eq!(iter.swap_remove_back(0).unwrap(), 1);
		assert_eq!(iter.clone().collect::<Vec<i32>>(), vec![4, 2, 3]);
		
		assert_eq!(iter.swap_remove_back(0).unwrap(), 4);
		assert_eq!(iter.clone().collect::<Vec<i32>>(), vec![3, 2]);
		
		assert_eq!(iter.swap_remove_back(0).unwrap(), 3);
		assert_eq!(iter.clone().collect::<Vec<i32>>(), vec![2]);
		
		assert_eq!(iter.swap_remove_back(0).unwrap(), 2);
		assert_eq!(iter.clone().collect::<Vec<i32>>(), vec![]);
	}
	
	#[test]
	fn iter_swap_front() {
		let mut iter = TreeIter::from_iter(1..5);
		
		assert_eq!(iter.swap_remove_front(iter.len() - 1).unwrap(), 4);
		assert_eq!(iter.clone().collect::<Vec<i32>>(), vec![2, 3, 1]);
		
		assert_eq!(iter.swap_remove_front(iter.len() - 1).unwrap(), 1);
		assert_eq!(iter.clone().collect::<Vec<i32>>(), vec![3, 2]);
		
		assert_eq!(iter.swap_remove_front(iter.len() - 1).unwrap(), 2);
		assert_eq!(iter.clone().collect::<Vec<i32>>(), vec![3]);
		
		assert_eq!(iter.swap_remove_front(iter.len() - 1).unwrap(), 3);
		assert_eq!(iter.clone().collect::<Vec<i32>>(), vec![]);
	}
	
	#[test]
	fn iter_truncate_test() {
		let mut iter = TreeIter::from_iter(1..1000);
		iter.truncate(500);
		assert_eq!(iter.collect::<Vec<i32>>(), (1..501).collect::<Vec<i32>>());
	}
	
	#[test]
	fn iter_extend_test() {
		let mut iter = TreeIter::new();
		
		iter.extend(vec![1, 2, 3, 4, 5]);
		assert_eq!(iter.clone().collect::<Vec<i32>>(), vec![1, 2, 3, 4, 5]);
		
		iter.extend(vec![1, 2, 3, 4, 5]);
		assert_eq!(iter.collect::<Vec<i32>>(), vec![1, 2, 3, 4, 5, 1, 2, 3, 4, 5]);
	}
}

/// Tests for tree

#[allow(unused_imports)]
mod tree_test {
	use crate::tree::BinaryTree;
	use crate::node::Node;
	use crate::iter::TreeIter;
	use crate::branch::Branch;
	use std::collections::VecDeque;
	use std::iter::FromIterator;
	
	#[test]
	fn tree_default_test() {
		let test_tree:BinaryTree<i32> = BinaryTree::default();
		assert_eq!(
			test_tree,
			BinaryTree {
				top: Node::Empty,
				size: 0,
			}
		)
	}
	
	#[test]
	fn tree_new_test() {
		let test_tree:BinaryTree<i32> = BinaryTree::new();
		assert_eq!(
			test_tree,
			BinaryTree {
				top: Node::Empty,
				size: 0,
			}
		);
	}
	
	#[test]
	fn tree_len_test() {
		let mut tree_test = BinaryTree::from_iter((0..100).step_by(2));
		assert_eq!(tree_test.len(), 50);
		
		tree_test.extend(vec![1; 50]);
		assert_eq!(tree_test.len(), 100);
		
		tree_test.drain_filter(|x| x % 2 == 0);
		assert_eq!(tree_test.len(), 50);
	}
	
	#[test]
	fn tree_insert_test() {
		let mut test_tree = BinaryTree::new();
		test_tree.insert(&3);
		assert_eq!(test_tree, BinaryTree {
			top: Node::NonEmpty(Box::new(Branch {
				key: 3,
				right: Node::Empty,
				left: Node::Empty,
			})),
			size: 1,
		});
	}
	
	#[test]
	fn tree_insert_full_test() {
		let mut tree_test = BinaryTree::new();
		tree_test.insert(&3);
		tree_test.insert(&2);
		tree_test.insert(&3);
		
		assert_eq!(tree_test, BinaryTree {
			top: Node::NonEmpty(Box::new(Branch {
				key: 3,
				right:  Node::NonEmpty(Box::new(Branch {
					key: 3,
					right: Node::Empty,
					left: Node::Empty,
				})),
				left: Node::NonEmpty(Box::new(Branch {
					key: 2,
					right: Node::Empty,
					left: Node::Empty,
				})),
			})),
			size: 3,
		});
	}
	
	#[test]
	fn tree_empty_test() {
		let mut nonempty = BinaryTree::new();
		
		nonempty.insert(&1);
		assert_eq!(nonempty.is_empty(), false);
		
		nonempty.remove(&1);
		assert_eq!(nonempty.is_empty(), true);
		
		let empty = BinaryTree::<i32>::new();
		assert_eq!(empty.is_empty(), true);
	}
	
	#[test]
	fn tree_contains_test() {
		let mut tree_test = BinaryTree::new();
		tree_test.insert(&3);
		tree_test.insert(&2);
		tree_test.insert(&4);
		tree_test.insert(&5);
		tree_test.insert(&6);
		tree_test.insert(&7);
		
		assert_eq!(tree_test.contains(&3), true);
		assert_eq!(tree_test.contains(&2), true);
		assert_eq!(tree_test.contains(&8), false);
		assert_eq!(tree_test.contains(&4), true);
		assert_eq!(tree_test.contains(&5), true);
		assert_eq!(tree_test.contains(&9), false);
		assert_eq!(tree_test.contains(&6), true);
		assert_eq!(tree_test.contains(&1), false);
		assert_eq!(tree_test.contains(&7), true);
	}
	
	#[test]
	fn tree_first_last_test() {
		let mut tree_test = BinaryTree::new();
		
		tree_test.insert(&3);
		assert_eq!(*tree_test.first(), 3);
		assert_eq!(*tree_test.last(), 3);
		
		tree_test.insert(&2);
		assert_eq!(*tree_test.first(), 2);
		assert_eq!(*tree_test.last(), 3);
		
		tree_test.insert(&3);
		assert_eq!(*tree_test.first(), 2);
		assert_eq!(*tree_test.last(), 3);
		
		tree_test.insert(&4);
		assert_eq!(*tree_test.first(), 2);
		assert_eq!(*tree_test.last(), 4);
		
		tree_test.insert(&3);
		assert_eq!(*tree_test.first(), 2);
		assert_eq!(*tree_test.last(), 4);
		
		tree_test.insert(&4);
		assert_eq!(*tree_test.first(), 2);
		assert_eq!(*tree_test.last(), 4);
		
		tree_test.insert(&1);
		assert_eq!(*tree_test.first(), 1);
		assert_eq!(*tree_test.last(), 4);
	}
	
	#[test]
	fn tree_iter_test() {
		let mut tree_test = BinaryTree::new();
		
		tree_test.insert(&5);
		tree_test.insert(&6);
		tree_test.insert(&4);
		tree_test.insert(&7);
		tree_test.insert(&3);
		tree_test.insert(&8);
		tree_test.insert(&2);
		tree_test.insert(&9);
		tree_test.insert(&1);
		
		assert_eq!(tree_test.iter(), TreeIter { iter: VecDeque::from(vec![1, 2, 3, 4, 5, 6, 7, 8, 9])});
	}
	
	#[test]
	fn tree_append_test() {
		let mut first = BinaryTree::new();
		let mut second = BinaryTree::new();
		
		first.insert(&1);
		second.insert(&2);
		first.append(&second);
		
		assert_eq!(first, BinaryTree {
			top: Node::NonEmpty(Box::new(Branch {
				key: 1,
				right: Node::NonEmpty(Box::new(Branch {
					key: 2,
					right: Node::Empty,
					left: Node::Empty,
				})),
				left: Node::Empty,
			})),
			size: 2,
		});
		
		assert_eq!(second, BinaryTree {
			top: Node::NonEmpty(Box::new(Branch {
				key: 2,
				right: Node::Empty,
				left: Node::Empty,
			})),
			size: 1,
		});
	}
	
	#[test]
	fn tree_clear_test() {
		let mut tree = BinaryTree::from_iter(1..1000);
		tree.clear();
		assert_eq!(tree, BinaryTree {
			top: Node::Empty,
			size: 0,
		});
	}
	
	#[test]
	fn tree_remove_test() {
		let mut tree = BinaryTree::from_iter(1..10);
		
		tree.remove(&1);
		assert_eq!(tree.len(), 8);
		
		tree.remove(&2);
		assert_eq!(tree.len(), 7);
		
		tree.remove(&3);
		assert_eq!(tree.len(), 6);
		
		tree.remove(&4);
		assert_eq!(tree.len(), 5);
		
		tree.remove(&5);
		assert_eq!(tree.len(), 4);
		
		tree.remove(&6);
		assert_eq!(tree.len(), 3);
		
		tree.remove(&7);
		assert_eq!(tree.len(), 2);
		
		tree.remove(&8);
		assert_eq!(tree.len(), 1);
		
		tree.remove(&9);
		assert_eq!(tree.len(), 0);
	}
	
	#[test]
	fn tree_difference_test() {
		let tree1 = BinaryTree::from_iter(1..1000);
		let tree2 = BinaryTree::from_iter(500..1500);
		
		assert_eq!(tree1.difference(&tree2).collect::<Vec<i32>>(), Vec::from_iter(1..500));
	}
	
	#[test]
	fn tree_drain_filter_test() {
		
		// Eratosthenes's sieve
		
		let mut tree = BinaryTree::from_iter(1..1501);
		assert_eq!(tree.contains(&37), true);
		
		tree.drain_filter(|x| x % 2 == 0);
		assert_eq!(tree.len(), 750);
		assert_eq!(tree.contains(&37), true);
		
		tree.drain_filter(|x| x % 3 == 0);
		assert_eq!(tree.len(), 500);
		assert_eq!(tree.contains(&37), true);
		
		tree.drain_filter(|x| x % 5 == 0);
		assert_eq!(tree.len(), 400);
		assert_eq!(tree.contains(&37), true);
		
		tree.drain_filter(|x| x % 7 == 0);
		assert_eq!(tree.len(), 343);
		assert_eq!(tree.contains(&37), true);
		
		tree.drain_filter(|x| x % 11 == 0);
		assert_eq!(tree.len(), 313);
		assert_eq!(tree.contains(&37), true);
		
		tree.drain_filter(|x| x % 13 == 0);
		assert_eq!(tree.len(), 287);
		assert_eq!(tree.contains(&37), true);
		
		tree.drain_filter(|x| x % 17 == 0);
		assert_eq!(tree.len(), 269);
		assert_eq!(tree.contains(&37), true);
		
		tree.drain_filter(|x| x % 19 == 0);
		assert_eq!(tree.len(), 254);
		assert_eq!(tree.contains(&37), true);
		
		tree.drain_filter(|x| x % 23 == 0);
		assert_eq!(tree.len(), 243);
		assert_eq!(tree.contains(&37), true);
		
		tree.drain_filter(|x| x % 29 == 0);
		assert_eq!(tree.len(), 236);
		assert_eq!(tree.contains(&37), true);
		
		tree.drain_filter(|x| x % 31 == 0);
		assert_eq!(tree.len(), 230);
		assert_eq!(tree.contains(&37), true);
		
		tree.drain_filter(|x| x % 37 == 0);
		assert_eq!(tree.len(), 228);
		assert_eq!(tree.contains(&37), false);
		
		assert_eq!(tree.to_vec(), vec![1, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109,
		 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229,
		 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353,
		 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479,
		 487, 491, 499, 503, 509, 521, 523, 541, 547, 557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617,
		 619, 631, 641, 643, 647, 653, 659, 661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743, 751, 757,
		 761, 769, 773, 787, 797, 809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907,
		 911, 919, 929, 937, 941, 947, 953, 967, 971, 977, 983, 991, 997, 1009, 1013, 1019, 1021, 1031, 1033, 1039,
		 1049, 1051, 1061, 1063, 1069, 1087, 1091, 1093, 1097, 1103, 1109, 1117, 1123, 1129, 1151, 1153, 1163, 1171,
		 1181, 1187, 1193, 1201, 1213, 1217, 1223, 1229, 1231, 1237, 1249, 1259, 1277, 1279, 1283, 1289, 1291, 1297,
		 1301, 1303, 1307, 1319, 1321, 1327, 1361, 1367, 1373, 1381, 1399, 1409, 1423, 1427, 1429, 1433, 1439, 1447,
		 1451, 1453, 1459, 1471, 1481, 1483, 1487, 1489, 1493, 1499]);  // all remain numbers
	}
	
	#[test]
	fn tree_intersection_test() {
		let tree1 = BinaryTree::from_iter(1..1000);
		let  tree2 = BinaryTree::from_iter(500..1500);
		
		assert_eq!(tree1.intersection(&tree2), TreeIter::from_iter(500..1000));
	}
	
	#[test]
	fn tree_is_disjoint_test() {
		let tree1 = BinaryTree::from_iter(1..1000);
		let  tree2 = BinaryTree::from_iter(500..1500);
		assert_eq!(tree1.is_disjoint(&tree2), false);
		
		let tree3 = BinaryTree::from_iter(1..1000);
		let  tree4 = BinaryTree::from_iter(1000..1500);
		assert_eq!(tree3.is_disjoint(&tree4), true);
	}
	
	#[test]
	fn tree_pop_first() {
		let mut tree = BinaryTree::from_iter(1..10);
		assert_eq!(*tree.first(), 1);
		
		tree.pop_first();
		assert_eq!(*tree.first(), 2);
		
		tree.pop_first();
		assert_eq!(*tree.first(), 3);
		
		tree.pop_first();
		assert_eq!(*tree.first(), 4);
		
		tree.pop_first();
		assert_eq!(*tree.first(), 5);
		
		tree.pop_first();
		assert_eq!(*tree.first(), 6);
		
		tree.pop_first();
		assert_eq!(*tree.first(), 7);
		
		tree.pop_first();
		assert_eq!(*tree.first(), 8);
		
		tree.pop_first();
		assert_eq!(*tree.first(), 9);
		
		tree.pop_first();
		assert_eq!(tree.len(), 0);
	}
	
	#[test]
	fn tree_pop_last() {
		let mut tree = BinaryTree::from_iter(1..10);
		assert_eq!(*tree.last(), 9);
		
		tree.pop_last();
		assert_eq!(*tree.last(), 8);
		
		tree.pop_last();
		assert_eq!(*tree.last(), 7);
		
		tree.pop_last();
		assert_eq!(*tree.last(), 6);
		
		tree.pop_last();
		assert_eq!(*tree.last(), 5);
		
		tree.pop_last();
		assert_eq!(*tree.last(), 4);
		
		tree.pop_last();
		assert_eq!(*tree.last(), 3);
		
		tree.pop_last();
		assert_eq!(*tree.last(), 2);
		
		tree.pop_last();
		assert_eq!(*tree.last(), 1);
		
		tree.pop_last();
		assert_eq!(tree.len(), 0);
	}
	
	#[test]
	fn tree_replace_val_test() {
		let mut tree = BinaryTree::new();
		tree.extend(vec![1; 1000]);
		
		tree.replace_val(&1, &2);
		assert_eq!(tree.to_vec(), vec![2; 1000]);
	}
	
	#[test]
	fn tree_sym_dif_test() {
		let tree1 = BinaryTree::from_iter(1..1000);
		let tree2 = BinaryTree::from_iter(500..1500);
		
		let mut check = vec![];
		check.extend(1..500);
		check.extend(1000..1500);
		
		assert_eq!(tree1.symmetric_difference(&tree2).collect::<Vec<i32>>(), check);
	}
	
	#[test]
	fn tree_union_test() {
		let tree1 = BinaryTree::from_iter(1..1000);
		let tree2 = BinaryTree::from_iter(500..1500);
		
		assert_eq!(tree1.union(&tree2).collect::<BinaryTree<i32>>().to_vec(), (1..1500).collect::<Vec<i32>>());
	}
	
	#[test]
	fn tree_multi_remove_test() {
		let mut tree = BinaryTree::new();
		
		tree.extend(vec![25; 50]);
		assert_eq!(tree.to_vec(), vec![25; 50]);
		
		tree.multi_remove(vec![25; 25]);
		assert_eq!(tree.to_vec(), vec![25; 25]);
		
		let mut tree2 = BinaryTree::new();
		
		tree2.extend(vec![75; 100]);
		tree2.extend(vec![100; 100]);
		
		tree2.multi_remove(vec![75; 25]);
		tree2.multi_remove(vec![100; 75]);
		
		let mut check = vec![75; 75];
		check.extend(vec![100; 25]);
		
		assert_eq!(tree2.to_vec(), check);
	}
	
	#[test]
	fn tree_bitand_test() {
		let tree1 = BinaryTree::from_iter(1..1500);
		let tree2 = BinaryTree::from_iter(500..2000);
		assert_eq!((&tree1 & &tree2).to_vec(), (500..1500).collect::<Vec<i32>>());
	}
	
	#[test]
	fn tree_bitor_test() {
		let tree1 = BinaryTree::from_iter(1..1500);
		let tree2 = BinaryTree::from_iter(500..2000);
		assert_eq!((&tree1 | &tree2).to_vec(), (1..2000).collect::<Vec<i32>>());
	}
	
	#[test]
	fn tree_bitxor_test() {
		let tree1 = BinaryTree::from_iter(1..1500);
		let tree2 = BinaryTree::from_iter(500..2000);
		
		let mut check= Vec::new();
		check.extend(1..500);
		check.extend(1500..2000);
		
		assert_eq!((&tree1 ^ &tree2).to_vec(), check);
	}
}

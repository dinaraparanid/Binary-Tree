use crate::node::Node;
use crate::iter::TreeIter;
use crate::branch::Branch;
use std::iter::FromIterator;
use std::collections::VecDeque;
use std::ops::{BitAnd, BitOr, BitXor};

/// **Realisation of Binary Search Tree in Rust lang**
/// ---------------------------------------------------------
/// *English*: **ABOUT STRUCTURE**
///
/// *Binary Search Tree* is a directed graph, each *node* of it has a *key* -
/// a certain value that can be compared and two *branches* - *right* and *left*.
/// (we will assume that tree *always* have branches, but they can be
/// *empty*, or *contain a value*).
///
/// The *left branch* stores nodes whose keys (values) are *less* than the key of the *current node*.
/// In the classic implementation, the binary tree does not store the same keys.
/// In my implementation, this is possible, nodes whose keys are *greater
/// or equal to the current one* are stored in the *left branch*.
///
/// You can add all values, if they implement Clone, Copy, Eq, Ord traits.
/// It was made to not broke tree's logic (NAN value).
///
/// Tree can be used as *sorted multiset*
/// ```C++
/// std::multiset // C++
///``` 
/// from C++: you can collect
/// equal and not equal values in a sorted order, make some manipulations with tree
/// like insertion, clearing, removing values, change equal values to other values and so on.
/// Check **method list** and examples to be familiar with my crate.
/// Also you can visit the project's repository https://github.com/dinaraparanid/Binary-Tree
/// And check the full code. I highly recommend to see **tests.rs** file.
/// There are a lot of examples of code. so you'll easily figure out how to use it
///
/// *Russian*: **О СТРУКТУРЕ**
///
/// *Бинарное дерево поиска* - условно направленный граф,
/// каждый узел которого имеет *ключ* - некое значение, которое можно сравнивать
/// и две ветви - правую и левую.
/// (будем считать, что ветви есть всегда, просто они могут быть
/// либо пустыми, либо содержать значение).
///
/// В левой ветви хранятся узлы, ключи (значения) которых меньше ключа текущего
/// узла. В классической реализации бинарное дерево не хранит одинаковые ключи.
/// В моей реализации это возможно, узлы, ключи которых больше или равны текущему
/// хранятся в левой ветви.
///
/// Дерево может хранить только те типы, которые реализуют
/// трейты Clone, Copy, Ord, Eq. Это сделанно с целью не нарушать логику дерева (NAN)
///
/// Дерево можно использовать как отсортированный мультисэт
/// ```C++
/// std::multiset // C++
/// ```
/// из C++:
/// Можно хранить одинаковые ключи, добавлять или удалять их из дерева, заменять
/// одинаковые ключи на другие значения. Изучите список методов и примеры
/// для полного понимания возможностей дерева. Так же советую посмотреть
/// репозиторий проекта https://github.com/dinaraparanid/Binary-Tree на котором
/// есть весь код. Особенно стоит изучить **tests.rs** файл. Там много примеров
/// того, как всё это можно использовать.
///
/// *English*: The tree itself. Contains the *top node* (start of tree) and it's *size*.
///
/// *Russian*: Само дерево. Храним *головной узел - начало дерева*, и *размер*.

#[derive(PartialEq, Debug, Clone)]
pub struct BinaryTree<T>
	where T: Copy + Clone + Ord + Eq
{
	pub(crate) top: Node<T>,
	pub(crate) size: usize,
}

/// *English*: **Default trait** for tree. Bu default tree is *empty*.
///
/// *Russian*: *Характеристика Default*. По-умолчанию дерево *пусто*.
///
/// # Example
/// ```
/// use binartree::tree::BinaryTree;
///
/// let tree = BinaryTree::<i32>::default();
///
/// assert_eq!(tree.len(), 0);
/// assert_eq!(tree.to_vec(), vec![]);
/// ```

impl<T> Default for BinaryTree<T>
	where T: Copy + Clone + Ord + Eq
{
	#[inline]
	fn default() -> Self {
		BinaryTree {
			top: Node::Empty,
			size: 0,
		}
	}
}

/// Realisation of tree's methods.
/// ------------------------------------------

impl<T> BinaryTree<T>
	where T: Copy + Clone + Ord + Eq
{
	
	/// *English*: Method **new()** creates *empty* tree.
	/// You can call it a constructor
	///
	/// *Russian*: Метод **new()** создаёт *пустое* дерево.
	/// По сути является конструктором для дерева.
	///
	/// # Example
	///
	/// ```
	/// use binartree::tree::BinaryTree;
	///
	/// let new_tree = BinaryTree::<i32>::new();
	///
	/// assert_eq!(new_tree.len(), 0);
	/// assert_eq!(new_tree.to_vec(), vec![]);
	/// ```
	
	#[inline]
	pub fn new() -> Self {
		BinaryTree {
			top: Node::Empty,
			size: 0,
		}
	}
	
	/// *English*: Method **len()** returns *tree's length*.
	///
	/// *Russian*: Метод **len()** возвращает *длину дерева*.
	///
	/// # Example
	///
	/// ```
	/// use binartree::tree::BinaryTree;
	///
	/// let mut tree = BinaryTree::new();
	/// assert_eq!(tree.len(), 0);
	///
	/// tree.insert(&1);
	/// tree.insert(&2);
	/// tree.insert(&3);
	///
	/// assert_eq!(tree.len(), 3);
	/// ```
	
	#[inline]
	pub fn len(&self) -> usize {
		self.size
	}
	
	/// *English*: Method **is_empty()** answers the question: "is out tree empty?"
	/// If it's true, returns *true*, else *false*.
	///
	/// *Russian*: Метод **is_empty()** отвечает на вопрос: "пусто ли наше дерево?"
	/// Если пусто, то возвращает *true*, иначе *false*.
	///
	/// # Example
	///
	/// ```
	/// use binartree::tree::BinaryTree;
	///
	/// let empty_tree = BinaryTree::<i32>::new();
	/// let mut not_empty_tree = BinaryTree::new();
	///
	/// not_empty_tree.insert(&1);
	/// not_empty_tree.insert(&2);
	/// not_empty_tree.insert(&3);
	///
	/// assert_eq!(empty_tree.len(), 0);
	/// assert_eq!(empty_tree.is_empty(), true);
	///
	/// assert_eq!(not_empty_tree.len(), 3);
	/// assert_eq!(not_empty_tree.is_empty(), false);
	/// ```
	
	#[inline]
	pub fn is_empty(&self) -> bool {
		self.size == 0
	}
	
	/// *English*: Method **to_vec()** converts tree to *std::vec::Vec*
	///
	/// *Russian*: Метод **to_vec()** конвертирует дерево в *std::vec::Vec*
	///
	/// # Example
	/// ```
	/// use binartree::tree::BinaryTree;
	/// use std::iter::FromIterator;
	///
	/// let tree = BinaryTree::from_iter((1..6));
	/// assert_eq!(tree.to_vec(), vec![1, 2, 3, 4, 5]);
	/// ```

	#[inline]
	pub fn to_vec(&self) -> Vec<T> {
		Vec::from(self.top.walk())
	}
	
	/// *English*: Method **to_deque()** converts tree to *std::collections::VecDeque*
	///
	/// *Russian*: Метод **to_deque()** конвертирует дерево в *std::collections::VecDeque*
	///
	/// # Example
	/// ```
	/// use binartree::tree::BinaryTree;
	/// use std::iter::FromIterator;
	/// use std::collections::VecDeque;
	///
	/// let tree = BinaryTree::from_iter((1..6));
	/// assert_eq!(tree.to_deque(), VecDeque::from_iter(1..6));
	/// ```
	
	#[inline]
	pub fn to_deque(&self) -> VecDeque<T> {
		self.top.walk()
	}
	
	/// *English*: Method **insert()** adds value to tree.
	/// Value takes by *immutable reference* (&T), so there is no ownership.
	///
	/// *Russian*: Метод **insert()** добавляет значение в дерево.
	/// Значение берётся по *неизменяемой ссылке* (&T), так что
	/// владение исключено.
	///
	/// # Example
	///
	/// ```
	/// use binartree::tree::BinaryTree;
	///
	/// let mut tree = BinaryTree::new();
	/// assert_eq!(tree.len(), 0);
	///
	/// tree.insert(&1);
	/// tree.insert(&2);
	/// tree.insert(&3);
	///
	/// assert_eq!(tree.len(), 3);
	/// assert_eq!(tree.contains(&1), true);
	/// assert_eq!(tree.contains(&2), true);
	/// assert_eq!(tree.contains(&3), true);
	/// assert_eq!(tree.to_vec(), vec![1, 2, 3]);
	/// ```
	
	pub fn insert(&mut self, val: &T) {
		if let Node::Empty = self.top {
			self.top = Node::NonEmpty(Box::new(Branch {
				key: (*val).clone(),
				right: Node::Empty,
				left: Node::Empty,
			}));
			self.size = 1;
		} else {
			self.top.insert(val);
			self.size += 1;
		}
	}
	
	/// *English*: Method **contains()** checks that value is in the tree.
	/// If it's true, returns *true*, else *false*
	///
	/// *Russian*: Метод **contains()** проверяет наличие значения в дереве
	/// Если оно есть, то возвращает *true*, иначе *false*
	///
	/// # Example
	///
	/// ```
	/// use binartree::tree::BinaryTree;
	///
	/// let mut tree = BinaryTree::new();
	/// tree.insert(&1);
	/// tree.insert(&3);
	///
	/// assert_eq!(tree.len(), 2);
	/// assert_eq!(tree.contains(&1), true);
	/// assert_eq!(tree.contains(&2), false);
	/// assert_eq!(tree.contains(&3), true);
	/// assert_eq!(tree.to_vec(), vec![1, 3]);
	/// ```
	
	#[inline]
	pub fn contains(&self, val: &T) -> bool {
		*self.top.find(&val) != Node::Empty
	}
	
	/// *English*: Method **first()** returns *minimum value in the tree*.
	/// if tree is empty, it'll panic.
	///
	/// *Russian*: Метод **first()** возвращает *минимальное значение
	/// в дереве*. Если дерево пустое, то будет вызвана *паника*.
	///
	/// # Example
	///
	/// ```
	/// use binartree::tree::BinaryTree;
	///
	/// let mut tree = BinaryTree::new();
	///
	/// tree.insert(&3);
	/// assert_eq!(tree.first(), &3);
	///
	/// tree.insert(&4);
	///  assert_eq!(tree.first(), &3);
	///
	/// tree.insert(&1);
	///  assert_eq!(tree.first(), &1);
	/// ```
	
	#[inline]
	pub fn first(&self) -> &T {
		self.top.min().get_key()
	}
	
	/// *English*: Method **last()** returns *maximum value in the tree*.
	/// if tree is empty, than it'll panic.
	///
	/// *Russian*: Метод **first()** возвращает *максимальное значение
	/// в дереве*. Если дерево пустое, то будет вызвана *паника*.
	///
	/// # Example
	///
	/// ```
	/// use binartree::tree::BinaryTree;
	///
	/// let mut tree = BinaryTree::new();
	///
	/// tree.insert(&1);
	/// assert_eq!(tree.last(), &1);
	///
	/// tree.insert(&0);
	/// assert_eq!(tree.last(), &1);
	///
	/// tree.insert(&3);
	/// assert_eq!(tree.last(), &3);
	/// ```
	
	#[inline]
	pub fn last(&self) -> &T {
		self.top.max().get_key()
	}
	
	/// *English*: Method **iter()** converts tree to *iterator*,
	/// which collects *elements in sorted order*,
	/// It takes tree by *immutable reference*, so it'll be *no ownership*.
	/// Check **iter.rs** for *TreeIter*.
	///
	/// *Russian*: Метод **iter()** превращает дерево в *итератор*,
	/// который хранит *элементы по-возрастанию*.
	/// Берётся *неизменяемая ссылка*, так что *владения нет*.
	/// Изучите **iter.rs** для полного понимания.
	///
	/// # Example
	///
	/// ```
	/// use binartree::tree::BinaryTree;
	///
	/// let mut tree = BinaryTree::new();
	/// tree.insert(&1);
	/// tree.insert(&2);
	/// tree.insert(&3);
	///
	/// assert_eq!(tree.iter().collect::<Vec<i32>>(), vec![1, 2, 3]);
	/// ```
	
	#[inline]
	pub fn iter(&self) -> TreeIter<T> {
		TreeIter {
			iter: self.top.walk()
		}
	}
	
	/// *English*: Method **apend()** translates all elements
	/// *from 2-nd tree to 1-st*. All trees are taking by *immutable reference*,
	/// no ownership.
	///
	/// *Russian*: Метод **append()** передаёт элементы *из 2 дерева
	/// в 1*. Используемое дерево остаётся *неизменным*,
	/// так что его *можно использовать повторно*.
	///
	/// # Example
	///
	/// ```
	/// use binartree::tree::BinaryTree;
	///
	/// let mut tree1 = BinaryTree::new();
	/// let mut tree2 = BinaryTree::new();
	///
	/// tree1.insert(&1);
	/// tree2.insert(&2);
	/// tree2.insert(&3);
	///
	/// tree1.append(&tree2);
	///
	/// assert_eq!(tree1.to_vec(), vec![1, 2, 3]);
	/// ```
	
	pub fn append(&mut self, src: &Self) {
		let collect = src.iter().iter;
		for elem in collect.iter() {
			self.insert(elem);
		}
	}
	
	/// *English*: Method **celan()** makes tree empty.
	///
	/// *Russian*: Метод **clean()** полностью очищает дерево.
	///
	/// # Example
	///
	/// ```
	/// use binartree::tree::BinaryTree;
	/// use std::iter::FromIterator;
	///
	/// let mut tree = BinaryTree::from_iter((1..11));
	/// assert_eq!(tree.to_vec(), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
	///
	/// tree.clear();
	/// assert_eq!(tree.to_vec(), vec![]);
	/// ```
	
	pub fn clear(&mut self) {
		self.top.rec_drop();
		self.size = 0;
	}
	
	/// *English*: *Removing element from tree*.
	/// Keys, that are in subnodes are *still in tree*.
	/// **USE THIS METHOD ONLY IF YOU WANT TO REMOVE
	/// ONE ELEMENT FROM TREE. USE *MULTI_REMOVE()* IF
	/// YOU WISH TO REMOVE MORE THAN 1 ELEM. IT'S A LOT MORE FASTER**
	///
	/// *Russian*: *Удаление элемента из дерева*.
	/// Ключи, которые есть в подузлах искомого узла
	/// *остаются в нашем дереве*.
	/// **ИСПОЛЬЗУЙТЕ ЭТОД МЕТОД ТОЛЬК ЕСЛИ ХОТИТЕ
	/// УДАЛИТЬ 1 ЭЛЕМЕНТ ИЗ ДЕРЕВА. ЛУЧШЕ ИСПОЛЬЗОВАТЬ
	/// *MULTI_ERMOVE()*, КОТОРЫЙ РАБОТАЕТ В РАЗЫ БЫСТРЕЕ**
	///
	/// # Example
	///
	/// ```
	/// use binartree::tree::BinaryTree;
	/// use std::iter::FromIterator;
	///
	/// let mut tree = BinaryTree::from_iter((1..11));
	/// assert_eq!(tree.to_vec(), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
	///
	/// tree.remove(&5);
	/// tree.remove(&1);
	/// assert_eq!(tree.to_vec(), vec![2, 3, 4, 6, 7, 8, 9, 10]);
	/// ```
	
	pub fn remove(&mut self, val: &T) {
		let keys = self.top.remove(val);
		if keys.0 {
			self.size -= keys.1.len() + 1;
			self.extend(keys.1);
		}
	}
	
	/// *English*: Method **difference()** returns TreeIter<T>, which contains all elements,
	/// that are *in 1-st tree, but not in 2-nd*,
	///
	/// *Russian*: Метод **difference()** возвращает *"деревянный итератор"*, который
	/// хранит все те элементы, *которые есть в 1 дереве, но которых нет во 2*.
	///
	///  # Example
	///
	/// ```
	/// use binartree::tree::BinaryTree;
	/// use std::iter::FromIterator;
	///
	/// let tree_1 = BinaryTree::from_iter((1..4));
	/// let tree_2 = BinaryTree::from_iter((3..4));
	///
	/// assert_eq!(tree_1.difference(&tree_2).collect::<Vec<i32>>(), vec![1, 2]);
	/// ```
	
	pub fn difference(&self, other: &Self) -> TreeIter<T> {
		let iter_1 = self.to_vec();
		let iter_2 = other.to_vec();
		
		let mut iter = vec![];
		for elem in iter_1 {
			if let Err(_) = iter_2.binary_search(&elem) {
				iter.push(elem);
			}
		}
		
		let mut it = TreeIter::new();
		it.extend(iter);
		it
	}
	
	/// *English*: Method **drain_filter()** *stoles all values* from tree,
	/// which are *match the simple or lambda function*.
	/// Returns *iterator with all removed values*.
	///
	/// *Russian*: Метод **drain_filter()** *крадёт все элементы* из дерева,
	/// которые *соответстуют той функции или замыканию*.
	/// Возвращает *итератор с удалёнными значениями*.
	///
	/// # Example
	/// ```
	///
	/// use binartree::tree::BinaryTree;
	/// use std::iter::FromIterator;
	///
	/// let mut tree1 = BinaryTree::from_iter(1..11);
	/// let tree2 = tree1.drain_filter(|x| x % 2 == 0);
	///
	/// assert_eq!(tree1.iter().collect::<Vec<i32>>(), vec![1, 3, 5, 7, 9]);
	/// assert_eq!(tree2.collect::<Vec<i32>>(), vec![2, 4, 6, 8, 10]);
	/// ```
	
	pub fn drain_filter<F: FnMut(&T) -> bool>(&mut self, mut fun: F) -> TreeIter<T> {
		let mut new_tree_iter = TreeIter::new();
		let iter = self.iter();
		
		for elem in iter {
			if fun(&elem) {
				new_tree_iter.iter.push_back(elem.clone());
			}
		}
		
		let new_vec = Vec::from(new_tree_iter.clone().iter);
		let mut old_new_vec = vec![];
		
		for elem in self.to_vec() {
			if let Err(_) = new_vec.binary_search(&elem) {
				old_new_vec.push(elem);
			}
		}
		self.clear();
		self.extend(old_new_vec);
		
		new_tree_iter
	}
	
	/// *English*: Method **intersection()** returns TreeIter<T>, which contains
	/// all elements, that *are in 1-st and 2-nd tree*
	///
	/// *Russian*: Метод **intersection()** возвращает *"деревянный итератор"*, который
	/// хранит все те элементы, которые *есть и в 1, и во 2 дереве*
	///
	///  # Example
	///
	/// ```
	/// use binartree::tree::BinaryTree;
	/// use std::iter::FromIterator;
	///
	/// let tree_1 = BinaryTree::from_iter((1..10));
	/// let tree_2 = BinaryTree::from_iter((1..15));
	///
	/// assert_eq!(tree_1.intersection(&tree_2).collect::<Vec<i32>>(), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
	/// ```
	
	pub fn intersection(&self, other: &Self) -> TreeIter<T> {
		let iter_1 = self.to_vec();
		let iter_2 = other.to_vec();
		
		let mut iter = vec![];
		for elem in iter_1 {
			if let Ok(_) = iter_2.binary_search(&elem) {
				iter.push(elem);
			}
		}
		
		let mut it = TreeIter::new();
		it.extend(iter);
		it
	}
	
	/// *English*: Method **is_disjoint()** answers the question
	/// *"Are any elements in common?"*
	/// If yes than *false* else *true*
	///
	/// *Russian*: Метод **is_disjoint()** отвечает на вопрос
	/// *"есть ли у деревьев хотя бы 1 общий элемент?"*
	/// Если да, то *false*, иначе *true*
	///
	///  # Example
	///
	/// ```
	/// use binartree::tree::BinaryTree;
	/// use std::iter::FromIterator;
	///
	/// let tree_1 = BinaryTree::from_iter((1..10));
	/// let tree_2 = BinaryTree::from_iter((10..15));
	/// assert_eq!(tree_1.is_disjoint(&tree_2), true);
	///
	/// let tree_3 = BinaryTree::from_iter((1..10));
	/// let tree_4 = BinaryTree::from_iter((1..15));
	/// assert_eq!(tree_3.is_disjoint(&tree_4), false);
	/// ```
	
	pub fn is_disjoint(&self, other: &Self) -> bool {
		self.intersection(other).collect::<Vec<T>>().is_empty()
	}
	
	/// *English*: Method **pop_frist** *removes min value from tree*.
	/// If tree is empty, it'll *panic*
	///
	/// *Russian*: Метод **pop_first()** *удаляет наименьший элемент дерева*.
	/// Т.к. дерево может быть пустым, то будет вызвана *паника*
	///
	/// # Example
	///
	/// ```
	/// use binartree::tree::BinaryTree;
	/// use std::iter::FromIterator;
	/// use std::convert::TryInto;
	///
	/// let mut tree = BinaryTree::from_iter((1..6));
	/// tree.pop_first();
	/// tree.pop_first();
	/// assert_eq!(tree.to_vec(), vec![3, 4, 5]);
	/// ```
	
	pub fn pop_first(&mut self) {
		let key = self.first().clone();
		self.remove(&key);
	}
	
	/// *English*: Method *pop_last()* removes *min element from tree*.
	/// If tree is empty, it'll *panic*.
	///
	/// *Russian*: Метод *pop_last()* удаляет *наибольший элемент дерева*.
	/// Т.к. дерево может быть пустым, то вызывается *паника*
	///
	/// # Example
	///
	/// ```
	/// use binartree::tree::BinaryTree;
	/// use std::iter::FromIterator;
	/// use std::convert::TryInto;
	///
	/// let mut tree = BinaryTree::from_iter((1..6));
	///
	/// tree.pop_last();
	/// tree.pop_last();
	/// assert_eq!(tree.to_vec(), vec![1, 2, 3]);
	/// ```
	
	pub fn pop_last(&mut self) {
		let key = self.last().clone();
		self.remove(&key);
	}
	
	/// *English*: Method **replace_val()** changes all keys with
	/// *some* value to *another* value.
	///
	/// *Russian*: Метод **replace_val()** заменяет все ключи
	/// с *одним* значением на ключи с *другим* значением.
	///
	/// # Example
	///
	/// ```
	/// use binartree::tree::BinaryTree;
	///
	/// let mut tree = BinaryTree::new();
	/// tree.extend(vec![1, 1, 1]);
	///
	/// tree.replace_val(&2, &2);
	/// assert_eq!(tree.to_vec(), vec![1, 1, 1]);
	///
	/// tree.replace_val(&1, &3);
	/// assert_eq!(tree.to_vec(), vec![3, 3, 3]);
	/// ```
	
	pub fn replace_val(&mut self, old_val: &T, new_val: &T) {
		if old_val == new_val{
			return;
		}
		
		let mut count: usize = 0;
		let mut new_tree = vec![];
		
		for elem in self.to_vec() {
			if elem == *old_val {
				count += 1;
			} else {
				new_tree.push(elem);
			}
		}
		
		new_tree.extend(vec![new_val; count]);
		
		self.clear();
		self.extend(new_tree);
	}
	
	/// *English*: Method **symmetric_difference()** returns *TreeIter<T>* with keys,
	/// that are *only in 1-st or 2-nd tree*
	/// Elements in iterator are **not in sorted order**.
	///
	/// *Russian*: Метод **symmetric_difference()** возвращает *"деревянный" итератор*
	/// с ключами, которые есть *либо только в 1 дереве, либо только во 2*.
	/// Метод возвращает **неотсортированный итератор**.
	///
	/// # Example
	/// ```
	/// use binartree::tree::BinaryTree;
	/// use std::iter::FromIterator;
	///
	/// let tree1 = BinaryTree::from_iter((5..16));
	/// let tree2 = BinaryTree::from_iter((1..11));
	///
	/// assert_eq!(tree1.symmetric_difference(&tree2).collect::<Vec<i32>>(), vec![11, 12, 13, 14, 15, 1, 2, 3, 4]);
	/// ```
	
	pub fn symmetric_difference(&self, other: &Self) -> TreeIter<T> {
		let iter_1 = self.to_vec();
		let iter_2 = other.to_vec();
		
		let mut iter = vec![];
		
		for elem in &iter_1 {
			if let Err(_) = iter_2.binary_search(&elem) {
				iter.push(elem.clone());
			}
		}
		
		for elem in iter_2 {
			if let Err(_) = iter_1.binary_search(&elem) {
				iter.push(elem);
			}
		}
		
		let mut it = TreeIter::new();
		it.extend(iter);
		it
	}
	
	/// *English*: Method **union()** creates iterator which contains
	/// *elements from 1-st and 2-nd trees*.
	/// Elements in iterator are **not in sorted order**.
	///
	/// *Russian*: Метод **гтшщт()** создаёт итереатор, содержащий
	/// *элементы из 1 и 2 деревьев*.
	/// Метод возвращает **неотсортированный итератор**.
	///
	/// # Example
	///
	/// ```
	/// use binartree::tree::BinaryTree;
	/// use std::iter::FromIterator;
	///
	/// let tree1 = BinaryTree::from_iter(1..15);
	/// let tree2 = BinaryTree::from_iter(5..20);
	///
	/// assert_eq!(tree1.union(&tree2).collect::<BinaryTree<i32>>().to_vec(), (1..20).collect::<Vec<i32>>());
	/// ```
	
	pub fn union(&self, other: &Self) -> TreeIter<T> {
		let mut iter = TreeIter::new();
		iter.extend(self.symmetric_difference(other));
		iter.extend(self.intersection(other));
		iter
	}
	
	/// *English*: You should use method **multi_remove()**
	/// when you want to *remove more than one value from tree*.
	/// It's **a lot more faster** then removing elem-by-elem.
	/// It takes ownership, so you need to clone it, if you want to use src twice.
	///
	/// *Russian*: Метод **multi_remove()** нужен для *быстрого удаления сразу нескольких элементов*.
	/// Работает **ГОРАЗДО быстрее**, чем поэлементное удаление.
	/// Метод принимает владение значениями, так что нужно копировать вектор с ресурсами,
	/// если хотите использовать его дважды.
	///
	/// # Example
	///
	/// ```
	/// use binartree::tree::BinaryTree;
	/// use std::iter::FromIterator;
	///
	/// let mut tree = BinaryTree::from_iter(1..10);
	/// assert_eq!(tree.to_vec(), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
	///
	/// tree.multi_remove(vec![1, 2, 3, 4, 5]);
	/// assert_eq!(tree.to_vec(), vec![6, 7, 8, 9]);
	/// ```
	
	pub fn multi_remove(&mut self, mut src: Vec<T>) {
		let mut new_tree = vec![];
		let source = self.to_vec();
		
		for i in 0..source.len() {
			if let Some(ind) = find_vec(&src, &source[i]) {
				src.swap_remove(ind);
			} else {
				new_tree.push(source[i].clone());
			}
		}
		
		self.clear();
		self.extend(new_tree);
	}
}

/// *English*: Search function in vector.
///
/// *Russian*: Функция поиска в векторе.

fn find_vec<T>(src: &Vec<T>, val: &T) -> Option<usize>
	where T: Copy + Clone + Ord + Eq
{
	for i  in 0..src.len() {
		if src[i] == *val {
			return Some(i)
		}
	}
	None
}

/// *English*: Trait **Iterator** for tree. Now we can iterate in our tree.
/// The role of iterator lies on *TreeIter<T>* (iter.rs)
///
/// Once we add realisation of all required methods,
/// we can use all methods from **Iterator**
///
/// *Russian*: Добавляем дереву трейт **IntoIterator**.
/// Теперь мы можем итерироваться (ходить) по дереву.
/// В качестве итератора используем свою структуру *TreeIter<T>* (iter.rs)
///
/// Т.к. мы добавили трейт и реализовали все необходимые методы
/// для точной имплементации трейта, то можем юзать и другие методы
/// из трейта **Iterator**
///
/// # Example
///
/// ```
/// use binartree::tree::BinaryTree;
///
/// let mut tree = BinaryTree::new();
///
/// tree.insert(&1);
/// tree.insert(&2);
/// tree.insert(&3);
///
/// let test1 = tree.clone().into_iter().collect::<Vec<i32>>();
/// let test2 = tree.clone().into_iter().min().unwrap();
/// let test3 = tree.clone().into_iter().sum::<i32>();
///
/// assert_eq!(test1, vec![1, 2, 3]);
/// assert_eq!(test2, 1);
/// assert_eq!(test3, 6);
/// ```
///
impl<T> IntoIterator for BinaryTree<T>
	where T: Copy + Clone + Ord + Eq
{
	type Item = T;
	type IntoIter = TreeIter<T>;
	
	/// *English*: Method *into_iter()* converts tree to *TreeIter<T>*
	///
	/// *Russian*: Метод into_iter() превращает дерево в *TreeIter<T>*
	///
	/// # Example
	///
	/// ```
	/// use binartree::tree::BinaryTree;
	///
	/// let mut tree1 = BinaryTree::new();
	/// let mut tree2 = BinaryTree::new();
	///
	/// tree1.insert(&1);
	/// tree2.insert(&2);
	/// tree2.insert(&3);
	/// tree1.append(&tree2);
	///
	/// assert_eq!(tree1.to_vec(), vec![1, 2, 3]);
	/// ```
	
	fn into_iter(self) -> TreeIter<T> {
		self.iter()
	}
}

/// *English*: **Extend<T>** trait for tree.
/// Now we can add values from structs with iterators.
///
/// *Russian*: Добавление трейта **Extend<T>** для дерева.
/// Теперь мы можем добавлять элементы 
/// в дерево из других итерируемых значений.
///
/// # Example
///
/// ```
/// use binartree::tree::BinaryTree;
///
/// let mut tree = BinaryTree::new();
/// tree.extend(vec![1, 2, 3, 4]);
/// assert_eq!(tree.to_vec(), vec![1, 2, 3, 4]);
/// ```

impl<T> Extend<T> for BinaryTree<T>
	where T: Copy + Clone + Ord + Eq
{
	/// *English*: Method **extend()** *stoles keys from value*.
	/// It *takes ownership* of src, so if you want to continue
	/// using it, copy.
	///
	/// *Russian*: Метод **extend()** *крадёт ключи из итерируемого значения*.
	/// Метод *принимает владение* ресурсами, так что копируёте,
	/// если хотите повторно использовать значение.
	
	fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
		for it in iter {
			self.insert(&it);
		}
	}
}

/// *English*: **FromIterator<T>** trait for tree.
/// Now we can build tree from iterators
///
/// *Russian*: Добавление трейта **FromIterator<T>** для дерева.
/// Теперь мы можем построить дерево из итераторов
///
/// # Example
///
/// ```
/// use std::iter::FromIterator;
/// use binartree::tree::BinaryTree;
///
/// let range = (0..11).step_by(2);
/// let mut new_tree = BinaryTree::from_iter(range);
///
/// assert_eq!(new_tree.to_vec(), vec![0, 2, 4, 6, 8, 10]);
/// ```

impl<T> FromIterator<T> for BinaryTree<T>
	where T: Copy + Clone + Ord + Eq
{
	fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
		let mut tree = BinaryTree::new();
		tree.extend(iter);
		return tree;
	}
}

/// *English*: **BitAnd** trait implementation.
/// Creates *new Tree from 1-st and 2-nd trees*.
///
/// *Russian*: Трейт **BitAnd** позволяет создать
/// *дерево из 2-х других*.

impl<T> BitAnd for &BinaryTree<T>
	where T: Copy + Clone + Ord + Eq
{
	/// *English*: Return BinaryTree<T>
	///
	/// *Russian*: Возвращает BinaryTree<T>
	
	type Output = BinaryTree<T>;
	
	/// *English*: Creates new binary tree from 2 trees
	///
	/// *Russian*: Создаёт дерево из 2-х других
	///
	/// # Example
	///
	/// ```
	///  use binartree::tree::BinaryTree;
	/// use std::iter::FromIterator;
	///
	/// let tree1 = BinaryTree::from_iter(1..20);
	/// let tree2 = BinaryTree::from_iter(10..20);
	///
	/// assert_eq!((&tree1 & &tree2).to_vec(), (10..20).collect::<Vec<i32>>());
	/// ```
	
	fn bitand(self, rhs: Self) -> BinaryTree<T> {
		BinaryTree::from_iter(self.intersection(rhs))
	}
}

/// *English*: **BitOr** trait for tree. Creates tree,
/// which contains *elements from 1-st and 2-nd trees*.
/// Return BinaryTree<T>
///
/// *Russian*: **BitOr** Трейт для дерева. Создаёт
/// дерево *из элементов 1 и 2 дерева (пересечение)*.
/// Возвращает BinaryTree<T>

impl<T> BitOr for &BinaryTree<T>
	where T: Copy + Clone + Ord + Eq
{
	
	/// *English*: Return BinaryTree<T>
	///
	/// *Russian*: Возвращает BinaryTree<T>
	
	type Output = BinaryTree<T>;
	
	/// *English*: Creates new binary tree from 2 trees
	///
	/// *Russian*: Создаёт дерево из 2-х других
	///
	/// # Example
	///
	/// ```
	///  use binartree::tree::BinaryTree;
	/// use std::iter::FromIterator;
	///
	/// let tree1 = BinaryTree::from_iter(1..10);
	/// let tree2 = BinaryTree::from_iter(10..20);
	///
	/// assert_eq!((&tree1 | &tree2).to_vec(), (1..20).collect::<Vec<i32>>());
	/// ```
	
	fn bitor(self, rhs: Self) -> BinaryTree<T> {
		BinaryTree::from_iter(self.union(&rhs))
	}
}

/// *English*: **BitOr** trait for tree. Creates tree,
/// which contains *elements, which are only 1-st and only in 2-nd trees*.
/// Return BinaryTree<T>
///
/// *Russian*: **BitOr** Трейт для дерева. Создаёт
/// дерево *из элементов которые входят только в 1 и в 2 дерева*.
/// Возвращает BinaryTree<T>

impl<T> BitXor for &BinaryTree<T>
	where T: Copy + Clone + Ord + Eq
{
	/// *English*: Return BinaryTree<T>
	///
	/// *Russian*: Возвращает BinaryTree<T>
	
	type Output = BinaryTree<T>;
	
	/// *English*: Creates new binary tree from 2 trees
	///
	/// *Russian*: Создаёт дерево из 2-х других
	///
	/// # Example
	///
	/// ```
	///  use binartree::tree::BinaryTree;
	/// use std::iter::FromIterator;
	///
	/// let tree1 = BinaryTree::from_iter(1..15);
	/// let tree2 = BinaryTree::from_iter(10..20);
	///
	/// assert_eq!((&tree1 ^ &tree2).to_vec(), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 15, 16, 17, 18, 19]);
	/// ```
	
	fn bitxor(self, rhs: Self) -> BinaryTree<T> {
		BinaryTree::from_iter(self.symmetric_difference(&rhs))
	}
}
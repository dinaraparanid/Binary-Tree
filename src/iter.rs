use std::collections::VecDeque;
use std::iter::FromIterator;
use std::ops::RangeBounds;
use std::collections::vec_deque::Drain;

/// **Realisation of Iterator for Tree**
/// ---------------------------------------

/// *English*: Our Iterator contains elements in *std::collections::VecDeque*.
/// It's faster than *std::collections::LinkedList*, and it can push and
/// delete elements from begin and end.
///
/// *Russian*: Наш итератор хранит элементы в *std::collections::VecDeque*.
/// Дек шустрее *std::collections::LinkedList*, а так же умеет удалять
/// элементы как с конца, так и с начала.

#[derive(Debug, Clone, PartialEq)]
pub struct TreeIter<T>
	where T: Copy + Clone + Ord + Eq
{
	pub(crate)iter: VecDeque<T>
}

/// *English*: **Default** trait for iter. Default iterator is empty.
///
/// *Russian*: Добавляем трейт **Default** для реализации дефолтного
/// итератора. По-умолчанию наш итератор - пустой дек.

impl<T> Default for TreeIter<T>
	where T: Copy + Clone + Ord + Eq
{
	
	/// *Russian*: Создаём пустой итератор
	///
	/// *English*: Creates an empty iterator.
	///
	/// # Example
	///
	/// ```
	/// use binartree::iter::TreeIter;
	///
	/// let def_iter = TreeIter::<i32>::default();
	/// assert_eq!(def_iter.into_iter(), TreeIter::new());
	/// ```
	
	#[inline]
	fn default() -> Self {
		TreeIter { iter: VecDeque::new() }
	}
}


/// Realisation of methods for Iterator
/// ---------------------------------------

impl<T> TreeIter<T>
	where T: Copy + Clone + Ord + Eq
{
	
	/// *English*: Method **new()** creates *empty iterator*
	///
	/// *Russian*: Метод **new()** создаёт пустой итератор
	///
	/// # Example
	///
	/// ```
	/// use std::collections::VecDeque;
	/// use binartree::iter::TreeIter;
	///
	/// let new_iter = TreeIter::new();
	/// assert_eq!(new_iter.collect::<VecDeque<i32>>(), VecDeque::<i32>::new());
	/// ```
	
	#[inline]
	pub fn new() -> Self {
		TreeIter { iter: VecDeque::new() }
	}
	
	/// *English*: Method **with_capacity()** returns iterator with some capacity
	///
	/// *Russian*: Метод **with_capacity()** возвращает итератор с определённой ёмкостью
	///
	/// # Example
	///
	/// ```
	/// use binartree::iter::TreeIter;                // can contain 3 elements
	/// let iter = TreeIter::<i32>::with_capacity(3); // without reallocation
	/// assert_eq!(iter.capacity(), 3);
	/// ```
	
	#[inline]
	pub fn with_capacity(capacity: usize) -> Self {
		TreeIter { iter: VecDeque::with_capacity(capacity) }
	}
	
	/// *English*: returns iterator's capacity
	///
	/// *Russian*: возвращает ёмкость итератора
	///
	/// # Example
	///
	/// ```
	/// use binartree::iter::TreeIter;                // can contain 3 elements
	/// let iter = TreeIter::<i32>::with_capacity(3); // without reallocation
	/// assert_eq!(iter.capacity(), 3);
	/// ```
	
	#[inline]
	pub fn capacity(&self) -> usize {
		self.iter.capacity()
	}
	
	/// *English*: Method **len()** returns *iterator's len*
	///
	/// *Russian*: Метод **len()** возвращает длину итератора
	///
	/// # Example
	///
	/// ```
	/// use binartree::iter::TreeIter;
	/// use std::iter::FromIterator;
	///
	/// let iter = TreeIter::from_iter((1..11));
	/// assert_eq!(iter.len(), 10);
	/// ```
	
	#[inline]
	pub fn len(&self) -> usize {
		self.iter.len()
	}
	
	/// *English*: Method **is_empty()** answers the question: "is out iterator empty?"
	/// If it's true, returns *true*, else *false*.
	///
	/// *Rusiian*: Метод **is_empty()** отвечает на вопрос: "пуст ли итератор?"
	/// Если пусто, то возвращает *true*, иначе *false*.
	///
	/// # Example
	///
	/// ```
	/// use binartree::iter::TreeIter;
	/// use std::iter::FromIterator;
	///
	/// let not_empty_iter = TreeIter::from_iter(1..10);
	/// assert_eq!(not_empty_iter.is_empty(), false);
	///
	/// let empty_iter = TreeIter::<i32>::new();
	/// assert_eq!(empty_iter.is_empty(), true);
	/// ```
	
	#[inline]
	pub fn is_empty(&self) -> bool {
		self.iter.is_empty()
	}
	
	/// *English*: Method **append()** adds another iterator
	/// to our iterator. *Takes ownership*
	///
	/// *Russian*: Метод **append()** добавляет итератор
	/// в наш итератор. *Принимает владение*
	///
	/// # Example
	///
	/// ```
	/// use binartree::iter::TreeIter;
	/// use std::iter::FromIterator;
	///
	/// let mut iter1 = TreeIter::from_iter(1..15);
	/// let iter2 = TreeIter::from_iter(5..20);
	/// iter1.append(iter2);
	///
	/// let mut check = (1..15).collect::<Vec<i32>>();
	/// check.extend(5..20);
	///
	/// assert_eq!(iter1.collect::<Vec<i32>>(), check);
	/// ```
	
	#[inline]
	pub fn append(&mut self, src: Self) {
		self.iter.extend(src)
	}
	
	/// *English*: Method **clear()** cleans all iterator
	///
	/// *Russian* Метод **clear()** очищает всесь итератор
	///
	/// # Example
	///
	/// ```
	/// use binartree::iter::TreeIter;
	/// use std::iter::FromIterator;
	///
	/// let mut iter = TreeIter::from_iter(1..10);
	/// iter.clear();
	/// assert_eq!(iter.collect::<Vec<i32>>(), vec![]);
	/// ```
	
	#[inline]
	pub fn clear(&mut self) {
		self.iter.clear()
	}
	
	/// *English*: full clear of repeated elements.
	/// Makes Iterator sorted.
	///
	/// *Russian*: полностью очищает итератор от повторов.
	/// Возвращает отсортированный итератор.
	///
	/// # Example
	///
	/// ```
	/// use binartree::iter::TreeIter;
	///
	/// let mut iter = TreeIter::new();
	/// iter.extend(vec![1, 2, 3, 1, 2, 3]);
	/// iter.full_dedup();
	/// assert_eq!(iter.collect::<Vec<i32>>(), vec![1, 2, 3]);
	/// ```
	
	pub fn full_dedup(&mut self) {
		let mut vec = Vec::from(self.iter.clone());
		vec.sort(); vec.dedup();
		self.clear(); self.extend(vec);
	}
	
	/// *English*: Creates a draining iterator with removed
	/// elements from iterator. Takes elements from start's
	/// index to end's index.
	/// Panics if the starting point is greater than the end point 
	/// or if the end point is greater than the length of the iterator.
	///
	/// *Russian*: Удаляет с start-ого по finish-ный элементы.
	/// Паникует, если начало больше конца или конец больше длины.
	/// Возвращает итератор с удалёнными элементами.
	///
	/// # Example
	///
	/// ```
	/// use binartree::iter::TreeIter;
	/// use std::iter::FromIterator;
	///
	/// let mut iter = TreeIter::with_capacity(6);
	/// iter.extend((0..11).step_by(2));
	/// let remove = iter.drain(1..5);
	///
	/// assert_eq!(remove.collect::<Vec<i32>>(), (2..=8).step_by(2).collect::<Vec<i32>>());
	/// ```
	
	#[inline]
	pub fn drain<R> (&mut self, range: R) -> Drain<T>
		where R: RangeBounds<usize>
	{
		self.iter.drain(range)
	}
	
	/// *English*: Method **drain_filter()** removes all elements that returns *true* with some function
	///
	/// *Russian*: Метод **drain_filter()** крадёт все элементы, удоволетворяющие функции
	///
	/// # Example
	///
	/// ```
	/// use binartree::iter::TreeIter;
	/// use std::iter::FromIterator;
	///
	/// let mut iter = TreeIter ::from_iter(0..10);
	/// let iter2 = iter.drain_filter(|x| *x % 2 == 0);
	///
	/// assert_eq!(iter.collect::<Vec<i32>>(), vec![1, 3, 5, 7, 9]);
	/// assert_eq!(iter2.collect::<Vec<i32>>(), vec![0, 2, 4, 6, 8]);
	/// ```
	
	pub fn drain_filter<F: FnMut(&T) -> bool>(&mut self, mut filter: F) -> Self {
		let mut remove_it = Vec::with_capacity(self.len());
		let iter = self.iter.clone();
		
		for elem in iter {
			if filter(&elem) {
				remove_it.push(elem.clone());
			}
		}
		
		let mut old_vec = Vec::with_capacity(self.len());
		
		for elem in &self.iter {
			if let Err(_) = remove_it.binary_search(elem) {
				old_vec.push(elem.clone());
			}
		}
		
		self.clear();
		self.extend(old_vec);
		
		remove_it.shrink_to_fit();
		let mut rem = TreeIter::with_capacity(remove_it.len());
		rem.extend(remove_it);
		rem
	}
	
	/// *English*: Extends all elements from slice to the end of iterator
	///
	/// *Russian*: Добавляет все элементы в конец итератора из среза
	///
	/// # Example
	///
	/// ```
	/// use binartree::iter::TreeIter;
	///
	/// let mut iter = TreeIter::with_capacity(3);
	/// iter.extend_from_slice(&[2, 3, 4]);
	/// assert_eq!(iter.collect::<Vec<i32>>(), vec![2, 3, 4]);
	/// ```
	
	#[inline]
	pub fn extend_from_slice(&mut self, slice: &[T]) {
		let mut vec = Vec::with_capacity(slice.len());
		vec.extend_from_slice(slice);
		self.extend(vec);
	}
	
	/// *English*: *Inserts an element at position index* within the iterator,
	/// shifting all elements after it to the right.
	/// Panics if index > len
	///
	/// *Russian*: *Добавляет значение по-индексу*,
	/// передвигая все элементы после правее.
	/// Паникует если index > len
	///
	/// # Example
	///
	/// ```
	/// use binartree::iter::TreeIter;
	///
	/// let mut iter = TreeIter::new();
	/// iter.insert(0, &1);
	/// iter.insert(0, &2);
	/// iter.insert(0, &3);
	/// assert_eq!(iter.collect::<Vec<i32>>(), vec![3, 2, 1]);
	/// ```
	
	#[inline]
	pub fn insert(&mut self, index: usize, val: &T) {
		self.iter.insert(index, val.clone());
	}
	
	/// *English*: place value to the start
	///
	/// *Russian*: добавляет значение в начало
	///
	/// # Example
	///
	/// ```
	/// use binartree::iter::TreeIter;
	/// use std::iter::FromIterator;
	///
	/// let mut iter = TreeIter::from_iter(1..5);
	/// iter.push_front(&0);
	///
	/// assert_eq!(iter.collect::<Vec<i32>>(), vec![0, 1, 2, 3, 4]);
	/// ```
	
	#[inline]
	pub fn push_front(&mut self, val: &T) {
		self.iter.push_front(val.clone());
	}
	
	/// *English*: removes value from the start
	///
	/// *Russian*: удаляет значение из начала
	///
	/// # Example
	///
	/// ```
	/// use binartree::iter::TreeIter;
	/// use std::iter::FromIterator;
	///
	/// let mut iter = TreeIter::from_iter(1..5);
	/// iter.pop_front();
	///
	/// assert_eq!(iter.collect::<Vec<i32>>(), vec![2, 3, 4]);
	/// ```
	
	#[inline]
	pub fn pop_front(&mut self) -> Option<T> {
		self.iter.pop_front()
	}
	
	/// *English*: place value to the start
	///
	/// *Russian*: добавляет значение в начало
	///
	/// # Example
	///
	/// ```
	/// use binartree::iter::TreeIter;
	/// use std::iter::FromIterator;
	///
	/// let mut iter = TreeIter::from_iter(1..5);
	/// iter.push_back(&5);
	///
	/// assert_eq!(iter.collect::<Vec<i32>>(), vec![1, 2, 3, 4, 5]);
	/// ```
	
	#[inline]
	pub fn push_back(&mut self, val: &T) {
		self.iter.push_back(val.clone());
	}
	
	/// *English*: removes value from the start
	///
	/// *Russian*: удаляет значение из начала
	///
	/// # Example
	///
	/// ```
	/// use binartree::iter::TreeIter;
	/// use std::iter::FromIterator;
	///
	/// let mut iter = TreeIter::from_iter(1..5);
	/// iter.pop_back();
	///
	/// assert_eq!(iter.collect::<Vec<i32>>(), vec![1, 2, 3]);
	/// ```
	
	#[inline]
	pub fn pop_back(&mut self) -> Option<T> {
		self.iter.pop_back()
	}
	
	/// *English*: *Removes an element at position index* within the iterator,
	/// shifting all elements after it to the left.
	/// Panics if index > len
	///
	/// *Russian*: *Удаляет значение по-индексу*,
	/// передвигая все элементы после левее.
	/// Паникует если index > len
	///
	/// # Example
	///
	/// ```
	/// use binartree::iter::TreeIter;
	///
	/// let mut iter = TreeIter::new();
	/// iter.insert(0, &1);
	/// iter.insert(0, &2);
	/// iter.insert(0, &3);
	/// assert_eq!(iter.collect::<Vec<i32>>(), vec![3, 2, 1]);
	/// ```
	
	#[inline]
	pub fn remove(&mut self, index: usize) -> Option<T> {
		self.iter.remove(index)
	}
	
	/// *English*: Adds memory space to the iterator
	///
	/// *Russian*: Увеличивает ёмкость итератора
	///
	/// # Example
	///
	/// ```
	/// use binartree::iter::TreeIter;
	///
	/// let mut iter = TreeIter::<i32>::with_capacity(5);
	/// iter.reserve(10);
	/// assert!(iter.capacity() >= 15);
	/// ```
	
	#[inline]
	pub fn reserve(&mut self, reserve: usize) {
	   self.iter.reserve(reserve)
	}
	
	/// *English*: Method **retain()** removes all elements that returns *false* with some function
	///
	/// *Russian*: Метод **retain** крадёт все элементы, неудоволетворяющие функции
	///
	/// # Example
	///
	/// ```
	/// use binartree::iter::TreeIter;
	/// use std::iter::FromIterator;
	///
	/// let mut iter = TreeIter ::from_iter(0..10);
	/// iter.retain(|x| *x % 2 == 0);
	///
	/// assert_eq!(iter.collect::<Vec<i32>>(), vec![0, 2, 4, 6, 8]);
	/// ```
	
	pub fn retain<F: FnMut(&T) -> bool>(&mut self, fun: F) {
		let rem = self.drain_filter(fun);
		self.clear(); self.extend(rem);
	}
	
	/// *English*: removes unused memory space
	///
	/// *Russian*: убирает неиспользуемую область памяти под итератор
	///
	/// # Example
	///
	/// ```
	/// use binartree::iter::TreeIter;
	///
	/// let mut iter = TreeIter::with_capacity(10);
	///
	/// iter.extend_from_slice(&[1, 2, 3]);
	/// assert!(iter.capacity() >= 10);
	///
	/// iter.shrink_to_fit();
	/// assert!(iter.capacity() >= 3);
	/// ```
	
	#[inline]
	pub fn shrink_to_fit(&mut self) {
		self.iter.shrink_to_fit()
	}
	
	/// *English*: Splits the iterator into two at the given index.
	///
	/// *Russian*: Делит итератор на два по индексу
	///
	/// # Example
	///
	/// ```
	/// use binartree::iter::TreeIter;
	/// use std::iter::FromIterator;
	///
	/// let mut iter1 = TreeIter::from_iter(0..50);
	/// let iter2 = iter1.split_off(25);
	///
	/// assert_eq!(iter1.collect::<Vec<i32>>(), (0..25).collect::<Vec<i32>>());
	/// assert_eq!(iter2.collect::<Vec<i32>>(), (25..50).collect::<Vec<i32>>());
	/// ```
	
	pub fn split_off(&mut self, at: usize) -> Self {
		let vec = self.iter.split_off(at);
		let mut res = TreeIter::with_capacity(vec.len());
		res.extend(vec);
		res
	}
	
	/// *English*: Removes element by index for O(1) by replacing it by last element,
	/// but makes it unsorted.
	/// Panics if index is out of bounds
	///
	/// *Russian*: Удаляет элемент по индексу за O(1) заменяя последний элемент первым,
	/// но меняет порядок элементов.
	/// Паникует, если индекс > длины
	///
	/// # Example
	///
	/// ```
	/// use binartree::iter::TreeIter;
	/// use std::iter::FromIterator;
	///
	/// let mut iter = TreeIter::from_iter(1..5);
	///
	/// assert_eq!(iter.swap_remove_back(0).unwrap(), 1);
	/// assert_eq!(iter.clone().collect::<Vec<i32>>(), vec![4, 2, 3]);
	///
	/// assert_eq!(iter.swap_remove_back(0).unwrap(), 4);
	/// assert_eq!(iter.clone().collect::<Vec<i32>>(), vec![3, 2]);
	///
	/// assert_eq!(iter.swap_remove_back(0).unwrap(), 3);
	/// assert_eq!(iter.clone().collect::<Vec<i32>>(), vec![2]);
	///
	/// assert_eq!(iter.swap_remove_back(0).unwrap(), 2);
	/// assert_eq!(iter.clone().collect::<Vec<i32>>(), vec![]);
	/// ```
	
	#[inline]
	pub fn swap_remove_back(&mut self, at: usize) -> Option<T> {
		self.iter.swap_remove_back(at)
	}
	
	/// *English*: Removes element by index for O(1) by replacing it by last element,
	/// but makes it unsorted.
	/// Panics if index is out of bounds
	///
	/// *Russian*: Удаляет элемент по индексу за O(1) заменяя последний элемент первым,
	/// но меняет порядок элементов.
	/// Паникует, если индекс > длины
	///
	/// # Example
	///
	/// ```
	/// use binartree::iter::TreeIter;
	/// use std::iter::FromIterator;
	///
	/// let mut iter = TreeIter::from_iter(1..5);
	///
	/// assert_eq!(iter.swap_remove_front(iter.len() - 1).unwrap(), 4);
	/// assert_eq!(iter.clone().collect::<Vec<i32>>(), vec![2, 3, 1]);
	///
	/// assert_eq!(iter.swap_remove_front(iter.len() - 1).unwrap(), 1);
	/// assert_eq!(iter.clone().collect::<Vec<i32>>(), vec![3, 2]);
	///
	/// assert_eq!(iter.swap_remove_front(iter.len() - 1).unwrap(), 2);
	/// assert_eq!(iter.clone().collect::<Vec<i32>>(), vec![3]);
	///
	/// assert_eq!(iter.swap_remove_front(iter.len() - 1).unwrap(), 3);
	/// assert_eq!(iter.clone().collect::<Vec<i32>>(), vec![]);
	/// ```
	
	#[inline]
	pub fn swap_remove_front(&mut self, at: usize) -> Option<T> {
		self.iter.swap_remove_front(at)
	}
	
	/// *English*: resizes iterator to *len* size
	///
	/// *Russian*: изменяет размер итератора до заданного
	///
	/// # Example
	///
	/// ```
	/// use binartree::iter::TreeIter;
	/// use std::iter::FromIterator;
	///
	/// let mut iter = TreeIter::from_iter(1..100);
	/// iter.truncate(50);
	/// assert_eq!(iter.collect::<Vec<i32>>(), (1..51).collect::<Vec<i32>>());
	/// ```
	
	#[inline]
	pub fn truncate(&mut self, len: usize) {
		self.iter.truncate(len)
	}
	
	/// *English*: Converts to vector by coping iterator. Doesn't takes ownership
	///
	/// *Russian*: Конвертирует в вектор копируя итератор. Владения нет
	///
	/// # Example
	///
	/// ```
	/// use binartree::iter::TreeIter;
	/// use std::iter::FromIterator;
	///
	/// let iter = TreeIter::from_iter(1..5);
	/// assert_eq!(iter.to_vec(), vec![1, 2, 3, 4]);
	/// ```
	
	#[inline]
	pub fn to_vec(&self) -> Vec<T> {
		self.clone().collect::<Vec<T>>()
	}
	
	/// *English*: Converts to deque by coping iterator. Doesn't takes ownership
	///
	/// *Russian*: Конвертирует в дек копируя итератор. Владения нет
	///
	/// # Example
	///
	/// ```
	/// use binartree::iter::TreeIter;
	/// use std::iter::FromIterator;
	/// use std::collections::VecDeque;
	///
	/// let iter = TreeIter::from_iter(1..5);
	/// assert_eq!(iter.to_deque(), VecDeque::from(vec![1, 2, 3, 4]));
	/// ```
	
	#[inline]
	pub fn to_deque(&self) -> VecDeque<T> {
		self.clone().collect::<VecDeque<T>>()
	}
}

/// *English*: ~~Make our iterator iterator~~ **Iterator** trait
/// for our iterator. Now it can use *all methods* from **Iterator**.
///
/// *Russian**: ~~Делаем наш итератор итератором~~ добавляем
/// реализацию трейта **Iterator** для нашего итератора.
/// Это значит, что наш итератор *имеет те же методы,
/// что и все итераторы*
///
///  # Example
///
/// ```
/// use std::iter::FromIterator;
/// use binartree::tree::BinaryTree;
///
/// let mut tree = BinaryTree::from_iter((1..11).step_by(2));
/// let tree_iter = tree.into_iter();
///
/// assert_eq!(tree_iter.clone().min().unwrap(), 1);
/// assert_eq!(tree_iter.clone().max().unwrap(), 9);
/// assert_eq!(tree_iter.clone().collect::<Vec<i32>>(), vec![1, 3, 5, 7, 9]);
/// assert_eq!(tree_iter.clone().filter(|x| x % 3 == 0).collect::<Vec<i32>>(), vec![3, 9]);
/// ```

impl<T> Iterator for TreeIter<T>
	where T: Copy + Clone + Ord + Eq
{
	
	/// *English*: Type of iterator. Check *tree.rs* for more information.
	///
	/// *Russian*: Тип итератора соответствует типу дерева.
	/// О том, какой тип может находится в дереве подробно
	/// описано в *tree.rs*
	
	type Item = T;
	
	/// *English*: Method **next()** allows to walk in iterator
	/// *elem-by-elem*. If we reached the end, method will return *None*,
	/// else *Some(T)*.
	///
	/// When we work with iterator, we are *clearing* it.
	/// That means that **IT'S IS NOT ALLOWED TO USE IT MORE THEN ONE TIME**
	///
	/// *Russian*: Метод **next()** позволяет передвигаться по
	/// нашему итератору *поэлементно*.
	/// Если мы дошли до конца, то вернётяс *None*,
	/// иначе *Some(T)*.
	///
	/// Здесь мы *опустошаем* наш итератор, так что
	/// **ПОВТОРНОЕ ИСПОЛЬЗОВАНИЕ ОДНОГО И ТОГО ЖЕ
	/// ИТЕРАТОРА ЗАПРЕЩЕНО**.
	///
	/// # Example
	///
	/// ```
	/// use std::iter::FromIterator;
	/// use binartree::tree::BinaryTree;
	///
	/// let mut tree_iter = BinaryTree::from_iter((0..5).step_by(2)).into_iter();
	///
	/// assert_eq!(tree_iter.next(), Some(0));
	/// assert_eq!(tree_iter.next(), Some(2));
	/// assert_eq!(tree_iter.next(), Some(4));
	/// assert_eq!(tree_iter.next(), None);
	/// ```
	
	fn next(&mut self) -> Option<T> {
		return self.iter.pop_front()
	}
}

impl<T> ExactSizeIterator for TreeIter<T>
	where T: Copy + Clone + Ord + Eq
{
	fn len(&self) -> usize {
		self.len()
	}
}

/// *English*: **DoubleEndedIterator** makes out iterator reversed.
/// It means that we can used reverse methods like rfind() and so on.
///
/// *Russian*: Трейт **DoubleEndedIterator** делает наш итератор обратным.
/// Это значит, что мы можем использовать обратные методы как rfind()  другие
///
/// # Example
///
/// ```
/// use binartree::iter::TreeIter;
/// use std::iter::FromIterator;
///
/// let iter = TreeIter::from_iter(1..6);
/// assert_eq!(iter.clone().rfind(|x| *x == 5), Some(5));
/// assert_eq!(iter.clone().rev().collect::<Vec<i32>>(), vec![5, 4, 3, 2, 1]);
/// assert_eq!(iter.clone().rposition(|x| x == 1), Some(0));
/// ```

impl<T> DoubleEndedIterator for TreeIter<T>
	where T: Copy + Clone + Ord + Eq
{
	
	/// *English*: Method **next_back()** returns end of iterator if it's exist
	///
	///*Russian*: Метод **next_back()** возвращает конец итератора, если тот существует
	///
	/// # Example
	///
	/// ```
	/// use binartree::iter::TreeIter;
	/// use std::iter::FromIterator;
	/// 
	/// let mut iter = TreeIter::from_iter(1..5);
	/// assert_eq!(iter.next_back(), Some(4));
	/// assert_eq!(iter.next_back(), Some(3));
	/// assert_eq!(iter.next_back(), Some(2));
	/// assert_eq!(iter.next_back(), Some(1));
	/// assert_eq!(iter.next_back(), None);
	/// ```
	
	fn next_back(&mut self) -> Option<T> {
		return self.iter.pop_back()
	}
}

/// *English*: **Extend()** trait allows us to **move** *iterated value* into tree.
/// Method takes *ownership* of value, so it must be *copied*, if you wish to use it twice.
///
/// *Russian*: Трейт **Extend()** позволяет нам пихать в итератор
/// *итерируемое значение*. Мы принимаем *владение* значением,
/// так что его *следует копировать*, если хотим использовать его повторно.

impl<T> Extend<T> for TreeIter<T>
	where T: Copy + Clone + Ord + Eq
{
	/// *English*: Method **extend()** *moves* value to tree
	///
	/// *Russian*: Метод **extend()** *переносит* значение в дерево
	///
	/// # Example
	///
	/// ```
	/// use binartree::iter::TreeIter;
	///
	/// let mut iter = TreeIter::new();
	/// iter.extend(vec![1, 2, 3]);
	/// assert_eq!(iter.collect::<Vec<i32>>(), vec![1, 2, 3]);
	/// ```
	
	fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
		self.iter.extend(iter);
	}
}

/// *English*: **FromIterator<T>** trait helps us to *build iterator from another iterator*.
///
/// *Russian*: Трейт **FromIterator<T>** позволяет нам *строить итератор из других итераторов*.

impl<T> FromIterator<T> for TreeIter<T>
	where T: Copy + Clone + Ord + Eq
{
	
	/// *English*: construct TreeIter<T> from another iterator
	///
	/// *Russian*: строим итератор из других итераторов
	///
	/// # Example
	/// ```
	/// use binartree::iter::TreeIter;
	/// use std::iter::FromIterator;
	///
	/// let iter = TreeIter::from_iter((1..11));
	/// assert_eq!(iter.collect::<Vec<i32>>(), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
	/// ```
	fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
		let mut it = TreeIter::new();
		it.iter.extend(iter);
		it
	}
}



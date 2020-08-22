use std::collections::VecDeque;
use std::iter::FromIterator;

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
	
	fn default() -> Self {
		return TreeIter { iter: VecDeque::new() };
	}
}


/// Realisation of methods for Iterator
/// ---------------------------------------

#[allow(dead_code)]
impl<T> TreeIter<T>
	where T: Copy + Clone + Ord + Eq
{
	
	/// *English*: Method **new()** creates *empty iterator*. ~~Sometimes useful~~.
	///
	/// *Russian*: Добавляем метод **new()** для нашего итератора.
	/// ~~Иногда полезен~~
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
	
	pub fn new() -> Self {
		TreeIter::default()
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
	
	pub fn len(&self) -> usize {
		self.iter.len()
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
		return self.iter.pop_front();
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

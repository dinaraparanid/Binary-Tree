use crate::branch::Branch;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::mem;

/// **Node Realisation**
/// --------------------
///
/// *English*: Enumeration **Node<T>** for tree.
/// Node can be *empty* or *not empty and contains Box<Branch<T>>*.
///
/// I decided to use *Box<T>* type, cause our structure is *recursive*
/// (nodes collect branches and branches contain nodes).
/// That's means that all *branches are in heap and have constant size on stack*.
///
/// Node<T> is a *private* struct, so check **tests.rs** for more information.
///
/// *Russian*: Перечисление **узел (Node<T>)** для дерева.
/// Узел может быть либо *пустым*, либо *не пустым*.
/// Если он пуст, то принимает значение *Empty*, иначе
/// *NonEmpty, которое хранит Box<Branch<T>>*.
///
/// Т.к. наша структура *рекурсивна* (Узлы хранят ветви,
/// а ветви хранят узлы), но наша структура должна иметь
/// некий размер, который можно вычислить в момент компиляции,
/// то я принял решение использовать Box<T>, который добавляет всё
/// содержимое в кучу, и хранит некий указатель на всё это,
/// который на стеке имеет константное значение.
///
/// Т.к. перечисление приватно, то советую изучить tests.rs
/// В котором для каждого метода есть тесты.

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub(crate) enum Node<T>
	where T: Copy + Clone + Ord + Eq
{
	Empty,
	NonEmpty(Box<Branch<T>>),
}

/// *English*: **Default** trait for node. Creates *empty* node.
///
/// *Russian*: Трейт **Default** для узла. По-умолчанию наш узел *пуст*

impl<T> Default for Node<T>
	where T: Copy + Clone + Ord + Eq
{
	#[inline]
	fn default() -> Self {
		Node::Empty
	}
}

/// *English*: All methods for node.
///
/// *Russian*: Все методы для нашего узла.

#[allow(dead_code)]
impl<T> Node<T>
	where T: Copy + Clone + Ord + Eq
{
	
	/// *English*: creates empty node
	///
	/// *Russian*: Создаём новый узел
	
	#[inline]
	pub(crate) fn new() -> Self {
		Node::default()
	}
	
	/// *English*: Methods **ignore()** and **ignore_mut()** convert node to branch
	/// If node is empty, it'll panic
	///
	/// *Russian*: Методы **ignore()** и **ignore_mut()** конвертируют узел в ветвь.
	/// Т.к. узел может быть пуст, то может вызваться паника
	
	#[inline]
	pub(crate) fn ignore(&self) -> &Box<Branch<T>> {
		if let Node::NonEmpty(ref branch) = *self {
			return branch;
		} else {
			panic!("Empty tree");
		}
	}
	
	#[inline]
	pub(crate) fn ignore_mut(&mut self) -> &mut Box<Branch<T>> {
		if let Node::NonEmpty(ref mut branch) = *self {
			return branch;
		} else {
			panic!("Empty tree");
		}
	}
	
	/// *English*: *Get key* from node. *Panic* is possible
	///
	/// *Russian*: *Получение ключа* из ветви. Возможна *паника*
	
	#[inline]
	pub(crate) fn get_key(&self) -> &T {
		&self.ignore().key
	}
	
	/// *English*: Method **insert()** adds value to node.
	///
	/// *Russian*: Метод **insert()** добавляет значения в узел.
	
	pub(crate) fn insert(&mut self, val: &T) {
		match *self {
			Node::Empty => {
				*self = Node::NonEmpty(Box::new(Branch {
					key: (*val).clone(),
					right: Node::Empty,
					left: Node::Empty,
				}))
			}
			Node::NonEmpty(ref mut branch) => {
				if branch.key <= *val {
					branch.right.insert(val);
				} else {
					branch.left.insert(val);
				}
			}
		}
	}
	
	/// *English*: Method **find()** searches for value in subtree of node.
	/// If the value isn't exists or tree is empty, it'll return *Empty*, else *NonEmpty*
	///
	/// *Russian*: Метод **find()** ищет значение в поддереве узла.
	/// В случае, когда мы *не находим значение, или поддерево пусто*,
	/// Вернётся *пустой узел*, иначе *узел с эквивалентным значением*.
	
	pub(crate) fn find(&self, val: &T) -> &Self {
		let mut find = self;
		while let Node::NonEmpty(ref branch) = *find {
			match val.cmp(find.get_key()) {
				Ordering::Less => find = &branch.left,
				Ordering::Greater => find = &branch.right,
				Ordering::Equal => return find,
			}
		}
		&Node::Empty
	}
	
	/// *English*: Search min value in node. If it's empty return *Empty*
	///
	/// *Russian*: Поиск минимального узла в ветви.
	/// В случае, если поддерево, возвращаем пустой узел.
	
	pub(crate) fn min(&self) -> &Self {
		let mut min = self;
		while min.ignore().left != Node::Empty {
			min = &min.ignore().left;
		}
		min
	}
	
	/// *English*: Search max value in node. If it's empty return *Empty*
	///
	/// *Russian*: Поиск максимального узла в ветви.
	/// В случае, если поддерево, возвращаем пустой узел.
	
	pub(crate) fn max(&self) -> &Self {
		let mut max = self;
		while max.ignore().right != Node::Empty {
			max = &max.ignore().right;
		}
		max
	}
	
	/// *English*: *Deep-First-Search (DFS)* realisation.
	/// Returns VecDeque with keys.
	///
	/// *Russian*: *Обход поддерева в глубину*.
	/// Возвращаем дек со значениями из поддерева.
	
	pub(crate) fn walk(&self) -> VecDeque<T> {
		return match *self {
			Node::Empty => VecDeque::new(),
			Node::NonEmpty(ref branch) => {
				let mut result = branch.left.walk();
				result.push_back(branch.key.clone());
				result.extend(branch.right.walk());
				result
			}
		}
	}
	
	/// *English*: Removing value from tree. ~~It's simple clear
	/// sub_node and adds all keys of it~~.
	///
	/// *Russian*: Рекурсивное удаление узла и его подузлов.
	/// В итоге узел становится пустым.
	/// ~~Указатели на "удалённые узлы" остаются,
	/// но они ничего не весят, а значит мы можем
	/// сказать, что произошло реальное удаление~~.
	
	pub(crate) fn rec_drop(&mut self) {
		match *self {
			Node::Empty => return,
			Node::NonEmpty(ref mut branch) => {
				branch.left.rec_drop();
				branch.right.rec_drop();
				mem::drop(branch.key);
				*self = Node::Empty;
			}
		}
	}
	
	/// *English*: Removes keys and saves it keys
	///
	/// *Russian*: Удаление узла и сохранение ключей его подузлов.
	
	pub(crate) fn remove(&mut self, val: &T) -> (bool, Vec<T>) {
		let mut find = self;
		let mut search = false;
		while *find != Node::Empty {
			match val.cmp(find.get_key()) {
				Ordering::Less => find = &mut find.ignore_mut().left,
				Ordering::Greater => find = &mut find.ignore_mut().right,
				Ordering::Equal => {
					search = true;
					break;
				},
			}
		}
		
		let mut safe = Vec::new();
		if search {
			safe.extend(find.ignore().left.walk());
			safe.extend(find.ignore().right.walk());
			find.rec_drop();
		} else {
			if let Node::NonEmpty(ref node) = *find {
				if node.key == *val {
					search = true;
					safe.extend(node.left.walk());
					safe.extend(node.right.walk());
					find.rec_drop();
				}
			}
		}
		(search, safe)
	}
}

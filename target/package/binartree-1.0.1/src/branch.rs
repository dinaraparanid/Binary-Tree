use crate::node::Node;

/// *English*: Tree's branch
/// Contains key, right and left node.
///
/// *Russian*: Ветвь дерева.
/// Хранит ключ, правый и левый узел.

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub(crate) struct Branch<T>
	where T: Copy + Clone + Ord + Eq
{
	pub(crate) key: T,
	pub(crate) right: Node<T>,
	pub(crate) left: Node<T>,
}
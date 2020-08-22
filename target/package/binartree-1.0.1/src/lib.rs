/// *English*: Library with all modules.
/// User need only tree and iter, so other modules are private.
///
/// *Russian*: Библиотека со всеми модулями.
/// Пользователь использует только самое дерево
/// и итератор, так что незачем давать доступ
/// к другим структурам.

mod branch;
mod node;
mod tests;
pub mod iter;
pub mod tree;

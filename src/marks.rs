//! Value marks: opaque annotations that propagate through operations.
//!
//! Mirrors go-cty's `marks.go`, `marks_wrangle.go`, and the `ctymarks` helper
//! package. In Go a mark is any comparable value (`any`); in Rust a mark is a
//! [`Mark`], constructed from any hashable, comparable value via [`Mark::of`]
//! or the `From` conversions for common cases like `&str`.

use std::any::Any;
use std::fmt::Debug;
use std::hash::Hash;

use crate::error::Error;
use crate::path::Path;
use crate::value::Value;

/// A single mark: a type-erased, comparable, hashable annotation value.
///
/// Two marks are equal when they wrap values of the same type that compare
/// equal, mirroring Go's interface equality.
#[derive(Debug, Clone)]
pub struct Mark {
    _priv: (),
}

impl Mark {
    /// Wraps an arbitrary native value as a mark.
    pub fn of<T>(value: T) -> Mark
    where
        T: Any + Eq + Hash + Debug + Clone + Send + Sync,
    {
        let _ = value;
        todo!()
    }

    /// Downcasts to the wrapped native value, if it has type `T`.
    pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
        todo!()
    }
}

impl PartialEq for Mark {
    fn eq(&self, other: &Self) -> bool {
        let _ = other;
        todo!()
    }
}

impl Eq for Mark {}

impl Hash for Mark {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let _ = state;
        todo!()
    }
}

impl From<&str> for Mark {
    fn from(v: &str) -> Mark {
        Mark::of(v.to_string())
    }
}

impl From<String> for Mark {
    fn from(v: String) -> Mark {
        Mark::of(v)
    }
}

impl From<i64> for Mark {
    fn from(v: i64) -> Mark {
        Mark::of(v)
    }
}

impl From<bool> for Mark {
    fn from(v: bool) -> Mark {
        Mark::of(v)
    }
}

/// A set of marks associated with a value (go-cty: `cty.ValueMarks`).
#[derive(Debug, Clone)]
pub struct ValueMarks {
    _priv: (),
}

impl Default for ValueMarks {
    fn default() -> ValueMarks {
        ValueMarks::new()
    }
}

impl ValueMarks {
    /// The empty mark set.
    pub fn new() -> ValueMarks {
        todo!()
    }

    /// A mark set containing the given marks (go-cty: `cty.NewValueMarks`).
    pub fn from_marks<M: Into<Mark>>(marks: impl IntoIterator<Item = M>) -> ValueMarks {
        let _ = marks.into_iter().map(Into::into).collect::<Vec<_>>();
        todo!()
    }

    /// Whether the set contains the given mark (go-cty: `ValueMarks.Has`).
    pub fn has(&self, mark: impl Into<Mark>) -> bool {
        let _ = mark.into();
        todo!()
    }

    /// Adds all of the given marks to the set (go-cty: `ValueMarks.Insert`).
    pub fn insert<M: Into<Mark>>(&mut self, marks: impl IntoIterator<Item = M>) {
        let _ = marks.into_iter().map(Into::into).collect::<Vec<_>>();
        todo!()
    }

    /// The number of marks in the set.
    pub fn len(&self) -> usize {
        todo!()
    }

    /// Whether the set is empty.
    pub fn is_empty(&self) -> bool {
        todo!()
    }

    /// Iterates over the marks in the set, in unspecified order.
    pub fn iter(&self) -> impl Iterator<Item = &Mark> {
        std::iter::empty::<&Mark>()
    }

    /// The Go-syntax representation, identical to go-cty's `ValueMarks.GoString`,
    /// e.g. `cty.NewValueMarks("a")`.
    pub fn go_string(&self) -> String {
        todo!()
    }
}

/// Equality of mark sets (go-cty: `ValueMarks.Equal`).
impl PartialEq for ValueMarks {
    fn eq(&self, other: &Self) -> bool {
        let _ = other;
        todo!()
    }
}

impl Eq for ValueMarks {}

/// A set of marks associated with a path inside a value, used to attach and
/// recover marks on nested values (go-cty: `cty.PathValueMarks`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathValueMarks {
    /// The path at which the marks apply.
    pub path: Path,
    /// The marks applied at that path.
    pub marks: ValueMarks,
}

/// The action a [`WrangleFunc`] tells [`Value::wrangle_marks_deep`] to take for
/// one mark (go-cty: `ctymarks.WrangleAction`).
#[derive(Debug, Clone)]
pub enum WrangleAction {
    /// Keep the mark as-is (go-cty: `ctymarks.WrangleKeep`).
    Keep,
    /// Discard the mark (go-cty: `ctymarks.WrangleDrop`).
    Drop,
    /// Move the mark from a collection onto each of its elements
    /// (go-cty: `ctymarks.WrangleExpand`).
    Expand,
    /// Replace the mark with another mark (go-cty: `ctymarks.WrangleReplace`).
    Replace(Mark),
}

/// A callback deciding what to do with each mark encountered by
/// [`Value::wrangle_marks_deep`] (go-cty: `cty.WrangleFunc`).
pub type WrangleFunc<'a> = &'a mut dyn FnMut(&Mark, &Path) -> Result<WrangleAction, Error>;

impl Value {
    /// Whether this value is directly marked (go-cty: `Value.IsMarked`).
    pub fn is_marked(&self) -> bool {
        todo!()
    }

    /// Whether this value or any nested value is marked
    /// (go-cty: `Value.ContainsMarked`).
    pub fn contains_marked(&self) -> bool {
        todo!()
    }

    /// Whether this value is directly marked with the given mark
    /// (go-cty: `Value.HasMark`).
    pub fn has_mark(&self, mark: impl Into<Mark>) -> bool {
        let _ = mark.into();
        todo!()
    }

    /// Whether this value or any nested value carries the given mark
    /// (go-cty: `Value.HasMarkDeep`).
    pub fn has_mark_deep(&self, mark: impl Into<Mark>) -> bool {
        let _ = mark.into();
        todo!()
    }

    /// Whether this value has exactly the same direct marks as `other`
    /// (go-cty: `Value.HasSameMarks`).
    pub fn has_same_marks(&self, other: &Value) -> bool {
        let _ = other;
        todo!()
    }

    /// A copy of this value with the given mark added (go-cty: `Value.Mark`).
    pub fn mark(&self, mark: impl Into<Mark>) -> Value {
        let _ = mark.into();
        todo!()
    }

    /// The direct marks of this value (go-cty: `Value.Marks`).
    pub fn marks(&self) -> ValueMarks {
        todo!()
    }

    /// A copy with marks applied at the given paths within the value
    /// (go-cty: `Value.MarkWithPaths`).
    pub fn mark_with_paths(&self, path_marks: &[PathValueMarks]) -> Value {
        let _ = path_marks;
        todo!()
    }

    /// A copy with all of the given mark sets added (go-cty: `Value.WithMarks`).
    pub fn with_marks(&self, marks: impl IntoIterator<Item = ValueMarks>) -> Value {
        let _ = marks.into_iter().collect::<Vec<_>>();
        todo!()
    }

    /// A copy carrying the same direct marks as all of the given source values
    /// (go-cty: `Value.WithSameMarks`).
    pub fn with_same_marks(&self, sources: &[&Value]) -> Value {
        let _ = sources;
        todo!()
    }

    /// Removes and returns this value's direct marks (go-cty: `Value.Unmark`).
    pub fn unmark(&self) -> (Value, ValueMarks) {
        todo!()
    }

    /// Removes marks from this value and all nested values, returning the
    /// union of all removed marks (go-cty: `Value.UnmarkDeep`).
    pub fn unmark_deep(&self) -> (Value, ValueMarks) {
        todo!()
    }

    /// Removes marks from this value and all nested values, returning them
    /// with the paths they were found at (go-cty: `Value.UnmarkDeepWithPaths`).
    pub fn unmark_deep_with_paths(&self) -> (Value, Vec<PathValueMarks>) {
        todo!()
    }

    /// The direct marks of this value that wrap a native value of type `T`
    /// (go-cty: `cty.ValueMarksOfType[T]`).
    pub fn marks_of_type<T: Any + Clone>(&self) -> Vec<T> {
        todo!()
    }

    /// The marks of this value and all nested values that wrap a native value
    /// of type `T` (go-cty: `cty.ValueMarksOfTypeDeep[T]`).
    pub fn marks_of_type_deep<T: Any + Clone>(&self) -> Vec<T> {
        todo!()
    }

    /// Rewrites the marks of this value and all nested values by applying the
    /// given wrangler callbacks in turn (go-cty: `Value.WrangleMarksDeep`).
    pub fn wrangle_marks_deep(&self, wranglers: &mut [WrangleFunc<'_>]) -> Result<Value, Error> {
        let _ = wranglers;
        todo!()
    }
}

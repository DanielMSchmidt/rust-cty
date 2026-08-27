//! Paths through nested value structures: [`Path`], [`PathStep`], [`PathSet`].
//!
//! Mirrors go-cty's `path.go` and `path_set.go`. Where go-cty builds paths with
//! free functions (`cty.GetAttrPath("a").Index(...)`), the Rust API starts from
//! `Path::new()` and chains: `Path::new().attr("a").index_int(0)`.

use crate::error::Error;
use crate::value::Value;

/// One step in a [`Path`] (go-cty: `cty.PathStep`, i.e. `GetAttrStep` or
/// `IndexStep`).
#[derive(Debug, Clone)]
pub enum PathStep {
    /// Accessing an attribute of an object value (go-cty: `cty.GetAttrStep`).
    GetAttr(String),
    /// Indexing into a list, map, or set value (go-cty: `cty.IndexStep`).
    Index(Value),
}

impl PathStep {
    /// Applies this single step to a value (go-cty: `PathStep.Apply`).
    pub fn apply(&self, value: &Value) -> Result<Value, Error> {
        let _ = value;
        todo!()
    }

    /// The Go-syntax representation, identical to go-cty's `GoString` for the
    /// step, e.g. `cty.GetAttrStep{Name:"foo"}`.
    pub fn go_string(&self) -> String {
        todo!()
    }
}

impl PartialEq for PathStep {
    fn eq(&self, other: &Self) -> bool {
        let _ = other;
        todo!()
    }
}

impl Eq for PathStep {}

/// A sequence of [`PathStep`]s describing a route to a nested value
/// (go-cty: `cty.Path`).
#[derive(Debug, Clone)]
pub struct Path {
    _priv: (),
}

/// Step-wise path equality (go-cty: `Path.Equals`).
impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        let _ = other;
        todo!()
    }
}

impl Eq for Path {}

impl Default for Path {
    fn default() -> Path {
        Path::new()
    }
}

impl Path {
    /// The empty path, addressing the root value itself (go-cty: `cty.Path{}`).
    pub fn new() -> Path {
        todo!()
    }

    /// A path built from explicit steps.
    pub fn from_steps(steps: impl IntoIterator<Item = PathStep>) -> Path {
        let _ = steps.into_iter().collect::<Vec<_>>();
        todo!()
    }

    /// The steps of this path, in order.
    pub fn steps(&self) -> &[PathStep] {
        todo!()
    }

    /// Whether this is the empty path.
    pub fn is_empty(&self) -> bool {
        todo!()
    }

    /// The number of steps.
    pub fn len(&self) -> usize {
        todo!()
    }

    /// Returns a new path with a get-attribute step appended
    /// (go-cty: `Path.GetAttr` / `cty.GetAttrPath`).
    pub fn attr(&self, name: impl Into<String>) -> Path {
        let _ = name.into();
        todo!()
    }

    /// Returns a new path with an index step appended
    /// (go-cty: `Path.Index` / `cty.IndexPath`).
    pub fn index(&self, key: Value) -> Path {
        let _ = key;
        todo!()
    }

    /// Shorthand for [`Path::index`] with a number key
    /// (go-cty: `Path.IndexInt` / `cty.IndexIntPath`).
    pub fn index_int(&self, key: i64) -> Path {
        let _ = key;
        todo!()
    }

    /// Shorthand for [`Path::index`] with a string key
    /// (go-cty: `Path.IndexString` / `cty.IndexStringPath`).
    pub fn index_string(&self, key: impl Into<String>) -> Path {
        let _ = key.into();
        todo!()
    }

    /// Follows the path down into the given value (go-cty: `Path.Apply`).
    pub fn apply(&self, value: &Value) -> Result<Value, Error> {
        let _ = value;
        todo!()
    }

    /// Applies all but the final step, returning the penultimate value and the
    /// final step (go-cty: `Path.LastStep`).
    pub fn last_step(&self, value: &Value) -> Result<(Value, PathStep), Error> {
        let _ = value;
        todo!()
    }

    /// Whether this path equals another (go-cty: `Path.Equals`).
    /// Also available via `==` through `PartialEq`.
    pub fn equals(&self, other: &Path) -> bool {
        self == other
    }

    /// Whether this path starts with all the steps of `prefix`
    /// (go-cty: `Path.HasPrefix`).
    pub fn has_prefix(&self, prefix: &Path) -> bool {
        let _ = prefix;
        todo!()
    }

    /// Creates an error carrying this path as context (go-cty: `Path.NewErrorf`).
    pub fn error(&self, message: impl Into<String>) -> Error {
        Error::new_at_path(self.clone(), message)
    }
}

/// A mutable set of [`Path`]s (go-cty: `cty.PathSet`).
#[derive(Debug, Clone)]
pub struct PathSet {
    _priv: (),
}

impl Default for PathSet {
    fn default() -> PathSet {
        PathSet::new([])
    }
}

impl PathSet {
    /// A new path set containing the given paths (go-cty: `cty.NewPathSet`).
    pub fn new(paths: impl IntoIterator<Item = Path>) -> PathSet {
        let _ = paths.into_iter().collect::<Vec<_>>();
        todo!()
    }

    /// Adds a path to the set (go-cty: `PathSet.Add`).
    pub fn add(&mut self, path: Path) {
        let _ = path;
        todo!()
    }

    /// Adds the path along with every one of its ancestor prefixes
    /// (go-cty: `PathSet.AddAllSteps`).
    pub fn add_all_steps(&mut self, path: Path) {
        let _ = path;
        todo!()
    }

    /// Whether the set contains the given path (go-cty: `PathSet.Has`).
    pub fn has(&self, path: &Path) -> bool {
        let _ = path;
        todo!()
    }

    /// Removes a path from the set (go-cty: `PathSet.Remove`).
    pub fn remove(&mut self, path: &Path) {
        let _ = path;
        todo!()
    }

    /// All paths in the set, in an unspecified but consistent order
    /// (go-cty: `PathSet.List`).
    pub fn list(&self) -> Vec<Path> {
        todo!()
    }

    /// Whether the set is empty (go-cty: `PathSet.Empty`).
    pub fn is_empty(&self) -> bool {
        todo!()
    }

    /// The union of this set and another (go-cty: `PathSet.Union`).
    pub fn union(&self, other: &PathSet) -> PathSet {
        let _ = other;
        todo!()
    }

    /// The intersection of this set and another (go-cty: `PathSet.Intersection`).
    pub fn intersection(&self, other: &PathSet) -> PathSet {
        let _ = other;
        todo!()
    }

    /// The paths in this set that are not in the other
    /// (go-cty: `PathSet.Subtract`).
    pub fn subtract(&self, other: &PathSet) -> PathSet {
        let _ = other;
        todo!()
    }

    /// The paths in exactly one of the two sets
    /// (go-cty: `PathSet.SymmetricDifference`).
    pub fn symmetric_difference(&self, other: &PathSet) -> PathSet {
        let _ = other;
        todo!()
    }
}

/// Set equality (go-cty: `PathSet.Equal`).
impl PartialEq for PathSet {
    fn eq(&self, other: &Self) -> bool {
        let _ = other;
        todo!()
    }
}

impl Eq for PathSet {}

///
/// Contains raw bindings to the types and apis declared in libpipeworks
/// Apis are subject to their documentation in libpipeworks, and care must be taken to observe
/// the limitations of the api as documented.
/// Changes to these bindings are not considered to be semver incompatible changes,
///  regardless of the result of those changes.
pub mod sys;

pub mod engine;
pub mod game;
pub mod thing;
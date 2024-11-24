
use crate::data::Tag;

/// Fire can spread to blocks with this tag.
/// 
/// TODO: determine what this means for burnable half-blocks
/// like slabs and fences.
pub const BURNABLE: Tag = Tag::new("burnable");

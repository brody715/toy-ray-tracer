pub mod containers;
pub mod shapes;
pub mod transforms;
mod visitor;
mod visitors;
pub mod volumes;

pub use visitor::{EnterContext, GeometryVisitor, GeometryWalker};

pub use visitors::try_get_light_from_world;

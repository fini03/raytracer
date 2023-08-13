mod light;
mod ambient;
mod parallel;
mod point;
mod spot;
pub mod structs;

pub use light::{LightModel, Lights, Phong, CookTorrance};
pub use structs::{Lights as OtherLights};
pub use ambient::Ambient;
pub use parallel::Parallel;
pub use point::Point;
pub use spot::Spot;

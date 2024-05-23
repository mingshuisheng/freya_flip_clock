mod close;
mod locked;
mod to_bottom;
mod to_normal;
mod to_top;
mod unclocked;

pub use close::CloseSvg;
use freya::prelude::*;
pub use locked::LockedSvg;
pub use to_bottom::ToBottomSvg;
pub use to_normal::ToNormalSvg;
pub use to_top::ToTopSvg;
pub use unclocked::UnLockedSvg;

#[derive(Props, Clone, PartialEq, Debug)]
pub struct SvgProps {
    stroke_color: String,
}

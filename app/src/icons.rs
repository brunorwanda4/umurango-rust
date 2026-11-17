use gpui_kit::{svg, Svg};

pub const WALLET: &[u8] =
    include_bytes!("../../assets/fontawesome/free-solid-svg-icons/wallet.svg");
pub const ROTATE: &[u8] =
    include_bytes!("../../assets/fontawesome/free-solid-svg-icons/rotate.svg");
pub const CIRCLE_CHECK: &[u8] =
    include_bytes!("../../assets/fontawesome/free-solid-svg-icons/circle-check.svg");

pub fn icon(data: &'static [u8]) -> Svg {
    svg().data(data)
}

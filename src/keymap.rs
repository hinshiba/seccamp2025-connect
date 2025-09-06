use rmk::action::KeyAction;
use rmk::{k, layer, mo};
pub(crate) const COL: usize = 4;
pub(crate) const ROW: usize = 2;
pub(crate) const NUM_LAYER: usize = 2;

#[rustfmt::skip]
pub const fn get_default_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
    [
        layer!([
            [k!(A), k!(B), k!(Backspace), k!(G)],
            [k!(C), k!(D), mo!(1), k!(H)]
        ]),
        layer!([
            [k!(E), k!(F), k!(Enter), k!(I)],
            [k!(Kc4), k!(Kc5), k!(Kc6), k!(J)]
        ]),
    ]
}

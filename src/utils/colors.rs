use crate::math::Color;

pub fn get_int_color(
    out: &mut [u8;3],
    color: &Color
) {
    let r = color.x;
    let g = color.y;
    let b = color.z;

    out[0] = (r.clamp(0., 0.999) * 256.) as u8;
    out[1] = (g.clamp(0., 0.999) * 256.) as u8;
    out[2] = (b.clamp(0., 0.999) * 256.) as u8;
}

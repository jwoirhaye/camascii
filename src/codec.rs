pub const ASCII_CHARS: &[u8] =
    b" .'`^\",:;Il!i><~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";

#[inline]
pub fn luma_to_char(luma: u8) -> char {
    let idx = (luma as usize * (ASCII_CHARS.len() - 1)) / 255;
    ASCII_CHARS[idx] as char
}

#[inline]
pub fn yuyv_to_rgb(frame: &[u8], src_w: u32, x: u32, y: u32) -> (u8, u8, u8, u8) {
    let pair_idx = (y * src_w + (x & !1)) as usize * 2;
    if pair_idx + 3 >= frame.len() {
        return (0, 0, 0, 0);
    }
    let y0 = frame[pair_idx] as f32;
    let u = frame[pair_idx + 1] as f32 - 128.0;
    let y1 = frame[pair_idx + 2] as f32;
    let v = frame[pair_idx + 3] as f32 - 128.0;
    let luma = if x & 1 == 0 { y0 } else { y1 };

    let r = (luma + 1.402 * v).clamp(0.0, 255.0) as u8;
    let g = (luma - 0.344_136 * u - 0.714_136 * v).clamp(0.0, 255.0) as u8;
    let b = (luma + 1.772 * u).clamp(0.0, 255.0) as u8;

    (r, g, b, luma as u8)
}

#[cfg(test)]
mod tests {
    use crate::codec::{ASCII_CHARS, luma_to_char, yuyv_to_rgb};

    #[test]
    fn darkest_luma_is_space() {
        assert_eq!(luma_to_char(0), ' ');
    }

    #[test]
    fn brightest_luma_is_last_char() {
        let expected = *ASCII_CHARS.last().unwrap() as char;
        assert_eq!(luma_to_char(255), expected);
    }

    #[test]
    fn out_of_bounds_returns_black() {
        let frame = [0u8; 4];
        assert_eq!(yuyv_to_rgb(&frame, 2, 10, 0), (0, 0, 0, 0));
    }

    #[test]
    fn pure_white_bt601() {
        let frame = [235u8, 128, 235, 128];
        let (r, g, b, luma) = yuyv_to_rgb(&frame, 2, 0, 0);
        assert!(r > 200, "R={r}");
        assert!(g > 200, "G={g}");
        assert!(b > 200, "B={b}");
        assert_eq!(luma, 235);
    }

    #[test]
    fn pure_black_bt601() {
        let frame = [16u8, 128, 16, 128];
        let (r, g, b, luma) = yuyv_to_rgb(&frame, 2, 0, 0);
        assert!(r < 20, "R={r}");
        assert!(g < 20, "G={g}");
        assert!(b < 20, "B={b}");
        assert_eq!(luma, 16);
    }
}

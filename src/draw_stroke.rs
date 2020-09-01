use crate::{bold, path3::Path, fill_rule, fill_path2::draw_fill};

pub fn draw_stroke<F: FnMut(u32, u32, f64)>(
    width: u32,
    height: u32,
    path: &Path,
    line_width: f64,
    writer: &mut F,
) {
    let path = Path(bold::path_bold1(path, line_width / 2.0));
    let fill_rule = fill_rule::NonZero;
    draw_fill(width, height, &path, fill_rule, writer)
}

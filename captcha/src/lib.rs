#[cfg(test)]
mod tests {
    use std::any::Any;

    use ab_glyph::FontRef;
    use image::{imageops, Pixel, Rgba, RgbaImage};
    use imageproc::drawing;

    #[test]
    fn solid_color_bg() {
        // 像素（颜色）：红色，不透明
        let pixel = image::Rgba([255u8, 0u8, 0u8, 255u8]);
        // 创建图片缓冲区，并使用上面定义的像素点（颜色）作为背景填充整个图片
        let img = RgbaImage::from_fn(100, 100, |_, _| pixel);
        // 保存
        img.save("foo.png").unwrap();
    }

    #[test]
    fn gradient_bg() {
        let mut img = RgbaImage::new(100, 100);

        // #22d3ee - cyan-400(tailwind css)
        let start = Rgba::from_slice(&[0x22, 0xd3, 0xee, 0xff]);
        // #164e63 - cyan-900(tailwind css)
        let end = Rgba::from_slice(&[0x16, 0x4e, 0x63, 0xff]);

        imageops::vertical_gradient(&mut img, start, end);
        // 保存
        img.save("foo.png").unwrap();
    }

    #[test]
    fn draw_pixel() {
        let mut img = RgbaImage::new(100, 100);

        // #22d3ee - cyan-400(tailwind css)
        let start = Rgba::from_slice(&[0x22, 0xd3, 0xee, 0xff]);
        // #164e63 - cyan-900(tailwind css)
        let end = Rgba::from_slice(&[0x16, 0x4e, 0x63, 0xff]);
        imageops::vertical_gradient(&mut img, start, end);

        // 像素（颜色）：黄色，不透明
        let pixel = image::Rgba::from_slice(&[0xff, 0xff, 0x0, 0xff]);
        img.put_pixel(12, 34, *pixel);

        // 保存
        img.save("foo.png").unwrap();
    }

    #[test]
    fn draw_line() {
        let mut img = RgbaImage::new(100, 100);

        // #22d3ee - cyan-400(tailwind css)
        let start = Rgba::from_slice(&[0x22, 0xd3, 0xee, 0xff]);
        // #164e63 - cyan-900(tailwind css)
        let end = Rgba::from_slice(&[0x16, 0x4e, 0x63, 0xff]);
        imageops::vertical_gradient(&mut img, start, end);

        // 绘制直线
        drawing::draw_line_segment_mut(
            &mut img,                       // 画布
            (12.0, 34.0),                   // 开始坐标
            (56.0, 78.0),                   // 结束坐标
            Rgba([255u8, 0u8, 0u8, 255u8]), // 颜色
        );

        // 保存
        img.save("foo.png").unwrap();
    }

    #[test]
    fn draw_curve() {
        let mut img = RgbaImage::new(100, 100);

        // #22d3ee - cyan-400(tailwind css)
        let start = Rgba::from_slice(&[0x22, 0xd3, 0xee, 0xff]);
        // #164e63 - cyan-900(tailwind css)
        let end = Rgba::from_slice(&[0x16, 0x4e, 0x63, 0xff]);
        imageops::vertical_gradient(&mut img, start, end);

        // 绘制曲线
        drawing::draw_cubic_bezier_curve_mut(
            &mut img,
            (78.9, 90.0),
            (89.0, 99.0),
            (78.9, 90.0),
            (81.0, 7.0),
            Rgba([255u8, 0u8, 0u8, 255u8]),
        );

        // 保存
        img.save("foo.png").unwrap();
    }

    #[test]
    fn load_font() {
        let font = FontRef::try_from_slice(include_bytes!("../jxzk.ttf")).unwrap();
        println!("{:?}", font.type_id());
    }
}

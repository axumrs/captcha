#[cfg(test)]
mod tests {
    use image::{imageops, Pixel, Rgba, RgbaImage};

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
        let end = Rgba::from_slice(&[0x16, 0x43, 0x63, 0xff]);

        imageops::vertical_gradient(&mut img, start, end);
        // 保存
        img.save("foo.png").unwrap();
    }
}

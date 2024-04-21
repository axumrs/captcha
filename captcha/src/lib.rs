#[cfg(test)]
mod tests {
    use image::RgbaImage;

    #[test]
    fn solid_color_bg() {
        // 像素（颜色）：红色，不透明
        let pixel = image::Rgba([255u8, 0u8, 0u8, 255u8]);
        // 创建图片缓冲区，并使用上面定义的像素点（颜色）作为背景填充整个图片
        let img = RgbaImage::from_fn(100, 100, |_, _| pixel);
        // 保存
        img.save("foo.png").unwrap();
    }
}

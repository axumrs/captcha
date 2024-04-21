use std::error::Error;

use ab_glyph::FontRef;
use image::{imageops, Pixel, Rgba, RgbaImage};
use imageproc::drawing;
use rand::Rng;

const WIDTH: u32 = 100;
const HEIGHT: u32 = 50;

pub fn generator() -> Result<(RgbaImage, String), Box<dyn Error>> {
    let code = code();
    let mut img = RgbaImage::new(WIDTH, HEIGHT);
    // #22d3ee - cyan-400(tailwind css)
    let start = Rgba::from_slice(&[0x22, 0xd3, 0xee, 0xff]);
    // #164e63 - cyan-900(tailwind css)
    let end = Rgba::from_slice(&[0x16, 0x4e, 0x63, 0xff]);
    imageops::vertical_gradient(&mut img, start, end);

    // 加载字体
    let font = FontRef::try_from_slice(include_bytes!("../jxzk.ttf")).unwrap();

    let mut rnd = rand::thread_rng();

    let r: u8 = rnd.gen_range(0..=255);
    let g: u8 = rnd.gen_range(0..=255);
    let b: u8 = rnd.gen_range(0..=255);
    // 绘制文本
    drawing::draw_text_mut(
        &mut img,
        image::Rgba([r, g, b, 255u8]),
        16,
        8,
        32f32,
        &font,
        &code,
    );

    // 画点
    for _ in 0..50 {
        let x: u32 = rnd.gen_range(0..WIDTH);
        let y: u32 = rnd.gen_range(0..HEIGHT);
        let r: u8 = rnd.gen_range(0..=255);
        let g: u8 = rnd.gen_range(0..=255);
        let b: u8 = rnd.gen_range(0..=255);
        let pixel = image::Rgba([r, g, b, 255u8]);
        img.put_pixel(x, y, pixel);
    }

    // 画线
    for _ in 0..10 {
        let start: (f32, f32) = (
            rnd.gen_range(0..WIDTH) as f32,
            rnd.gen_range(0..HEIGHT) as f32,
        );
        let end: (f32, f32) = (
            rnd.gen_range(0..WIDTH) as f32,
            rnd.gen_range(0..HEIGHT) as f32,
        );
        let r: u8 = rnd.gen_range(0..=255);
        let g: u8 = rnd.gen_range(0..=255);
        let b: u8 = rnd.gen_range(0..=255);
        // 绘制直线
        drawing::draw_line_segment_mut(
            &mut img,               // 画布
            start,                  // 开始坐标
            end,                    // 结束坐标
            Rgba([r, g, b, 255u8]), // 颜色
        );
    }

    // 画曲线
    for _ in 0..5 {
        let start: (f32, f32) = (
            rnd.gen_range(0..WIDTH) as f32,
            rnd.gen_range(0..HEIGHT) as f32,
        );
        let end: (f32, f32) = (
            rnd.gen_range(0..WIDTH) as f32,
            rnd.gen_range(0..HEIGHT) as f32,
        );
        let control_a: (f32, f32) = (rnd.gen_range(0..360) as f32, rnd.gen_range(0..360) as f32);
        let control_b: (f32, f32) = (
            rnd.gen_range(0..control_a.0 as i32) as f32,
            rnd.gen_range(0..control_a.1 as i32) as f32,
        );
        let r: u8 = rnd.gen_range(0..=255);
        let g: u8 = rnd.gen_range(0..=255);
        let b: u8 = rnd.gen_range(0..=255);
        // 绘制曲线
        drawing::draw_cubic_bezier_curve_mut(
            &mut img,
            start,
            end,
            control_a,
            control_b,
            Rgba([r, g, b, 255u8]),
        );
    }

    Ok((img, code))
}

pub fn code() -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789";
    const LEN: usize = 5;
    let mut rnd = rand::thread_rng();
    let code: String = (0..LEN)
        .map(|_| {
            let idx = rnd.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    code
}

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

    #[test]
    fn draw_text() {
        let mut img = RgbaImage::new(100, 100);

        // #22d3ee - cyan-400(tailwind css)
        let start = Rgba::from_slice(&[0x22, 0xd3, 0xee, 0xff]);
        // #164e63 - cyan-900(tailwind css)
        let end = Rgba::from_slice(&[0x16, 0x4e, 0x63, 0xff]);
        imageops::vertical_gradient(&mut img, start, end);

        // 加载字体
        let font = FontRef::try_from_slice(include_bytes!("../jxzk.ttf")).unwrap();

        // 绘制文本
        drawing::draw_text_mut(
            &mut img,
            image::Rgba([255u8, 255u8, 255u8, 255u8]),
            10,
            35,
            20f32,
            &font,
            "AXUM中文网",
        );

        // 保存
        img.save("foo.png").unwrap();
    }

    #[test]
    fn generator() {
        let (img, code) = super::generator().unwrap();
        img.save("foo.png").unwrap();
        println!("{code}");
    }

    #[test]
    fn code() {
        for _ in 0..10 {
            println!("{}", super::code());
        }
    }
}

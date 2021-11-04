mod color_input;
use crate::color_input::color_input;

struct ColorRgb
{
    r: u8,
    g: u8,
    b: u8,
}

struct ColorPov
{
    r: f64,
    g: f64,
    b: f64,
}

fn main() {
    let mut ret;
    let mut run = true;

    // colors

    let mut color_rgb = ColorRgb {r: 0, g: 0, b: 0};
    let mut color_pov = ColorPov {r: 0.0, g: 0.0, b: 0.0};

    let conv:f64 = 1.0 / 255.0;

    while run == true
    {
        ret = color_input (&mut color_rgb.r, &mut color_rgb.g, &mut color_rgb.b);
        if ret == 0
        {
            // calculate POV Ray colors in range of 0.0 to 1.0;

            color_pov.r = color_rgb.r as f64;
            color_pov.r = color_pov.r * conv;

            color_pov.g = color_rgb.g as f64;
            color_pov.g = color_pov.g * conv;

            color_pov.b = color_rgb.b as f64;
            color_pov.b = color_pov.b * conv;

            println! ("color rgb<{:.4}, {:.4}, {:.4}>", color_pov.r, color_pov.g, color_pov.b);
            println! ()
        }
        else
        {
            run = false;
        }
    }
}

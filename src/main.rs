mod color_input;
use crate::color_input::color_input;

fn main() {
    let mut ret;
    let mut run = true;

    // colors
    let mut r:u8 = 0;
    let mut g:u8 = 0;
    let mut b:u8 = 0;
    let mut pov_r:f64;
    let mut pov_g:f64;
    let mut pov_b:f64;

    let conv:f64 = 1.0 / 255.0;

    while run == true
    {
        ret = color_input (&mut r, &mut g, &mut b);
        if ret == 0
        {
            // calculate POV Ray colors in range of 0.0 to 1.0;

            pov_r = r as f64;
            pov_r = pov_r * conv;

            pov_g = g as f64;
            pov_g = pov_g * conv;

            pov_b = b as f64;
            pov_b = pov_b * conv;

            println! ("color rgb<{:.4}, {:.4}, {:.4}>", pov_r, pov_g, pov_b);
            println! ()
        }
        else
        {
            run = false;
        }
    }
}

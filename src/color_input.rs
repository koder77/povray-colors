use std::io;
use std::io::Write;

pub fn color_input(r: &mut u8, g: &mut u8, b: &mut u8) -> i16
{
    let input_len;

    let mut input_buffer_r = String::new();
    let mut input_buffer_g = String::new();
    let mut input_buffer_b = String::new();

    // get user rgb input
    println! ("Enter rgb color, only RETURN to exit:");
    print! ("r? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line (&mut input_buffer_r).expect("error reading inpu!");

    // check for empty input
    input_len = input_buffer_r.chars().count();
    if input_len == 1
    {
        return 1;
    }

    let inp_r: u8 = input_buffer_r.trim().parse::<u8>().unwrap();

    print! ("g? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line (&mut input_buffer_g).expect("error reading inpu!");
    let inp_g: u8 = input_buffer_g.trim().parse::<u8>().unwrap();

    print! ("b? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line (&mut input_buffer_b).expect("error reading inpu!");
    let inp_b: u8 = input_buffer_b.trim().parse::<u8>().unwrap();

    *r = inp_r;
    *g = inp_g;
    *b = inp_b;

    return 0;
}

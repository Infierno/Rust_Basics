//This Funtion Get Terminal Size, and send it to optics_inhibition.

fn reboot_optics() -> (u16, u16) {
    use terminal_size::{Height, Width, terminal_size};

    let size = terminal_size();
    if let Some((Width(w), Height(h))) = size {
        (w, h)
    } else {
        let error_w = 120;
        let error_h = 15;
        (error_h, error_w)
    }
}

// This Function Gets reboot_optics terminal value and draws cli box.

fn optics_inhibition() {
    use termion::color;
    /*Asigns white color using termion crate*/
    println!("{}", color::Fg(color::White));
    /*Print value of optics a and b */
    let (optics_a, optics_b) = reboot_optics();
    println!("Reboot Optics value: {}", optics_a);
    println!("Reboot Optics value: {}", optics_b);

    let optics_copy = optics_a;
    let mut optics_a_loop = 0;

    while optics_a_loop <= optics_copy - 1 {
        print!("-");
        optics_a_loop = optics_a_loop + 1;
    }

    /*reset terminal colors*/

    println!("{}", color::Fg(color::Reset));
}

use std::error::Error;
fn main() -> Result<(), Box<dyn Error>> {
    use std::io::{Write, stdout};
    use termion::raw::IntoRawMode;

    let mut stdout = stdout().into_raw_mode()?;
    write!(stdout, "Hey there.").unwrap();
    optics_inhibition();
    Ok(()) // Return Ok(()) on success
}

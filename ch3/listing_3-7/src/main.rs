use rand::{random}; // brings the rand crate into scope

static mut ERROR: isize = 0; // initializes ERROR to 0.

struct File; // createa a zero-sized type to stand in for a struct while we're experimenting.

fn read(f: &File, save_to: &mut Vec<u8>) -> usize {
    if random() && random() && random() { // returns true one out of either times this function is
                                          // called
        unsafe {
            ERROR = 1; // sets ERROR to 1, notifying the rest of thge system that an error has
                       // occurred.
        }
    }

    0 // always reads 0 bytes
}

#[allow(unused_mut)] // keeping buffer mutable for consistency with other code even though it isn't
                     // touched here.
fn main() {
    let mut f = File;
    let mut buffer = vec![];

    read(&f, &mut buffer);

    unsafe { // accessing static mut variables is an unsafe operation.
        if ERROR != 0 {
            panic!("An error has occurred")
        }
    }
}

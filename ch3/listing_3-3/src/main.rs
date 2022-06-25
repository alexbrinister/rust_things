#[allow(unused_variables)] // silences warnings

#[derive(Debug)] // enables File to work with println! and its fmt! sibling macros (used at the
                 // bottom of the listing)
struct File {
    name: String,
    data: Vec<u8>,
}

// inert for now
fn open(f: &mut File) -> bool {
    true
}

// inert for now
fn close(f: &mut File) -> bool {
    true
}

fn read(f: &File, save_to: &mut Vec<u8>) -> usize { // returns the number of bytes read
    let mut tmp = f.data.clone();   // makes a copy of the data here because save_to.append()
                                    // shrinks the input Vec<T>
    let read_length = tmp.len();

    save_to.reserve(read_length);   // Ensures that there is sufficient space to fit the incomming
                                    // data
    save_to.append(&mut tmp);       // allocates sufficent data in the save_to buffer to hold the
                                    // contents of f
    read_length
}

fn main() {
    let mut f2 = File {
        name: String::from("2.txt"),
        data: vec![114, 117, 115, 116, 33],
    };

    let mut buffer: Vec<u8> = vec![];

    open(&mut f2);
    let f2_length = read(&f2, &mut buffer);
    close(&mut f2);

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f2);
    println!("{} is {} bytes long", &f2.name, f2_length);
    println!("{}", text);
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    fn new(name: &str) -> File {            // As File::new() is a completely normal function, we
                                            // need to tell Rust that it will return a File from
                                            // this function.

        File {                              // File::new() does little more than encapsulate the
                                            // object creation syntax, which is normal.
            name: String::from(name),
            data: Vec::new(),
        }
    }
}

fn main() {
    let f3 = File::new("f3.txt");

    let f3_name = &f3.name;                 // fields are private by default but can be accessed
                                            // within the module that defines the struct. the
                                            // module system is discussed later in the chapter.
    let f3_length = f3.data.len();

    println!("{:?}", f3);
    println!("{} is {} bytes long", f3_name, f3_length);
}

#[derive(Debug)]    // Allows println! to print File. The std::fmt::Debug trait works in
                    // conjunction with {:?} within the macro to enable File as a printable
                    // string.
struct File {     
    name: String, 
    data: Vec<u8>,  // Using Vec<u8> provides access to some useful conveniences, like dynamic
                    // sizing, which makes it possible to simulate writing to a file.
}

fn main() {
    let f1 = File {
        name: String::from("f1.txt"), // String::from generates owned strings from string literals,
                                      // which are slices.
        data: Vec::new(),
    };

    let f1_name = &f1.name;             // Accessing fields uses the `.` operator. Accessing fields
    let f1_length = &f1.data.len();     // by reference prevents their use after move issues.

    println!("{:?}", f1);
    println!("{} is {} bytes long", f1_name, f1_length);
}

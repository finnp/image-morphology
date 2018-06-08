use std::fmt;

#[derive(Debug)]
pub struct BinaryImage {
    pub width: usize,
    pub height: usize,
    pub data: Vec<bool>
}

impl BinaryImage {
    fn new (width: usize, height: usize) -> Self {
        BinaryImage {
            data: vec![false; width * height],
            width: width,
            height: height
        }
    }
}

impl fmt::Display for BinaryImage {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.data.chunks(self.width) {
            for cell in row.iter() {
                try! {write!(f, "{}", if *cell { "1" } else { "0"}) };
            }
            try! { write!(f, "\n") };
        }
        write!(f, "")
    }
}

fn main () {
    let test = BinaryImage::new(10, 20);
    println!("{}", test);
}

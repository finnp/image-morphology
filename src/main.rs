use std::fmt;
use std::cmp::min;

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

    fn filled (width: usize, height: usize) -> Self {
        BinaryImage {
            data: vec![true; width * height],
            width: width,
            height: height
        }
    }

    fn get (&self, x: usize, y: usize ) -> bool {
        if x >= self.width {
            return false
        }
        if y >= self.height {
            return false
        }
        self.data[x + y * self.width]
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

fn check_neighbours (img: &BinaryImage, kernel: &BinaryImage, x: usize, y: usize) -> bool {
    let anchor_x = kernel.width / 2;
    let anchor_y = kernel.height / 2;

    let offset_x = anchor_x - min(anchor_x, x);
    let offset_y = anchor_y - min(anchor_y, y);

    for dx in offset_x..(kernel.width - offset_x) {
        for dy in offset_y..(kernel.height - offset_y) {
            if img.get(x + dx - anchor_x, y + dy - anchor_y) {
                return true;
            }
        }
    }
    false
}

fn dilate (input: BinaryImage, kernel: BinaryImage) -> BinaryImage {
    let mut output = BinaryImage::new(input.width, input.height);
    for x in 1..input.width {
        for y in 1..output.height {
            output.data[x + y * output.width] = check_neighbours(&input, &kernel, x, y);
        }
    }
    output
}

fn main () {
    let kernel = BinaryImage::filled(3, 3);
    let mut test = BinaryImage::new(10, 20);
    test.data[44] = true;
    let output = dilate(test, kernel);
    println!("{}", output);
}

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

fn dilate (input: BinaryImage) -> BinaryImage {
    let mut output = BinaryImage::new(input.width, input.height);
    for x in 1..input.width {
        for y in 1..output.height {
            let current = input.get(x, y);
            let up = input.get(x, y - 1);
            let down = input.get(x, y + 1);
            let left = input.get(x - 1, y);
            let right = input.get(x + 1, y);
            output.data[x + y * output.width] = current || up || down || left || right;
        }
    }
    output
}

fn main () {
    let mut test = BinaryImage::new(10, 20);
    test.data[44] = true;
    let output = dilate(test);
    println!("{}", output);
}

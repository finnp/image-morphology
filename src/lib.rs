use std::fmt;
use std::cmp::min;

#[derive(Debug)]
pub struct Vec2d {
    pub width: usize,
    pub height: usize,
    pub data: Vec<bool>
}

impl Vec2d {
    pub fn new (width: usize, height: usize) -> Self {
        Vec2d {
            data: vec![false; width * height],
            width: width,
            height: height
        }
    }

    pub fn filled (width: usize, height: usize) -> Self {
        Vec2d {
            data: vec![true; width * height],
            width: width,
            height: height
        }
    }

    pub fn get (&self, x: usize, y: usize ) -> bool {
        if x >= self.width {
            return false
        }
        if y >= self.height {
            return false
        }
        self.data[x + y * self.width]
    }
}

impl fmt::Display for Vec2d {
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

fn check_neighbours (img: &Vec2d, kernel: &Vec2d, x: usize, y: usize, flip: bool) -> bool {
    let anchor_x = kernel.width / 2;
    let anchor_y = kernel.height / 2;

    let offset_x = anchor_x - min(anchor_x, x);
    let offset_y = anchor_y - min(anchor_y, y);

    for dx in offset_x..(kernel.width - offset_x) {
        for dy in offset_y..(kernel.height - offset_y) {
            if flip == img.get(x + dx - anchor_x, y + dy - anchor_y) {
                return flip;
            }
        }
    }
    !flip
}

pub fn dilate (input: &Vec2d, kernel: &Vec2d) -> Vec2d {
    let mut output = Vec2d::new(input.width, input.height);
    for x in 1..input.width {
        for y in 1..output.height {
            output.data[x + y * output.width] = check_neighbours(&input, &kernel, x, y, true);
        }
    }
    output
}

pub fn erode (input: &Vec2d, kernel: &Vec2d) -> Vec2d {
    let mut output = Vec2d::new(input.width, input.height);
    for x in 1..input.width {
        for y in 1..output.height {
            output.data[x + y * output.width] = check_neighbours(&input, &kernel, x, y, false);
        }
    }
    output
}

#[test]
fn it_works() {
    let kernel = Vec2d::filled(5, 5);
    let mut input = Vec2d::new(10, 20);
    input.data[44] = true;
    let output = dilate(&input, &kernel);
    let reversed = erode(&output, &kernel);
    assert!(!input.data.iter().eq(output.data.iter()));
    assert!(input.data.iter().eq(reversed.data.iter()), "for the simple case reverse is the same as the input");
}

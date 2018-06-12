use std::fmt;
use std::cmp::min;

/// Represents a [binary image](https://en.wikipedia.org/wiki/Binary_image)
#[derive(Debug)]
pub struct Vec2d {
    /// width of the vector
    pub width: usize,
    /// height of the vector
    pub height: usize,
    /// 2d vector represented in a 1d vector
    pub data: Vec<bool>
}

impl Vec2d {
    /// create a 2d vector filled with `false`
    pub fn empty (width: usize, height: usize) -> Self {
        Vec2d {
            data: vec![false; width * height],
            width: width,
            height: height
        }
    }

    /// create a 2d vector filled with `true`
    pub fn filled (width: usize, height: usize) -> Self {
        Vec2d {
            data: vec![true; width * height],
            width: width,
            height: height
        }
    }

    /// get the pixel on a `x` and `y` position
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

/// Performs [dilation](https://docs.opencv.org/2.4/doc/tutorials/imgproc/erosion_dilatation/erosion_dilatation.html#dilation)
/// on a `Vec2d`
pub fn dilate (input: &Vec2d, kernel: &Vec2d) -> Vec2d {
    let mut output = Vec2d::empty(input.width, input.height);
    for x in 1..input.width {
        for y in 1..output.height {
            output.data[x + y * output.width] = check_neighbours(&input, &kernel, x, y, true);
        }
    }
    output
}

/// Performs an [erosion](https://docs.opencv.org/2.4/doc/tutorials/imgproc/erosion_dilatation/erosion_dilatation.html#erosion)
/// on a `Vec2d`
pub fn erode (input: &Vec2d, kernel: &Vec2d) -> Vec2d {
    let mut output = Vec2d::empty(input.width, input.height);
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
    let mut input = Vec2d::empty(10, 20);
    input.data[44] = true;
    let output = dilate(&input, &kernel);
    let reversed = erode(&output, &kernel);
    assert!(!input.data.iter().eq(output.data.iter()));
    assert!(input.data.iter().eq(reversed.data.iter()), "for the simple case reverse is the same as the input");
}

pub struct BinaryImage {
    pub width: usize,
    pub height: usize,
    pub data: Vec<bool>
}

impl BinaryImage {
    fn new (width: usize, height: usize) -> Self {
        BinaryImage {
            data: Vec::with_capacity(width * height),
            width: width,
            height: height
        }
    }
}

// fn dilate () {
//
// }


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

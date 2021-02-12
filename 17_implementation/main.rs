struct Rect {
    width: i32,
    height: i32
}

impl Rect {
    fn print_description(&self) {
        println!("Rectangle {} x {}", self.width, self.height);
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }

}

fn main() {
    let my_rect = Rect {width: 10, height: 60};

    my_rect.print_description();

    println!("Rectangle is a square ? >> {}", my_rect.is_square());

}
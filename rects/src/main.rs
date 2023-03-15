#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rect::new(30, 50);
    let area = rect.area();
    // println!("{:#?}", rect);
    // dbg!(&rect);
    println!("{:?} {}", &rect, area);

    let rect2 = Rect::new(10, 40);
    let rect3 = Rect::new(60, 45);

    println!("Can rect1 hold rect2? {}", rect.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect.can_hold(&rect3));

    dbg!(Rect::square(10));
}

impl Rect {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    fn square(size: u32) -> Self {
        Self::new(size, size)
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Self) -> bool {
        (self.width >= other.width && self.height >= other.height)
            || (self.width >= other.height && self.height >= other.width)
    }
}

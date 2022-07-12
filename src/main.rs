pub trait Graphics {
    fn area(&self) -> f64;
}

fn do_area<T: Graphics>(signle_graphic: T) -> f64 {
    signle_graphic.area()
}
// 长方形
struct Rectangle {
    width: f64,
    height: f64,
}

impl Graphics for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl Rectangle {
    pub fn new(x: f64, y: f64) -> Rectangle {
        Rectangle {
            width: x,
            height: y,
        }
    }
}

// 三角形
struct Triangle {
    base: f64,
    height: f64,
}

impl Graphics for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

impl Triangle {
    fn new(x: f64, y: f64) -> Triangle {
        Triangle { base: x, height: y }
    }
}

fn main() {
    // 计算两个图形的面积
    let r1 = Rectangle::new(12.0, 13.0);
    let t1 = Triangle::new(9.2, 6.8);
    println!("the rectangle area is  {}", do_area(r1));
    println!("the triangle area is {}", do_area(t1));
}

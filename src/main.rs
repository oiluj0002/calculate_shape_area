struct Vertices(u32, u32);

struct Rectangle {
    width: u32,
    height: u32,
}

fn calculate_distance(Vertices(a, b): Vertices) -> u32 {
    let distance = a - b;

    return distance;
}

fn calculate_rectangle_area(Rectangle{width, height}: &Rectangle) -> u32 {
    let area = width * height;

    return area;
}

fn main() {
    let point = Point()
    let shape = Rectangle { width: 20, height: 30 };
    let area = calculate_rectangle_area(&shape);

    println!("Area = {:?}, Width = {:?}, Height = {:?}", area, shape.width, shape.height);
}

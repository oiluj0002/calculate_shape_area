struct Vertices(u32, u32);

struct Rectangle {
    width: u32,
    height: u32,
}

fn calculate_distance(Vertices(a, b): Vertices) -> u32 {
    let distance = a.abs_diff(b);

    return distance;
}

fn calculate_rectangle_area(Rectangle { width, height }: &Rectangle) -> u32 {
    let area = width * height;

    return area;
}

fn main() {
    let pair1 = Vertices(42, 4);
    let pair2 = Vertices(6, 87);

    let width = calculate_distance(pair1);
    let height = calculate_distance(pair2);

    let shape = Rectangle { width, height };
    let area = calculate_rectangle_area(&shape);

    println!(
        "Width = {:?}, Height = {:?}, Area = {:?}",
        shape.width, shape.height, area
    );
}

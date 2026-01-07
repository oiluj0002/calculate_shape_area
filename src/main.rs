struct Vertices(u32, u32);

fn calculate_distance(Vertices(a, b): Vertices) -> u32 {
    let distance = a.abs_diff(b);

    return distance;
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        let area = self.width * self.height;

        return area;
    }
}

fn main() {
    let pair1 = Vertices(42, 4);
    let pair2 = Vertices(6, 87);

    let width = calculate_distance(pair1);
    let height = calculate_distance(pair2);

    let shape = Rectangle { width, height };
    let area = shape.area();

    println!(
        "Width = {:?}, Height = {:?}, Area = {:?}",
        shape.width, shape.height, area
    );
}

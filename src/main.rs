struct Color(i32, i32, i32); // These are examples of tuple structs
struct Point(i32, i32, i32);

struct AlwaysEqual; // This is an example of a unit struct; behaves similarly to a tuple struct with no elements

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;
}

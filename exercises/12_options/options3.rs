// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    // if the current second is even, then we will have Some(Point { x: 100, y: 200 }) otherwise None
    let y: Option<Point>;
    if std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        % 2
        == 0
    {
        y = Some(Point { x: 100, y: 200 });
    } else {
        y = None;
    }

    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => println!("No co-ordinates"),
    }
    y; // Fix without deleting this line.
}

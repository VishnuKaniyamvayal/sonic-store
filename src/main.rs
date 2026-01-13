#[derive(Debug)]
enum Shape{
    Circle(u64),
    Square(u64),
    Rectangle(u64),
}


fn main(){
    let circle = Shape::Circle(56);
    let square = Shape::Square(56);
    let rectangle = Shape::Rectangle(56);

    println!("{:?}",circle);
    println!("{:?}",square);
    println!("{:?}",rectangle);
}
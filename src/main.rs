use chapters::structs::Color;
use crate::chapters::structs::{adding_color, build_color, Point2D};

mod chapters;

fn main() {
    let mut rfan = Color{
        red: 23,
        green:53,
        blue:33
    };
    let garden = Point2D {
        x: 3.2,
        y: 5.2,
        carte: 's'
    };
    
    rfan.blue = 7;
    let fuchsia = Color{
        red: 33,
        green:2,
        ..rfan
    };

    let fuchsia3 = Color{
        red: 33,
        green:2,
        ..rfan
    };
    
    println!("Sum of Color ( rfan ) and Color ( fuchsia ) is equal with {}",adding_color(&rfan, &fuchsia));
    println!("fan{}", rfan.blue);
    println!("fuchsia {}", fuchsia.blue);
    
    println!("{}", build_color(200,200).red);
}

// 1 1 2 3
fn main() {

    let tri_1 = area_of_triangle(60.0,30.0);

    println!("The area of tri1: {}.", tri_1);
    
    //I forgot how to properly right a tuple so I had to review the code on the data_type lesson.
    let tri_2:(f32,f32) = (120.0,60.0);

    let tri_2_area = area_of_triangle_2(tri_2);

    println!("The area of the tri2: {}.", tri_2_area);

    let tri_3 = Triangle{
        base: 240.0,
        height: 120.0,
    }; //Create an instance/ object of the Triangle Struct.

    dbg!(&tri_3); //debug macro which will dislpays the struct as a debug process.

    let tri_3_area =  area_of_triangle_3(&tri_3);

    println!("This is the area of tri3: {}.", tri_3_area);

}


fn area_of_triangle(base:f32,height:f32) -> f32{
    (0.5) * base * height // This is the same return (0.5)*base*height;
}

// I forgot how to create a function that accepts a tuple as a paramater and needed to check the code on struct lesson.
fn area_of_triangle_2(dimensions:(f32,f32)) -> f32{
    return (0.5) * dimensions.0 * dimensions.1; // (0.5)*dimension.0*dimension.1 is the same.
}

#[derive(Debug)] //Debug annotation.
struct Triangle{
    base: f32,
    height: f32,
} //Create a blueprint called Triangle with fields called  base and height

fn area_of_triangle_3(triangle:&Triangle) -> f32{
    (0.5)*triangle.base*triangle.height 
}

// Seems I was able to recall most of everything.
// My execution on writing tuples is still poor.
// Everything else is more or repeat and repetition to get things down a little better.

fn main() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", xs);

    for x in &xs {
        x = &0;
        print!("{} ", x);
    }

    for x in xs {
        print!("{} ", x);
    }



}

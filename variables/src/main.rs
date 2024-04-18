fn main() {
    let mut missels = 8;
    let tst :f32 = 2.0;

    println!("Hello, world!");
    println!("First Variable: {}", missels);
    println!("Second Variable: {}", tst);

    missels = missels -tst;
    println!("First Variable after update: {}", missels);

}

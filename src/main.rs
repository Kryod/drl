mod policies;
mod line_world;

fn main() {
    let (S, A, T, P, R) = line_world::init();

    println!("{}", S);
    println!("{}", A);
    println!("{}", T);
    println!("{}", P);
    println!("{}", R);
}

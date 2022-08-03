mod generator;
pub fn printrandom_number() {
    let n = generator::gen_ran();
    println!("Random u8: {}", n);
}
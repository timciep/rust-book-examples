fn main() {
    let s1 = String::from("hello");

    // "move"
    let s2 = s1;

    // s1 no longer exists, so this would not work:
    // println!("{}, world!", s1);

    println!("{}, world!", s2);

    let s3 = s2.clone();

    println!("{}, world!", s3);
}

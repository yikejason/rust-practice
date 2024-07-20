fn main() {
    println!("Hello {:5}!", "x"); // =>  "Hello x    !"
    println!("Hello {:1$}!", "x", 5); // =>  "Hello x    !"

    /* 填空 */
    assert_eq!(format!("Hello {}{:5}!", 5, "x"), "Hello x    !");
    assert_eq!(
        format!("Hello {}{width:width$}!", "x", width = 5),
        "Hello x    !"
    );

    println!("Success!")
}

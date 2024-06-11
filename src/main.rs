fn main() {
    println!("Generating a dungeon for you my lord");

    let width = 80;
    let height = 50;

    let header = "#".repeat(width);
    let inside = " ".repeat(width - 2);
    let inside2 = format!("#{inside}#");

    // Print the enclosing walls
    println!("{}", header);
    for _i in 0..height - 2 {
        println!("{}", inside2);
    }
    println!("{}", header);

}

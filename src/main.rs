pub trait Room {
    fn to_coords(&self) -> Vec<Coord>;
}

#[derive(Debug, Eq, PartialEq)]
pub struct Coord {
    x: i32,
    y: i32, // Recall that smaller y means more at the top.
}

impl Coord {
    /// Creates a new [`Coord`].
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Room for Coord {
    fn to_coords(&self) -> Vec<Coord> {
        let mut v = Vec::new();
        let copy = Coord::new(self.x, self.y);
        v.push(copy);
        return v;
    }
}

#[cfg(test)]
mod tests {
    use crate::Coord;
    use crate::Room;

    #[test]
    fn coord_to_coords() {
        let c = Coord::new(0, 0);
        let coords = c.to_coords();
        let coords_head = coords.get(0);
        match coords_head {
            None => panic!("Coord.to_coords should return a non-empty list"),
            Some(head) => assert_eq!(c, *head),
        }
    }
}

mod proptests {
    use proptest::prelude::*;
    use crate::Coord;
    use crate::Room;

    proptest! {
        #[test]
        fn coord_to_coords(x in 0..256, y in 0..256) {
            let c = Coord::new(x, y);
            let coords = c.to_coords();
            let coords_head = coords.get(0);
            match coords_head {
                None => panic!("Coord.to_coords should return a non-empty list"),
                Some(head) => assert_eq!(c, *head),
            }
        }
    }
}

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

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

pub struct Rect {
    left_x: i32,
    top_y: i32,
    width: u32,
    height: u32,
}

impl Room for Rect {
    fn to_coords(&self) -> Vec<Coord> {
        if self.width == 0 || self.height == 0 {
            return Vec::new(); // TODO would be an immutable empty singleton be possible here?
        } else {
            let mut v = Vec::new();
            for x_shift in 0..self.width {
                for y_shift in 0..self.height {
                    let bigger_x = self.left_x.checked_add_unsigned(x_shift);
                    let bigger_y = self.top_y.checked_add_unsigned(y_shift);
                    match (bigger_x, bigger_y) {
                        (Some(bigger_x), Some(bigger_y)) => v.push(Coord::new(bigger_x, bigger_y)),
                        _ => (), // Don't extend on overflows
                    }
                }
            }
            return v;
        }
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
    use crate::Coord;
    use crate::Rect;
    use crate::Room;
    use proptest::prelude::*;

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

        #[test]
        fn rect_to_coords_size(width in 0..256u32, height in 0..256u32, left_x in 0..256, top_y in 0..256) { // Small sizes, because we don't want to run into overflows in Rect::to_coords()
            let r = Rect{left_x, top_y, width, height};
            let coords = r.to_coords();
            match usize::try_from(r.width * r.height) {
                Ok(area_size) => assert_eq!(coords.len(), area_size),
                Err(_) => panic!("Unexpected conversion failure"),
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

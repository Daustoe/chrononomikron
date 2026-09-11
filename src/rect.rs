use std::collections::HashSet;

/// Rect object that represents a rectangle on a grid using integer coordinates.
/// 
/// The rectangle is defined by its minimum and maximum coordinates represented
/// by [x1, y1] and [x2, y2] positional coordinates.
#[derive(Clone, Copy, PartialEq)]
pub struct Rect {
    /// The leftmost x-coordinate.
    pub x1 : i32, 
    /// The rightmost x-coordinate.
    pub x2 : i32,
    /// The uppermost y-coordinate.
    pub y1 : i32,
    /// The lowermost y-coordinate. 
    pub y2 : i32
}

impl Rect {
    /// Creates a new Rect from a point at the top left and a size.
    /// 
    /// # Arguments
    /// - `x` - The x-coordinate of the starting position.
    /// - `y` - The y-coordinate of the starting position.
    /// - `w` - Width of the rectangle in Tiles.
    /// - `h` - Height of the rectangle in Tiles.
    /// 
    /// # Examples
    ///
    /// ```
    /// let rect = Rect::new(10, 20, 30, 40);
    ///
    /// assert_eq!(rect.x1, 10);
    /// assert_eq!(rect.y1, 20);
    /// assert_eq!(rect.x2, 40);
    /// assert_eq!(rect.y2, 60);
    /// ```
    pub fn new(x:i32, y:i32, w:i32, h:i32) -> Rect {
        Rect{x1:x, y1:y, x2:x+w, y2:y+h}
    }

    /// Determines whether two Rect objects overlap. 
    /// 
    /// # Arguments
    /// `other` - The other Rect object to compare against.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let a = Rect::new(0, 0, 10, 10);
    /// let b = Rect::new(5, 5, 10, 10);
    /// let c = Rect::new(20, 20, 5, 5);
    ///
    /// assert!(a.intersect(&b));
    /// assert!(!a.intersect(&c));
    /// ```
    pub fn intersect(&self, other:&Rect) -> bool {
        self.x1 <= other.x2 && 
        self.x2 >= other.x1 &&
        self.y1 <= other.y2 &&
        self.y2 >= other.y1
    }

    /// Returns the center of the Rect as an (x, y) tuple.
    /// 
    /// Integer division is used here as we can't return a half position. As a result
    /// round down when a coordinate falls between two integers.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let rect = Rect::new(0, 0, 10, 20);
    /// 
    /// assert_eq!(rect.center(), (5, 10));
    /// ```
    pub fn center(&self) -> (i32, i32) {
        ((self.x1 + self.x2)/2, (self.y1 + self.y2)/2)
    }

    /// Returns all tile coordinate pairs that are contained fully within the Rect.
    /// 
    /// The returned [`HashSet`] contains `(x, y)` coordinates. 
    /// 
    /// The boundaries are **Exclusive** and will not be part of the returend [`HashSet`]
    /// 
    /// # Examples
    /// ```
    /// let rect = Rect::new(0, 0, 3, 2);
    /// let tiles = rect.get_all_tiles();
    ///
    /// assert_eq!(tiles.len(), 6);
    /// assert!(tiles.contains(&(0, 0)));
    /// assert!(tiles.contains(&(2, 1)));
    /// assert!(!tiles.contains(&(3, 1)));
    /// ```
    pub fn get_all_tiles(&self) -> HashSet<(i32, i32)> {
        let mut result = HashSet::new();
        for y in self.y1 .. self.y2 {
            for x in self.x1 .. self.x2 {
                result.insert((x, y));
            }
        }
        result
    }
}
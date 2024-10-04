/*
** src/types.rs
*/

pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn new<T>(x: T, y: T) -> Self
    where
        T: Into<i64>,
    {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }
}

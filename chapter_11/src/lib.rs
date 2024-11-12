pub fn add(left: usize, right:usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    /* 
    #[test]
    fn it_works() {
        let result = add (2,4);
        assert_eq!(result, 6);
    }

    #[test]
    fn another()
    {
        panic!("Whoops!")
    }
    */

    #[test]
    fn larger_can_hold_smaller() {
        let smaller = Rectangle{
            width: 3, height: 6};
        let larger = Rectangle{
            width: 6,
            height: 12
        };

        assert!(larger.can_hold(&smaller))
    }
    #[test]
    fn smaller_cant_hold_larger() {
        let smaller = Rectangle{
            width: 3, height: 6};
        let larger = Rectangle{
            width: 6,
            height: 12
        };

        assert!(!smaller.can_hold(&larger))
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}


// TODO: Implement the `From` trait for the `WrappingU32` type to make `example` compile.

#[derive(Debug)]
pub struct WrappingU32 {
    value: u32,
}

impl From<u32> for WrappingU32 {
    fn from(value: u32) -> Self {
        WrappingU32 { value }
    }
}

fn example() {
    let wrapping: WrappingU32 = 42.into();
    dbg!(wrapping);
    let wrapping = WrappingU32::from(42);
    dbg!(wrapping);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example() {
        example();
    }

}
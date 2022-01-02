fn factorial(n: u32) -> u32 {
    if n == 1 {
        return 1;
    }

    n * factorial(n - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_1() {
        assert_eq!(1, factorial(1));
    }

    #[test]
    fn two_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn three_6() {
        assert_eq!(6, factorial(3));
    }

    #[test]
    fn four_24() {
        assert_eq!(24, factorial(4));
    }

    #[test]
    fn five_120() {
        assert_eq!(120, factorial(5));
    }
}

pub fn main() {}

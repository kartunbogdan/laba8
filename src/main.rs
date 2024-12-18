pub struct Algorithms;

impl Algorithms {
    pub fn find_min_positive(arr: &[u32]) -> Option<u32> {
        arr.iter().cloned().min()
    }

    pub fn sum_negative(arr: &[i32]) -> i32 {
        arr.iter().filter(|&&x| x < 0).sum()
    }

    pub fn fibonacci(n: u32) -> u64 {
        match n {
            0 => 0,
            1 => 1,
            _ => Algorithms::fibonacci(n - 1) + Algorithms::fibonacci(n - 2),
        }
    }

    pub fn current(voltage: f64, resistance: f64) -> Result<f64, &'static str> {
        if resistance == 0.0 {
            Err("Resistance cannot be zero")
        } else {
            Ok(voltage / resistance)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_min_positive() {
        assert_eq!(Algorithms::find_min_positive(&[3, 7, 1, 9]), Some(1));
        assert_eq!(Algorithms::find_min_positive(&[100]), Some(100));
        assert_eq!(Algorithms::find_min_positive(&[]), None); // Negative test
    }

    #[test]
    fn test_sum_negative() {
        assert_eq!(Algorithms::sum_negative(&[-5, -3, -1]), -9);
        assert_eq!(Algorithms::sum_negative(&[0, -2, 3]), -2);
        assert_eq!(Algorithms::sum_negative(&[1, 2, 3]), 0); // Negative test
    }

    #[test]
    fn test_fibonacci() {
        assert_eq!(Algorithms::fibonacci(0), 0);
        assert_eq!(Algorithms::fibonacci(1), 1);
        assert_eq!(Algorithms::fibonacci(10), 55);
        assert_eq!(Algorithms::fibonacci(15), 610);
    }

    #[test]
    fn test_current() {
        assert_eq!(Algorithms::current(10.0, 5.0), Ok(2.0));
        assert_eq!(Algorithms::current(0.0, 10.0), Ok(0.0));
        assert_eq!(Algorithms::current(10.0, 0.0), Err("Resistance cannot be zero")); // Negative test
    }
}


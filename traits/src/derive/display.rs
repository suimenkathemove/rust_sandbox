//! # Display

#[cfg(test)]
mod tests {
    use std::fmt::Display;

    #[test]
    fn test() {
        struct Complex {
            real: f64,
            imaginary: f64,
        }

        impl Display for Complex {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{} + {}i", self.real, self.imaginary)
            }
        }

        let complex = Complex {
            real: 1.,
            imaginary: 2.,
        };
        println!("{}", complex);
    }
}

use std::io;

/// A simple function that prints "Hello, world!" to the console.
pub fn hello_world(output: &mut dyn io::Write) -> io::Result<()> {
    writeln!(output, "Hello, world!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_world() {
        let mut output = Vec::new();
        hello_world(&mut output).unwrap();
        assert_eq!(output, b"Hello, world!\n");
    }
}

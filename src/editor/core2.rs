#[derive(PartialEq, Debug)]
pub struct Core2 {
    buffer: Vec<Vec<char>>,
    line: usize,
    column: usize,
}

impl Core2 {
    pub fn new(buffer: &str, line: usize, column: usize) -> Result<Core2, String> {
        let buf: Vec<Vec<char>> = buffer.lines().map(|l| l.chars().collect()).collect();

        if buf.len() <= line {
            return Err(format!(
                "Line {} is out of range [0, {})",
                line,
                buf.len()
            ));
        }
        let width = buf[line].len();
        if width < column {
            return Err(format!("Column {} is out of range [0, {}]", column, width));
        }
        Ok(Core2 {
            buffer: buf,
            line,
            column,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let buffer = "aa aa";
        let editor = Core2::new(buffer, 0, 0);
        assert!(editor.is_ok());

        let editor = Core2::new(buffer, 0, 6);
        assert!(editor.is_err());
    }
}

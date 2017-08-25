struct Iterator2d<'a> {
    iter: &'a [Vec<char>],
    line: usize,
    column: usize,
}

impl<'a> Iterator2d<'a> {
    fn new(vec: &[Vec<char>]) -> Iterator2d {
        Iterator2d {
            iter: vec,
            line: 0,
            column: 0,
        }
    }

    fn skip(&mut self, line: usize, column: usize) {
        self.line = line;
        self.column = column;
    }

    fn position<P>(&mut self, mut predicate: P) -> Option<(usize, usize)> where P: FnMut(char) -> bool,
    {
        while let Some(l) = self.iter.get(self.line) {
            while let Some(x) = l.get(self.column) {
                if predicate(x.clone()) {
                    return Some((self.line, self.column));
                }
                self.column += 1;
            }
            self.line += 1;
            self.column = 0;
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterator2d() {
        let vec = vec![vec!['a', 'b'], vec!['c', 'd', 'e', 'f']];
        let mut it = Iterator2d::new(&vec);
        it.skip(1, 1);
        assert_eq!(it.position(|x| x == 'e'), Some((1, 2)));
    }
}

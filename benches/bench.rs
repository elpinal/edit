#![feature(test)]

extern crate test;

extern crate edit;

#[cfg(test)]
mod tests {
    use test::Bencher;
    use edit::editor::*;


    #[bench]
    fn bench_move_right(b: &mut Bencher) {
        let mut editor = Editor::new(&"abcdef".repeat(10000), 0, 0).unwrap();
        b.iter(|| editor.move_right(10));
    }

    #[bench]
    fn bench_insert_at(b: &mut Bencher) {
        let buffer = &"abcdef".repeat(10000);
        let editor = Editor::new(buffer, 0, 0).unwrap();
        b.iter(|| {
            let mut ed = editor.clone();
            ed.insert_at('x', 0, 500)
        });
    }

    fn bench_insert_string_at_n(b: &mut Bencher, s: &str, n: usize) {
        let buffer = &"abcdef".repeat(10000);
        let editor = Editor::new(buffer, 0, 0).unwrap();
        b.iter(|| {
            let mut ed = editor.clone();
            ed.insert_string_at(&s.repeat(n), 0, 500)
        });
    }

    #[bench]
    fn bench_insert_string_at_1(b: &mut Bencher) {
        bench_insert_string_at_n(b, "x", 1);
    }

    #[bench]
    fn bench_insert_string_at_2(b: &mut Bencher) {
        bench_insert_string_at_n(b, "x", 2);
    }

    #[bench]
    fn bench_insert_string_at_4(b: &mut Bencher) {
        bench_insert_string_at_n(b, "x", 4);
    }

    #[bench]
    fn bench_insert_string_at_8(b: &mut Bencher) {
        bench_insert_string_at_n(b, "x", 8);
    }

    #[bench]
    fn bench_insert_string_at_16(b: &mut Bencher) {
        bench_insert_string_at_n(b, "x", 16);
    }

    #[bench]
    fn bench_insert_string_at_256(b: &mut Bencher) {
        bench_insert_string_at_n(b, "x", 256);
    }

    #[bench]
    fn bench_insert_string_at_1024(b: &mut Bencher) {
        bench_insert_string_at_n(b, "x", 1024);
    }


    #[bench]
    fn bench_insert_string_at_1_with_newline(b: &mut Bencher) {
        bench_insert_string_at_n(b, "x\ny", 1);
    }

    #[bench]
    fn bench_insert_string_at_2_with_newline(b: &mut Bencher) {
        bench_insert_string_at_n(b, "x\ny", 2);
    }

    #[bench]
    fn bench_insert_string_at_4_with_newline(b: &mut Bencher) {
        bench_insert_string_at_n(b, "x\ny", 4);
    }

    #[bench]
    fn bench_insert_string_at_8_with_newline(b: &mut Bencher) {
        bench_insert_string_at_n(b, "x\ny", 8);
    }

    #[bench]
    fn bench_insert_string_at_16_with_newline(b: &mut Bencher) {
        bench_insert_string_at_n(b, "x\ny", 16);
    }

    #[bench]
    fn bench_buffer(b: &mut Bencher) {
        let buffer: Vec<char> = "abcdef".repeat(10000).chars().collect();
        b.iter(|| {
            let mut buf = buffer.clone();
            buf.insert(3, 'x')
        });
    }
}

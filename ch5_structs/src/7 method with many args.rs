struct Line {
    start: i64,
    end: i64,
}

impl Line {
    fn length(&self) -> u64 {
        (self.end - self.start).abs() as u64
    }

    fn is_subline(&self, another: &Line) -> bool {
        self.start >= another.start && self.end <= another.end
    }
}

fn main() {
    let line1 = Line { start: -5, end: 4 };

    let line2 = Line { start: -4, end: 4 };

    let line3 = Line { start: 0, end: 5 };

    println!("Line 2 is subline of Line 1 - {}", line2.is_subline(&line1));
    println!("Line 3 is subline of Line 1 - {}", line3.is_subline(&line1));

    println!("{}", line1.length());
}

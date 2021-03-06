pub struct StringBuilder {
    buff: Vec<Line>,
    curr_indent: usize,
}

impl StringBuilder {
    pub fn new(curr_indent: usize) -> Self {
        Self {
            buff: vec![],
            curr_indent,
        }
    }

    pub fn add_line(mut self, line: String) -> Self {
        let line = Line::new(self.curr_indent, line);
        self.buff.push(line);
        self
    }

    pub fn build_string(self, ch: char) -> String {
        self.buff
            .into_iter()
            .fold(String::new(), |acc, curr| acc + &curr.to_line(ch) + "\n")
    }

    pub fn increase_indent(mut self) -> Self {
        self.curr_indent += 1;
        self
    }

    pub fn decrease_indent(mut self) -> Self {
        self.curr_indent -= 1;
        self
    }
}

struct Line {
    indent: usize,
    code: String,
}

impl Line {
    fn new(indent: usize, code: String) -> Self {
        Self { indent, code }
    }

    fn to_line(self, ch: char) -> String {
        let indent = (0..self.indent)
            .map(|_| ch)
            .fold(String::new(), |mut acc, c| {
                acc.push(c);
                acc
            });
        indent + &self.code
    }
}

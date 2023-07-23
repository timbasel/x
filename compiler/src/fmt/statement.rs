use super::*;

impl Format for Statement {
    fn fmt(&self, fmt: &mut Formatter) -> String {
        use StatementKind::*;
        return format!(
            "{}{}",
            fmt.indent(),
            match &self.kind {
                Comment(c) => c.fmt(fmt),
                Declaration(d) => d.fmt(fmt),
                Assignment(a) => a.fmt(fmt),
                Block(b) => b.fmt(fmt),
                FunctionDeclaration(f) => f.fmt(fmt),
                Return(r) => r.fmt(fmt),
                Expression(e) => e.fmt(fmt),
            }
        );
    }
}

impl Format for Vec<Box<Statement>> {
    fn fmt(&self, fmt: &mut Formatter) -> String {
        let mut buffer = String::new();

        let mut iter = self.iter().peekable();
        while let Some(statement) = iter.next() {
            buffer.push_str(&statement.fmt(fmt));

            if let Some(next) = iter.peek() {
                if matches!(next.kind, StatementKind::Comment(_))
                    && next.position.start.line == statement.position.end.line
                {
                    // keep comments on the same line as the statement
                    buffer.push_str(" ")
                } else if next.position.start.line - statement.position.end.line >= 2 {
                    // keep a maximum of one empty line
                    buffer.push_str("\n\n");
                } else {
                    buffer.push_str("\n");
                }
            }
        }

        return buffer;
    }
}

impl Format for Comment {
    fn fmt(&self, _fmt: &mut Formatter) -> String {
        if self.inline {
            return format!("//{}", self.comment);
        } else {
            return format!("/*{}*/", self.comment);
        }
    }
}

impl Format for Declaration {
    fn fmt(&self, fmt: &mut Formatter) -> String {
        let mut buffer = String::new();

        if self.mutable {
            buffer.push_str("mut ")
        }
        buffer.push_str(&format!("{} := {}", self.name, self.value.fmt(fmt)));

        return buffer;
    }
}

impl Format for Assignment {
    fn fmt(&self, fmt: &mut Formatter) -> String {
        return format!("{} = {}", self.name, self.value.fmt(fmt));
    }
}

impl Format for Block {
    fn fmt(&self, fmt: &mut Formatter) -> String {
        let mut buffer = String::new();

        buffer.push_str("{\n");
        fmt.push_indent();
        buffer.push_str(&self.statements.fmt(fmt));
        fmt.pop_indent();
        buffer.push_str("\n}");

        return buffer;
    }
}

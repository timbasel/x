use super::*;

impl Format for Expression {
    fn fmt(&self, fmt: &mut Formatter) -> String {
        use ExpressionKind::*;

        return match &self.kind {
            Identifier(i) => format!("{}", i),
            Boolean(b) => format!("{}", b),
            Integer(i) => format!("{}", i),
            Float(f) => format!("{}", f),
            String(s) => format!("\"{}\"", s),

            Prefix(p) => p.fmt(fmt),
            Infix(i) => i.fmt(fmt),
        };
    }
}

impl Format for PrefixExpression {
    fn fmt(&self, fmt: &mut Formatter) -> String {
        return format!("{}{}", self.operator, self.right.fmt(fmt));
    }
}

impl Format for InfixExpression {
    fn fmt(&self, fmt: &mut Formatter) -> String {
        return format!(
            "{} {} {}",
            self.left.fmt(fmt),
            self.operator,
            self.right.fmt(fmt)
        );
    }
}

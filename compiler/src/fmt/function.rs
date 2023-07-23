use super::*;

impl Format for FunctionDeclaration {
    fn fmt(&self, fmt: &mut Formatter) -> String {
        let mut buffer = String::new();

        buffer.push_str(&format!(
            "fn {}({})",
            self.name,
            &fmt.join(self.typ.parameters.iter(), ", ")
        ));
        if let Some(return_type) = &self.typ.return_type {
            buffer.push_str(&format!(": {}", return_type.fmt(fmt)))
        }
        buffer.push_str(" ");
        buffer.push_str(&self.body.fmt(fmt));

        return buffer;
    }
}

impl Format for Return {
    fn fmt(&self, fmt: &mut Formatter) -> String {
        return format!("return {}", self.value.fmt(fmt));
    }
}
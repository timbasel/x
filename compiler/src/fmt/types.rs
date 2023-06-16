use super::*;

impl Format for Type {
    fn fmt(&self, fmt: &mut Formatter) -> String {
        return match self {
            Type::Unknown => "UNKNOWN".into(),
            Type::Name(name) => name.into(),

            Type::Boolean => "bool".into(),
            Type::Integer => "int".into(),
            Type::Float => "float".into(),
            Type::String => "string".into(),

            Type::Tuple(t) => t.fmt(fmt),
            Type::Struct(s) => s.fmt(fmt),
            Type::Function(f) => f.fmt(fmt),
        };
    }
}

impl Format for TupleType {
    fn fmt(&self, fmt: &mut Formatter) -> String {
        return format!("({})", fmt.join(self.elements.iter(), ", "));
    }
}

impl Format for StructType {
    fn fmt(&self, fmt: &mut Formatter) -> String {
        return format!("{{ {} }}", fmt.join(self.fields.iter(), ", "));
    }
}

impl Format for Field {
    fn fmt(&self, fmt: &mut Formatter) -> String {
        let mut buffer = String::new();

        buffer.push_str(&format!("{}: {}", self.name, self.typ.fmt(fmt)));
        if let Some(default) = &self.default {
            buffer.push_str(&format!(" = {}", default.fmt(fmt)))
        }

        return buffer;
    }
}

impl Format for FunctionType {
    fn fmt(&self, fmt: &mut Formatter) -> String {
        let mut buffer = String::new();

        buffer.push_str(&format!("fn ({})", fmt.join(self.parameters.iter(), ", ")));
        if let Some(return_type) = &self.return_type {
            buffer.push_str(&format!(": {}", return_type.fmt(fmt)));
        }

        return buffer;
    }
}

impl Format for Parameter {
    fn fmt(&self, fmt: &mut Formatter) -> String {
        return format!("{}: {}", self.name, self.typ.fmt(fmt));
    }
}

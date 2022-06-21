use crate::types::Type;

pub struct Struct {
    name: String,
    members: Vec<(String, Type)>,
    semantics: Option<Vec<String>>,
}

impl Struct {
    pub fn new(name: String, members: Vec<(String, Type)>, semantics: Option<Vec<String>>) -> Self {
        Struct {
            name,
            members,
            semantics,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn members(&self) -> &[(String, Type)] {
        &self.members
    }

    pub fn semantics(&self) -> &[String] {
        self.semantics.as_ref().unwrap()
    }

    pub fn generate_hlsl(&self) -> String {
        let mut struct_hlsl = format!("struct {} {{\n", self.name);
        let mut constructor_declaration_hlsl = format!("{} acsl_create_{}(", self.name, self.name);
        let mut constructor_body_hlsl = format!("    {} output;\n", self.name);

        let mut i = 0;
        for (name, member_type) in &self.members {
            struct_hlsl.push_str(&format!("    {} {}", member_type.hlsl(), name));
            constructor_body_hlsl.push_str(&format!("    output.{} = {};\n", name, name));
            constructor_declaration_hlsl.push_str(&format!("{} {}", member_type.hlsl(), name));
            if i != self.members.len() - 1 {
                constructor_declaration_hlsl.push_str(", ");
            }

            match &self.semantics {
                Some(semantics) => struct_hlsl.push_str(&format!(": {}", semantics[i])),
                None => {}
            }

            struct_hlsl.push_str(";\n");

            i += 1;
        }

        struct_hlsl.push_str("};\n");
        constructor_declaration_hlsl.push_str(") {\n");
        constructor_body_hlsl.push_str("    return output;\n}\n");

        format!(
            "{}{}{}",
            struct_hlsl, constructor_declaration_hlsl, constructor_body_hlsl
        )
    }

    pub fn generate_glsl(&self) -> String {
        let mut glsl = format!("struct {} {{\n", self.name);

        for i in 0..self.members.len() {
            glsl.push_str(&format!(
                "    {} {};\n",
                self.members[i].1.glsl(),
                self.members[i].0
            ));
        }

        glsl.push_str("};\n");

        glsl
    }
}

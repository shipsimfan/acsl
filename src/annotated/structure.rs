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

    pub fn generate_hlsl(&self) -> String {
        let mut hlsl = format!("struct {} {{\n", self.name);

        let mut i = 0;
        for (name, member_type) in &self.members {
            hlsl.push_str(&format!("    {} {}", member_type.hlsl(), name));

            match &self.semantics {
                Some(semantics) => hlsl.push_str(&format!(": {}", semantics[i])),
                None => {}
            }

            hlsl.push_str(";\n");

            i += 1;
        }

        hlsl.push_str("};\n");

        hlsl
    }
}
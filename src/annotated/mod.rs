use crate::{
    ast::{scope::Scope, SemanticAnalysisError},
    types::Type,
};
use constant_buffer::ConstantBuffer;
use function::Function;
use std::{collections::VecDeque, rc::Rc};
use structure::Struct;
use texture::Texture;

pub mod code_block;
pub mod constant_buffer;
pub mod expression;
pub mod function;
pub mod statement;
pub mod structure;
pub mod texture;

enum DeclarationType {
    Function,
    Struct,
    ConstantBuffer(usize),
    Texture(usize),
}

pub struct AnnotatedSyntaxTree {
    functions: VecDeque<Function>,
    structs: VecDeque<Rc<Struct>>,

    constant_buffers: Box<[Option<ConstantBuffer>]>,
    textures: Box<[Option<Texture>]>,

    declaration_order: Vec<DeclarationType>,

    builtin_functions: Box<[Function]>,

    type_aliases: Vec<(String, Type)>,
    global_scope: Scope,

    vertex_input_type: Option<Type>,
    fragment_input_type: Option<Type>,
}

pub const MAX_CONSTANT_BUFFERS: usize = 32;
pub const MAX_TEXTURES: usize = 8;

pub const CONSTANT_BUFFER_INDEX: usize = 0;
pub const TEXTURES_INDEX: usize = MAX_CONSTANT_BUFFERS;

impl AnnotatedSyntaxTree {
    pub fn new() -> Self {
        AnnotatedSyntaxTree {
            functions: VecDeque::new(),
            structs: VecDeque::new(),
            constant_buffers: vec![None; MAX_CONSTANT_BUFFERS].into_boxed_slice(),
            textures: vec![None; MAX_TEXTURES].into_boxed_slice(),
            declaration_order: Vec::new(),
            builtin_functions: Function::builtin_functions(),
            type_aliases: Vec::new(),
            global_scope: Scope::new(),
            vertex_input_type: None,
            fragment_input_type: None,
        }
    }

    pub fn verify_graphics_functions(&self) -> Result<(), SemanticAnalysisError> {
        // Verify the functions have been added
        if self.vertex_input_type.is_none() {
            return Err(SemanticAnalysisError::NoVertexMain);
        } else if self.fragment_input_type.is_none() {
            return Err(SemanticAnalysisError::NoFragmentMain);
        }

        Ok(())
    }

    pub fn get_function(&self, name: &str) -> Result<&Function, SemanticAnalysisError> {
        for function in &self.functions {
            if function.name() == name {
                return Ok(function);
            }
        }

        for function in self.builtin_functions.iter() {
            if function.name() == name {
                return Ok(function);
            }
        }

        Err(SemanticAnalysisError::UnknownFunction(name.to_owned()))
    }

    pub fn get_structure(&self, name: &str) -> Result<&Struct, SemanticAnalysisError> {
        for structure in &self.structs {
            if structure.name() == name {
                return Ok(&structure);
            }
        }

        Err(SemanticAnalysisError::UnknownType(name.to_owned()))
    }

    pub fn get_type(&self, name: &str) -> Result<Type, SemanticAnalysisError> {
        for structure in &self.structs {
            if structure.name() == name {
                return Ok(Type::Struct(structure.clone()));
            }
        }

        for type_alias in &self.type_aliases {
            if type_alias.0 == name {
                return Ok(type_alias.1.clone());
            }
        }

        return Err(SemanticAnalysisError::UnknownType(name.to_string()));
    }

    pub fn global_scope(&self) -> &Scope {
        &self.global_scope
    }

    pub fn push_function(&mut self, function: Function) -> Result<(), SemanticAnalysisError> {
        if !self.verify_type_name(function.name()) {
            return Err(SemanticAnalysisError::MultipleDefinition(
                function.name().to_owned(),
            ));
        }

        if self.vertex_input_type.is_none() && function.name() == "vertex_main" {
            // Verify parameter count
            if function.parameters().len() != 1 {
                return Err(SemanticAnalysisError::VertexMainParameterCount);
            }

            // Verify input type
            let vertex_input_type = function.parameters()[0].parameter_type();
            match vertex_input_type {
                Type::Struct(_) => {} // TODO: Check for semantics
                _ => {
                    return Err(SemanticAnalysisError::InvalidVertexMainParameterType(
                        vertex_input_type.to_string(),
                    ))
                }
            }
            self.vertex_input_type = Some(vertex_input_type.clone());

            // Verify return type
            match &self.fragment_input_type {
                Some(fragement_input_type) => {
                    if function.return_type() != fragement_input_type {
                        return Err(SemanticAnalysisError::VertexMainReturnTypeMismatch(
                            function.return_type().to_string(),
                            fragement_input_type.to_string(),
                        ));
                    }
                }
                None => match function.return_type() {
                    Type::Struct(_) => {
                        // TODO: Check for semantics
                        self.fragment_input_type = Some(function.return_type().clone())
                    }
                    _ => {
                        return Err(SemanticAnalysisError::InvalidVertexMainReturnType(
                            function.return_type().to_string(),
                        ))
                    }
                },
            }
        } else if function.name() == "fragment_main" {
            // Verify parameter count
            if function.parameters().len() != 1 {
                return Err(SemanticAnalysisError::FragmentMainParameterCount);
            }

            // Verify parameter type
            match &self.fragment_input_type {
                Some(fragement_input_type) => {
                    if function.parameters()[0].parameter_type() != fragement_input_type {
                        return Err(SemanticAnalysisError::FragmentMainParameterTypeMismatch(
                            function.return_type().to_string(),
                            fragement_input_type.to_string(),
                        ));
                    }
                }
                None => match function.return_type() {
                    Type::Struct(_) => {
                        // TODO: Check for semantics
                        self.fragment_input_type = Some(function.return_type().clone())
                    }
                    _ => {
                        return Err(SemanticAnalysisError::InvalidFragmentMainParameterType(
                            function.return_type().to_string(),
                        ))
                    }
                },
            }

            // Verify return type
            if *function.return_type() != Type::float4() {
                return Err(SemanticAnalysisError::InvalidFragmentMainReturnType(
                    function.return_type().to_string(),
                ));
            }
        }

        self.declaration_order.push(DeclarationType::Function);
        self.functions.push_back(function);

        Ok(())
    }

    pub fn push_struct(&mut self, structure: Struct) -> Result<(), SemanticAnalysisError> {
        if !self.verify_type_name(structure.name()) {
            return Err(SemanticAnalysisError::MultipleDefinition(
                structure.name().to_owned(),
            ));
        }

        self.declaration_order.push(DeclarationType::Struct);
        self.structs.push_back(Rc::new(structure));

        Ok(())
    }

    pub fn push_type_alias(
        &mut self,
        name: String,
        alias_type: Type,
    ) -> Result<(), SemanticAnalysisError> {
        if !self.verify_type_name(&name) {
            return Err(SemanticAnalysisError::MultipleDefinition(name));
        }

        self.type_aliases.push((name, alias_type));

        Ok(())
    }

    pub fn push_constant_buffer(
        &mut self,
        constant_buffer: ConstantBuffer,
    ) -> Result<(), SemanticAnalysisError> {
        self.global_scope.define_variable(
            constant_buffer.name().to_owned(),
            constant_buffer.cb_type().clone(),
        )?;

        let slot = constant_buffer.slot();
        self.constant_buffers[slot] = Some(constant_buffer);
        self.declaration_order
            .push(DeclarationType::ConstantBuffer(slot));

        Ok(())
    }

    pub fn push_texture(&mut self, texture: Texture) -> Result<(), SemanticAnalysisError> {
        self.global_scope
            .define_variable(texture.name().to_owned(), Type::texture())?;

        let slot = texture.slot();
        self.textures[slot] = Some(texture);
        self.declaration_order.push(DeclarationType::Texture(slot));

        Ok(())
    }

    pub fn generate_hlsl(mut self) -> String {
        let mut hlsl = format!("// Generated from Alexandria Common Shader Language\n\n");

        for declaration in self.declaration_order {
            match declaration {
                DeclarationType::Function => {
                    hlsl.push_str(&self.functions.pop_front().unwrap().generate_hlsl())
                }
                DeclarationType::Struct => {
                    hlsl.push_str(&self.structs.pop_front().unwrap().generate_hlsl())
                }
                DeclarationType::ConstantBuffer(slot) => {
                    hlsl.push_str(&self.constant_buffers[slot].take().unwrap().generate_hlsl())
                }
                DeclarationType::Texture(slot) => {
                    hlsl.push_str(&self.textures[slot].take().unwrap().generate_hlsl())
                }
            }

            hlsl.push('\n');
        }

        hlsl
    }

    pub fn generate_glsl(mut self) -> (String, String) {
        // Write header
        let mut glsl_vertex =
            format!("#version 430 core\n\n// Generated from Alexandria Common Shader Language\n\n");
        let mut glsl_frag = glsl_vertex.clone();

        // Write fragment output
        glsl_frag.push_str("out vec4 acsl_fragment_color;\n\n");

        // Write vertex input
        let vertex_input_type = self.vertex_input_type.unwrap();
        match vertex_input_type {
            Type::Struct(structure) => {
                let members = structure.members();
                for i in 0..members.len() {
                    glsl_vertex.push_str(&format!(
                        "layout (location = {}) in {} acsl_vertex_input_{};\n",
                        i,
                        members[i].1.glsl(),
                        members[i].0
                    ));
                }

                glsl_vertex.push('\n');
            }
            _ => panic!("Vertex input must be a structure"),
        }

        // Write fragment input & vertex output
        let fragment_input_type = self.fragment_input_type.unwrap();
        let mut position_variable_name = None;
        match fragment_input_type {
            Type::Struct(structure) => {
                let members = structure.members();
                let semantics = structure.semantics();
                for i in 0..members.len() {
                    glsl_vertex.push_str(&format!(
                        "out {} acsl_pixel_input_{};\n",
                        members[i].1.glsl(),
                        members[i].0
                    ));
                    glsl_frag.push_str(&format!(
                        "in {} acsl_pixel_input_{};\n",
                        members[i].1.glsl(),
                        members[i].0
                    ));

                    if semantics[i] == "SV_POSITION" {
                        position_variable_name = Some(members[i].0.to_string());
                    }
                }

                glsl_vertex.push('\n');
                glsl_frag.push('\n');
            }
            _ => panic!("Fragment input must be a structure"),
        };

        let position_variable_name = match position_variable_name {
            Some(position_variable_name) => position_variable_name,
            None => panic!("Pixel input type must have position semantic"),
        };

        // Write declarations
        for declaration in self.declaration_order {
            match declaration {
                DeclarationType::Function => {
                    let (vertex, frag) = self
                        .functions
                        .pop_front()
                        .unwrap()
                        .generate_glsl(&position_variable_name);
                    glsl_vertex.push_str(&vertex);
                    glsl_frag.push_str(&frag);
                }
                DeclarationType::Struct => {
                    let glsl = self.structs.pop_front().unwrap().generate_glsl();
                    glsl_vertex.push_str(&glsl);
                    glsl_frag.push_str(&glsl);
                }
                DeclarationType::ConstantBuffer(slot) => {
                    let glsl = self.constant_buffers[slot].take().unwrap().generate_glsl();
                    glsl_vertex.push_str(&glsl);
                    glsl_frag.push_str(&glsl);
                }
                DeclarationType::Texture(slot) => {
                    let glsl = self.textures[slot].take().unwrap().generate_glsl();
                    glsl_vertex.push_str(&glsl);
                    glsl_frag.push_str(&glsl);
                }
            }

            glsl_vertex.push('\n');
            glsl_frag.push('\n');
        }

        (glsl_vertex, glsl_frag)
    }

    fn verify_type_name(&self, name: &str) -> bool {
        const BUILTIN_TYPENAMES: &[&str] = &[
            "float", "float1", "float2", "float3", "float4", "float4x4", "texture",
        ];

        const RESERVED_TYPENAMES: &[&str] = &[
            "vec1",
            "vec2",
            "vec3",
            "vec4",
            "mat",
            "sampler2D",
            "Texture2D",
        ];

        for function in &self.functions {
            if function.name() == name {
                return false;
            }
        }

        for structure in &self.structs {
            if structure.name() == name {
                return false;
            }
        }

        for type_alias in &self.type_aliases {
            if type_alias.0 == name {
                return false;
            }
        }

        for builtin_name in BUILTIN_TYPENAMES {
            if *builtin_name == name {
                return false;
            }
        }

        for reserved_name in RESERVED_TYPENAMES {
            if *reserved_name == name {
                return false;
            }
        }

        true
    }
}

use crate::{
    annotated::{structure::Struct, AnnotatedSyntaxTree},
    ast::SemanticAnalysisError,
};
use std::{rc::Rc, sync::Once};

#[derive(Clone)]
pub enum Type {
    Primitive(Primitive),
    Struct(Rc<Struct>),
}

#[derive(Clone, PartialEq, Eq)]
pub enum Primitive {
    Void,
    Float,
    FloatVec(usize),
}

static INIT_MEMBERS: Once = Once::new();

static mut VOID_MEMBERS: Option<Rc<Vec<(String, Type)>>> = None;
static mut FLOAT_MEMBERS: Option<Rc<Vec<(String, Type)>>> = None;
static mut FLOAT2_MEMBERS: Option<Rc<Vec<(String, Type)>>> = None;
static mut FLOAT3_MEMBERS: Option<Rc<Vec<(String, Type)>>> = None;
static mut FLOAT4_MEMBERS: Option<Rc<Vec<(String, Type)>>> = None;

impl Type {
    pub fn void() -> Self {
        Type::Primitive(Primitive::Void)
    }

    pub fn float() -> Self {
        Type::Primitive(Primitive::Float)
    }

    pub fn float1() -> Self {
        Type::Primitive(Primitive::FloatVec(1))
    }

    pub fn float2() -> Self {
        Type::Primitive(Primitive::FloatVec(2))
    }

    pub fn float3() -> Self {
        Type::Primitive(Primitive::FloatVec(3))
    }

    pub fn float4() -> Self {
        Type::Primitive(Primitive::FloatVec(4))
    }

    pub fn from_name(
        name: &str,
        output_tree: &AnnotatedSyntaxTree,
    ) -> Result<Self, SemanticAnalysisError> {
        match name {
            "float" => Ok(Type::float()),
            "float1" => Ok(Type::float1()),
            "float2" => Ok(Type::float2()),
            "float3" => Ok(Type::float3()),
            "float4" => Ok(Type::float4()),
            _ => output_tree.get_type(name),
        }
    }

    pub fn member_type(&self, member: &str) -> Result<Type, SemanticAnalysisError> {
        let (members, name) = match self {
            Type::Primitive(primitive) => (primitive.members(), primitive.to_string()),
            Type::Struct(structure) => (structure.members(), structure.name().to_owned()),
        };

        for (name, member_type) in members {
            if name == member {
                return Ok(member_type.clone());
            }
        }

        Err(SemanticAnalysisError::InvalidMember(
            name,
            member.to_owned(),
        ))
    }

    pub fn members(&self) -> &[(String, Type)] {
        match self {
            Type::Primitive(primitive) => primitive.members(),
            Type::Struct(structure) => structure.members(),
        }
    }

    pub fn hlsl(&self) -> String {
        match self {
            Type::Primitive(primitive) => primitive.hlsl(),
            Type::Struct(structure) => structure.name().to_string(),
        }
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Type::Primitive(primitive1) => match other {
                Type::Primitive(primitive2) => primitive1 == primitive2,
                _ => false,
            },
            Type::Struct(struct1) => match other {
                Type::Struct(struct2) => Rc::ptr_eq(struct1, struct2),
                _ => false,
            },
        }
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Primitive(primitive) => write!(f, "{}", primitive),
            Type::Struct(structure) => write!(f, "{}", structure.name()),
        }
    }
}

impl Primitive {
    pub fn members(&self) -> &[(String, Type)] {
        Primitive::init();

        unsafe {
            match self {
                Primitive::Void => &[],
                Primitive::Float => FLOAT_MEMBERS.as_ref().unwrap(),
                Primitive::FloatVec(dimension) => match dimension {
                    1 => FLOAT_MEMBERS.as_ref().unwrap(),
                    2 => FLOAT2_MEMBERS.as_ref().unwrap(),
                    3 => FLOAT3_MEMBERS.as_ref().unwrap(),
                    4 => FLOAT4_MEMBERS.as_ref().unwrap(),
                    _ => panic!("Invalid float vector dimension"),
                },
            }
        }
    }

    pub fn hlsl(&self) -> String {
        match self {
            Primitive::Void => "void".to_owned(),
            Primitive::Float => "float".to_owned(),
            Primitive::FloatVec(dimension) => format!("float{}", dimension),
        }
    }

    fn init() {
        INIT_MEMBERS.call_once(|| unsafe {
            VOID_MEMBERS = Some(Rc::new(Vec::new()));

            FLOAT_MEMBERS = Some(Rc::new(vec![(
                "x".to_owned(),
                Type::Primitive(Primitive::Float),
            )]));

            FLOAT2_MEMBERS = Some(Rc::new(vec![
                ("x".to_owned(), Type::Primitive(Primitive::Float)),
                ("y".to_owned(), Type::Primitive(Primitive::Float)),
            ]));

            FLOAT3_MEMBERS = Some(Rc::new(vec![
                ("x".to_owned(), Type::Primitive(Primitive::Float)),
                ("y".to_owned(), Type::Primitive(Primitive::Float)),
                ("z".to_owned(), Type::Primitive(Primitive::Float)),
            ]));

            FLOAT4_MEMBERS = Some(Rc::new(vec![
                ("x".to_owned(), Type::Primitive(Primitive::Float)),
                ("y".to_owned(), Type::Primitive(Primitive::Float)),
                ("z".to_owned(), Type::Primitive(Primitive::Float)),
                ("w".to_owned(), Type::Primitive(Primitive::Float)),
            ]));
        });
    }
}

impl std::fmt::Display for Primitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Primitive::Void => write!(f, "()"),
            Primitive::Float => write!(f, "float"),
            Primitive::FloatVec(dimension) => write!(f, "float{}", dimension),
        }
    }
}

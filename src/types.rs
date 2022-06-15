use crate::{
    annotated::{structure::Struct, AnnotatedSyntaxTree},
    ast::{expression::MultiplyClass, SemanticAnalysisError},
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
    FloatMatrix(usize, usize),
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

    pub fn float1x1() -> Self {
        Type::Primitive(Primitive::FloatMatrix(1, 1))
    }

    pub fn float1x2() -> Self {
        Type::Primitive(Primitive::FloatMatrix(1, 2))
    }

    pub fn float1x3() -> Self {
        Type::Primitive(Primitive::FloatMatrix(1, 3))
    }

    pub fn float1x4() -> Self {
        Type::Primitive(Primitive::FloatMatrix(1, 4))
    }

    pub fn float2x1() -> Self {
        Type::Primitive(Primitive::FloatMatrix(2, 1))
    }

    pub fn float2x2() -> Self {
        Type::Primitive(Primitive::FloatMatrix(2, 2))
    }

    pub fn float2x3() -> Self {
        Type::Primitive(Primitive::FloatMatrix(2, 3))
    }

    pub fn float2x4() -> Self {
        Type::Primitive(Primitive::FloatMatrix(2, 4))
    }

    pub fn float3x1() -> Self {
        Type::Primitive(Primitive::FloatMatrix(3, 1))
    }

    pub fn float3x2() -> Self {
        Type::Primitive(Primitive::FloatMatrix(3, 2))
    }

    pub fn float3x3() -> Self {
        Type::Primitive(Primitive::FloatMatrix(3, 3))
    }

    pub fn float3x4() -> Self {
        Type::Primitive(Primitive::FloatMatrix(3, 4))
    }

    pub fn float4x1() -> Self {
        Type::Primitive(Primitive::FloatMatrix(4, 1))
    }

    pub fn float4x2() -> Self {
        Type::Primitive(Primitive::FloatMatrix(4, 2))
    }

    pub fn float4x3() -> Self {
        Type::Primitive(Primitive::FloatMatrix(4, 3))
    }

    pub fn float4x4() -> Self {
        Type::Primitive(Primitive::FloatMatrix(4, 4))
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
            "float1x1" => Ok(Type::float1x1()),
            "float1x2" => Ok(Type::float1x2()),
            "float1x3" => Ok(Type::float1x3()),
            "float1x4" => Ok(Type::float1x4()),
            "float2x1" => Ok(Type::float2x1()),
            "float2x2" => Ok(Type::float2x2()),
            "float2x3" => Ok(Type::float2x3()),
            "float2x4" => Ok(Type::float2x4()),
            "float3x1" => Ok(Type::float3x1()),
            "float3x2" => Ok(Type::float3x2()),
            "float3x3" => Ok(Type::float3x3()),
            "float3x4" => Ok(Type::float3x4()),
            "float4x1" => Ok(Type::float4x1()),
            "float4x2" => Ok(Type::float4x2()),
            "float4x3" => Ok(Type::float4x3()),
            "float4x4" => Ok(Type::float4x4()),
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

    pub fn multiply_type(
        &self,
        other: &Type,
        op: MultiplyClass,
    ) -> Result<Type, SemanticAnalysisError> {
        let left_primitive = match self {
            Type::Primitive(primitive) => primitive,
            Type::Struct(_) => {
                return Err(SemanticAnalysisError::InvalidOperation(
                    self.to_string(),
                    op.to_string(),
                    other.to_string(),
                ))
            }
        };

        let right_primitive = match other {
            Type::Primitive(primitive) => primitive,
            Type::Struct(_) => {
                return Err(SemanticAnalysisError::InvalidOperation(
                    self.to_string(),
                    op.to_string(),
                    other.to_string(),
                ))
            }
        };

        left_primitive.multiply_type(right_primitive, op)
    }

    pub fn hlsl(&self) -> String {
        match self {
            Type::Primitive(primitive) => primitive.hlsl(),
            Type::Struct(structure) => structure.name().to_string(),
        }
    }

    pub fn glsl(&self) -> String {
        match self {
            Type::Primitive(primitive) => primitive.glsl(),
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
                Primitive::FloatMatrix(_, _) => &[],
            }
        }
    }

    pub fn multiply_type(
        &self,
        other: &Primitive,
        op: MultiplyClass,
    ) -> Result<Type, SemanticAnalysisError> {
        match self {
            Primitive::Void => Err(SemanticAnalysisError::InvalidOperation(
                self.to_string(),
                op.to_string(),
                other.to_string(),
            )),
            Primitive::Float => match other {
                Primitive::Float => Ok(Type::float()),
                Primitive::FloatVec(dimension) => {
                    Ok(Type::Primitive(Primitive::FloatVec(*dimension)))
                }
                Primitive::FloatMatrix(n, m) => Ok(Type::Primitive(Primitive::FloatMatrix(*n, *m))),
                Primitive::Void => Err(SemanticAnalysisError::InvalidOperation(
                    self.to_string(),
                    op.to_string(),
                    other.to_string(),
                )),
            },
            Primitive::FloatVec(left_dimension) => match other {
                Primitive::Float => Ok(Type::Primitive(Primitive::FloatVec(*left_dimension))),
                Primitive::FloatVec(right_dimension) => match left_dimension == right_dimension {
                    true => Ok(Type::Primitive(Primitive::FloatVec(*left_dimension))),
                    false => Err(SemanticAnalysisError::InvalidOperation(
                        self.to_string(),
                        op.to_string(),
                        other.to_string(),
                    )),
                },
                Primitive::FloatMatrix(n, m) => match left_dimension == n {
                    true => Ok(Type::Primitive(Primitive::FloatVec(*m))),
                    false => Err(SemanticAnalysisError::InvalidOperation(
                        self.to_string(),
                        op.to_string(),
                        other.to_string(),
                    )),
                },
                Primitive::Void => Err(SemanticAnalysisError::InvalidOperation(
                    self.to_string(),
                    op.to_string(),
                    other.to_string(),
                )),
            },
            Primitive::FloatMatrix(left_n, left_m) => match other {
                Primitive::Float => Ok(Type::Primitive(Primitive::FloatMatrix(*left_n, *left_m))),
                Primitive::FloatVec(dimension) => match dimension == left_m {
                    true => Ok(Type::Primitive(Primitive::FloatVec(*left_n))),
                    false => Err(SemanticAnalysisError::InvalidOperation(
                        self.to_string(),
                        op.to_string(),
                        other.to_string(),
                    )),
                },
                Primitive::FloatMatrix(_, right_m) => {
                    Ok(Type::Primitive(Primitive::FloatMatrix(*left_n, *right_m)))
                }
                Primitive::Void => Err(SemanticAnalysisError::InvalidOperation(
                    self.to_string(),
                    op.to_string(),
                    other.to_string(),
                )),
            },
        }
    }

    pub fn hlsl(&self) -> String {
        match self {
            Primitive::Void => "void".to_owned(),
            Primitive::Float => "float".to_owned(),
            Primitive::FloatVec(dimension) => format!("float{}", dimension),
            Primitive::FloatMatrix(n, m) => format!("float{}x{}", n, m),
        }
    }

    pub fn glsl(&self) -> String {
        match self {
            Primitive::Void => "void".to_owned(),
            Primitive::Float => "float".to_owned(),
            Primitive::FloatVec(dimension) => format!("vec{}", dimension),
            Primitive::FloatMatrix(n, m) => format!("mat{}x{}", m, n),
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
            Primitive::FloatMatrix(n, m) => write!(f, "float{}x{}", n, m),
        }
    }
}

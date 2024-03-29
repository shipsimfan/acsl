use crate::{
    annotated::{structure::Struct, AnnotatedSyntaxTree},
    ast::SemanticAnalysisError,
};
use std::{rc::Rc, sync::Once};

#[derive(Clone)]
pub enum Type {
    Primitive(Primitive),
    Struct(Rc<Struct>),
    Alias(Box<Type>),
}

#[derive(Clone, PartialEq, Eq)]
pub enum Primitive {
    Void,
    Float,
    FloatVec(usize),
    FloatMatrix(usize, usize),
    Uint,
    Texture,
}

static INIT_MEMBERS: Once = Once::new();

static mut VOID_MEMBERS: Option<Rc<Vec<(String, Type)>>> = None;
static mut FLOAT_MEMBERS: Option<Rc<Vec<(String, Type)>>> = None;
static mut FLOAT2_MEMBERS: Option<Rc<Vec<(String, Type)>>> = None;
static mut FLOAT3_MEMBERS: Option<Rc<Vec<(String, Type)>>> = None;
static mut FLOAT4_MEMBERS: Option<Rc<Vec<(String, Type)>>> = None;

impl Type {
    pub fn alias(inner_type: Type) -> Self {
        Type::Alias(Box::new(inner_type))
    }

    pub fn void() -> Self {
        Type::Primitive(Primitive::Void)
    }

    pub fn float() -> Self {
        Type::Primitive(Primitive::Float)
    }

    pub fn floatn(n: usize) -> Self {
        assert!(n <= 4 && n >= 1);
        Type::Primitive(Primitive::FloatVec(n))
    }

    pub fn float1() -> Self {
        Type::floatn(1)
    }

    pub fn float2() -> Self {
        Type::floatn(2)
    }

    pub fn float3() -> Self {
        Type::floatn(3)
    }

    pub fn float4() -> Self {
        Type::floatn(4)
    }

    pub fn floatnxm(n: usize, m: usize) -> Self {
        assert!(n <= 4 && n >= 1 && m <= 4 && m >= 1);
        Type::Primitive(Primitive::FloatMatrix(n, m))
    }

    pub fn float1x1() -> Self {
        Type::floatnxm(1, 1)
    }

    pub fn float1x2() -> Self {
        Type::floatnxm(1, 2)
    }

    pub fn float1x3() -> Self {
        Type::floatnxm(1, 3)
    }

    pub fn float1x4() -> Self {
        Type::floatnxm(1, 4)
    }

    pub fn float2x1() -> Self {
        Type::floatnxm(2, 1)
    }

    pub fn float2x2() -> Self {
        Type::floatnxm(2, 2)
    }

    pub fn float2x3() -> Self {
        Type::floatnxm(2, 3)
    }

    pub fn float2x4() -> Self {
        Type::floatnxm(2, 4)
    }

    pub fn float3x1() -> Self {
        Type::floatnxm(3, 1)
    }

    pub fn float3x2() -> Self {
        Type::floatnxm(3, 2)
    }

    pub fn float3x3() -> Self {
        Type::floatnxm(3, 3)
    }

    pub fn float3x4() -> Self {
        Type::floatnxm(3, 4)
    }

    pub fn float4x1() -> Self {
        Type::floatnxm(4, 1)
    }

    pub fn float4x2() -> Self {
        Type::floatnxm(4, 2)
    }

    pub fn float4x3() -> Self {
        Type::floatnxm(4, 3)
    }

    pub fn float4x4() -> Self {
        Type::floatnxm(4, 4)
    }

    pub fn uint() -> Self {
        Type::Primitive(Primitive::Uint)
    }

    pub fn texture() -> Self {
        Type::Primitive(Primitive::Texture)
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
            "uint" => Ok(Type::uint()),
            _ => output_tree.get_type(name),
        }
    }

    pub fn is_uint(&self) -> bool {
        match self {
            Type::Primitive(primitive) => primitive.is_uint(),
            Type::Alias(inner_type) => inner_type.is_uint(),
            _ => false,
        }
    }

    pub fn is_float(&self) -> bool {
        match self {
            Type::Primitive(primitive) => primitive.is_float(),
            Type::Alias(inner_type) => inner_type.is_float(),
            _ => false,
        }
    }

    pub fn is_float_vector(&self) -> bool {
        match self {
            Type::Primitive(primitive) => primitive.is_float_vector(),
            Type::Alias(inner_type) => inner_type.is_float_vector(),
            _ => false,
        }
    }

    pub fn is_float_matrix(&self) -> bool {
        match self {
            Type::Primitive(primitive) => primitive.is_float_matrix(),
            Type::Alias(inner_type) => inner_type.is_float_matrix(),
            _ => false,
        }
    }

    pub fn member_type(&self, member: &str) -> Result<Type, SemanticAnalysisError> {
        let (members, name) = match self {
            Type::Primitive(primitive) => (primitive.members(), primitive.to_string()),
            Type::Struct(structure) => (structure.members(), structure.name().to_owned()),
            Type::Alias(inner_type) => return inner_type.member_type(member),
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
            Type::Alias(inner_type) => inner_type.members(),
        }
    }

    pub fn sum_type(&self, other: &Type) -> Result<Type, SemanticAnalysisError> {
        let left_primitive = match self {
            Type::Primitive(primitive) => primitive,
            Type::Struct(_) => {
                return Err(SemanticAnalysisError::InvalidOperation(
                    self.to_string(),
                    "+",
                    other.to_string(),
                ))
            }
            Type::Alias(inner_type) => return inner_type.sum_type(other),
        };

        let right_primitive = match other {
            Type::Primitive(primitive) => primitive,
            Type::Struct(_) => {
                return Err(SemanticAnalysisError::InvalidOperation(
                    self.to_string(),
                    "+",
                    other.to_string(),
                ))
            }
            Type::Alias(inner_type) => return inner_type.sum_type(other),
        };

        left_primitive.sum_type(right_primitive)
    }

    pub fn product_type(&self, other: &Type) -> Result<Type, SemanticAnalysisError> {
        let left_primitive = match self {
            Type::Primitive(primitive) => primitive,
            Type::Struct(_) => {
                return Err(SemanticAnalysisError::InvalidOperation(
                    self.to_string(),
                    "*",
                    other.to_string(),
                ))
            }
            Type::Alias(inner_type) => return inner_type.product_type(other),
        };

        let right_primitive = match other {
            Type::Primitive(primitive) => primitive,
            Type::Struct(_) => {
                return Err(SemanticAnalysisError::InvalidOperation(
                    self.to_string(),
                    "*",
                    other.to_string(),
                ))
            }
            Type::Alias(inner_type) => return self.product_type(inner_type),
        };

        left_primitive.product_type(right_primitive)
    }

    pub fn hlsl(&self) -> String {
        match self {
            Type::Primitive(primitive) => primitive.hlsl(),
            Type::Struct(structure) => structure.name().to_string(),
            Type::Alias(inner_type) => inner_type.hlsl(),
        }
    }

    pub fn glsl(&self) -> String {
        match self {
            Type::Primitive(primitive) => primitive.glsl(),
            Type::Struct(structure) => structure.name().to_string(),
            Type::Alias(inner_type) => inner_type.glsl(),
        }
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Type::Primitive(primitive1) => match other {
                Type::Primitive(primitive2) => primitive1 == primitive2,
                Type::Alias(inner_type) => self.eq(inner_type),
                _ => false,
            },
            Type::Struct(struct1) => match other {
                Type::Struct(struct2) => Rc::ptr_eq(struct1, struct2),
                Type::Alias(inner_type) => self.eq(inner_type),
                _ => false,
            },
            Type::Alias(inner_type) => inner_type.as_ref().eq(other),
        }
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Primitive(primitive) => write!(f, "{}", primitive),
            Type::Struct(structure) => write!(f, "{}", structure.name()),
            Type::Alias(inner_type) => inner_type.fmt(f),
        }
    }
}

impl Primitive {
    pub fn members(&self) -> &[(String, Type)] {
        Primitive::init();

        unsafe {
            match self {
                Primitive::Float => FLOAT_MEMBERS.as_ref().unwrap(),
                Primitive::FloatVec(dimension) => match dimension {
                    1 => FLOAT_MEMBERS.as_ref().unwrap(),
                    2 => FLOAT2_MEMBERS.as_ref().unwrap(),
                    3 => FLOAT3_MEMBERS.as_ref().unwrap(),
                    4 => FLOAT4_MEMBERS.as_ref().unwrap(),
                    _ => panic!("Invalid float vector dimension"),
                },
                Primitive::FloatMatrix(_, _)
                | Primitive::Void
                | Primitive::Texture
                | Primitive::Uint => &[],
            }
        }
    }

    pub fn is_uint(&self) -> bool {
        match self {
            Primitive::Uint => true,
            _ => false,
        }
    }

    pub fn is_float(&self) -> bool {
        match self {
            Primitive::Float => true,
            _ => false,
        }
    }

    pub fn is_float_vector(&self) -> bool {
        match self {
            Primitive::FloatVec(_) => true,
            _ => false,
        }
    }

    pub fn is_float_matrix(&self) -> bool {
        match self {
            Primitive::FloatMatrix(_, _) => true,
            _ => false,
        }
    }

    pub fn sum_type(&self, other: &Primitive) -> Result<Type, SemanticAnalysisError> {
        match self {
            Primitive::FloatMatrix(_, _) | Primitive::Void | Primitive::Texture => Err(()),
            Primitive::Float => match other {
                Primitive::Float => Ok(Type::float()),
                Primitive::FloatMatrix(_, _)
                | Primitive::FloatVec(_)
                | Primitive::Void
                | Primitive::Uint
                | Primitive::Texture => Err(()),
            },
            Primitive::FloatVec(left_dimension) => match other {
                Primitive::FloatVec(right_dimension) => match left_dimension == right_dimension {
                    true => Ok(Type::Primitive(Primitive::FloatVec(*left_dimension))),
                    false => Err(()),
                },
                Primitive::FloatMatrix(_, _)
                | Primitive::Float
                | Primitive::Void
                | Primitive::Texture
                | Primitive::Uint => Err(()),
            },
            Primitive::Uint => match other {
                Primitive::Uint => Ok(Type::uint()),
                _ => Err(()),
            },
        }
        .map_err(|_| {
            SemanticAnalysisError::InvalidOperation(self.to_string(), "+", other.to_string())
        })
    }

    pub fn product_type(&self, other: &Primitive) -> Result<Type, SemanticAnalysisError> {
        match self {
            Primitive::Void | Primitive::Texture => Err(()),
            Primitive::Float => match other {
                Primitive::Float => Ok(Type::float()),
                Primitive::FloatVec(dimension) => {
                    Ok(Type::Primitive(Primitive::FloatVec(*dimension)))
                }
                Primitive::FloatMatrix(n, m) => Ok(Type::Primitive(Primitive::FloatMatrix(*n, *m))),
                Primitive::Void | Primitive::Texture | Primitive::Uint => Err(()),
            },
            Primitive::FloatVec(left_dimension) => match other {
                Primitive::Float => Ok(Type::Primitive(Primitive::FloatVec(*left_dimension))),
                Primitive::FloatVec(right_dimension) => match left_dimension == right_dimension {
                    true => Ok(Type::Primitive(Primitive::FloatVec(*left_dimension))),
                    false => Err(()),
                },
                Primitive::FloatMatrix(n, m) => match left_dimension == n {
                    true => Ok(Type::Primitive(Primitive::FloatVec(*m))),
                    false => Err(()),
                },
                Primitive::Void | Primitive::Texture | Primitive::Uint => Err(()),
            },
            Primitive::FloatMatrix(left_n, left_m) => match other {
                Primitive::Float => Ok(Type::Primitive(Primitive::FloatMatrix(*left_n, *left_m))),
                Primitive::FloatVec(dimension) => match dimension == left_m {
                    true => Ok(Type::Primitive(Primitive::FloatVec(*left_n))),
                    false => Err(()),
                },
                Primitive::FloatMatrix(_, right_m) => {
                    Ok(Type::Primitive(Primitive::FloatMatrix(*left_n, *right_m)))
                }
                Primitive::Void | Primitive::Texture | Primitive::Uint => Err(()),
            },
            Primitive::Uint => match other {
                Primitive::Uint => Ok(Type::uint()),
                _ => Err(()),
            },
        }
        .map_err(|_| {
            SemanticAnalysisError::InvalidOperation(self.to_string(), "*", other.to_string())
        })
    }

    pub fn hlsl(&self) -> String {
        match self {
            Primitive::Void => "void".to_owned(),
            Primitive::Float => "float".to_owned(),
            Primitive::FloatVec(dimension) => format!("float{}", dimension),
            Primitive::FloatMatrix(n, m) => format!("float{}x{}", n, m),
            Primitive::Uint => "uint".to_owned(),
            Primitive::Texture => "Texture2D".to_owned(),
        }
    }

    pub fn glsl(&self) -> String {
        match self {
            Primitive::Void => "void".to_owned(),
            Primitive::Float => "float".to_owned(),
            Primitive::FloatVec(dimension) => format!("vec{}", dimension),
            Primitive::FloatMatrix(n, m) => format!("mat{}x{}", m, n),
            Primitive::Uint => "uint".to_owned(),
            Primitive::Texture => "sampler2D".to_owned(),
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
            Primitive::Uint => write!(f, "uint"),
            Primitive::Texture => write!(f, "texture"),
        }
    }
}

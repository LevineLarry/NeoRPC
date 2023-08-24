//Handles converting from the neorpc type names to the language-specific type names

pub struct TypeResolver;

impl TypeResolver {
    pub fn resolve(type_: &String) -> String {
        match type_.as_str() {
            "string" => "String".to_string(),
            "int" => "i32".to_string(),
            "float" => "f32".to_string(),
            "boolean" => "bool".to_string(),
            _ => panic!("Invalid type")
        }
    }

    pub fn reverse_resolve(type_: &String) -> String {
        match type_.as_str() {
            "String" => "string".to_string(),
            "i32" => "int".to_string(),
            "f32" => "float".to_string(),
            "bool" => "boolean".to_string(),
            _ => panic!("Invalid type")
        }
    }
}
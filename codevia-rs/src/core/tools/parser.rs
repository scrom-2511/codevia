use std::fs::read_to_string;

use tree_sitter::{Language, Node, Parser};

use crate::core::tools::WorkspaceErrors;

pub enum CodeLanguage {
    RUST,
    PYTHON,
    JAVA,
}

impl CodeLanguage {
    pub fn lang(&self) -> Result<Language, WorkspaceErrors> {
        let language = match self {
            CodeLanguage::RUST => tree_sitter_rust::LANGUAGE,
            CodeLanguage::PYTHON => tree_sitter_python::LANGUAGE,
            CodeLanguage::JAVA => tree_sitter_java::LANGUAGE,
        }
        .into();

        Ok(language)
    }
}

pub struct CodeParser {
    language: CodeLanguage,
    parser: Parser,
}

impl CodeParser {
    pub fn new(language: CodeLanguage) -> Result<Self, WorkspaceErrors> {
        let mut parser = Parser::new();
        parser
            .set_language(&CodeLanguage::lang(&language).unwrap())
            .map_err(|e| WorkspaceErrors::new(e.to_string(), crate::core::ErrorKind::TreeSitter))?;

        Ok(Self { language, parser })
    }

    fn traverse(&self, node: Node, code: &str, depth: usize) -> String {
        let mut cursor = node.walk();

        let mut final_string = String::new();

        for child in node.children(&mut cursor) {
            let description = match &self.language {
                CodeLanguage::RUST => RustLanguage::describe_node(child, code),
                CodeLanguage::PYTHON => PythonLanguage::describe_node(child, code),
                CodeLanguage::JAVA => JavaLanguage::describe_node(child, code),
            };

            if let Some(description) = description {
                final_string.push_str(&"  ".repeat(depth));
                final_string.push_str(&description);
                final_string.push('\n');
            }

            final_string.push_str(&self.traverse(child, code, depth + 1));
        }

        final_string
    }

    pub fn get_outline(&mut self, code: &str) -> Result<String, WorkspaceErrors> {
        let tree = self.parser.parse(code, None).ok_or_else(|| {
            WorkspaceErrors::new(
                "Failed to parse source code".to_owned(),
                crate::core::ErrorKind::TreeSitter,
            )
        })?;

        let result = self.traverse(tree.root_node(), code, 0);

        Ok(result)
    }
}

trait Describe {
    fn describe_node(node: Node, code: &str) -> Option<String>;
}

pub struct RustLanguage;

impl Describe for RustLanguage {
    fn describe_node(node: Node, code: &str) -> Option<String> {
        const FULL_TEXT_NODES: &[&str] = &[
            "type_item",
            "const_item",
            "static_item",
            "extern_crate_declaration",
        ];

        const HEADER_NODES: &[&str] = &[
            "struct_item",
            "union_item",
            "enum_item",
            "trait_item",
            "impl_item",
            "function_item",
            "mod_item",
            "foreign_mod_item",
            "function_signature_item",
        ];

        if node.kind() == "macro_definition" {
            let name = node
                .child_by_field_name("name")
                .unwrap()
                .utf8_text(code.as_bytes())
                .unwrap();

            Some(format!("macro_rules! {name}"))
        } else if HEADER_NODES.contains(&node.kind()) {
            let start = node.start_byte();

            let end = if let Some(body) = node.child_by_field_name("body") {
                body.start_byte()
            } else {
                node.end_byte()
            };

            Some(code[start..end].trim().to_owned())
        } else if FULL_TEXT_NODES.contains(&node.kind()) {
            Some(node.utf8_text(&code.as_bytes()).ok()?.to_owned())
        } else {
            None
        }
    }
}

pub struct PythonLanguage;

impl Describe for PythonLanguage {
    fn describe_node(node: Node, code: &str) -> Option<String> {
        const HEADER_NODES: &[&str] = &["class_definition", "function_definition"];

        const FULL_TEXT_NODES: &[&str] = &["type_alias_statement"];

        if node.kind() == "decorated_definition" {
            let start = node.start_byte();

            let end = if let Some(def) = node.child_by_field_name("definition") {
                def.start_byte()
            } else {
                node.end_byte()
            };

            Some(code[start..end].trim().to_owned())
        } else if HEADER_NODES.contains(&node.kind()) {
            let start = node.start_byte();

            let end = if let Some(body) = node.child_by_field_name("body") {
                body.start_byte()
            } else {
                node.end_byte()
            };

            Some(code[start..end].trim().to_owned())
        } else if FULL_TEXT_NODES.contains(&node.kind()) {
            Some(node.utf8_text(code.as_bytes()).ok()?.trim().to_owned())
        } else if node.kind() == "expression_statement" {
            let is_module_level = node
                .parent()
                .map(|parent| parent.kind() == "module")
                .unwrap_or(false);

            let has_assignment = node
                .children(&mut node.walk())
                .any(|c| c.kind() == "assignment");

            if is_module_level && has_assignment {
                Some(node.utf8_text(code.as_bytes()).ok()?.trim().to_owned())
            } else {
                None
            }
        } else {
            None
        }
    }
}

pub struct JavaLanguage;

impl Describe for JavaLanguage {
    fn describe_node(node: Node, code: &str) -> Option<String> {
        const HEADER_NODES: &[&str] = &[
            "class_declaration",
            "interface_declaration",
            "enum_declaration",
            "record_declaration",
            "constructor_declaration",
            "compact_constructor_declaration",
            "method_declaration",
            "annotation_type_declaration",
        ];

        const FULL_TEXT_NODES: &[&str] = &["package_declaration"];

        if HEADER_NODES.contains(&node.kind()) {
            let start = node.start_byte();

            let end = if let Some(body) = node.child_by_field_name("body") {
                body.start_byte()
            } else {
                node.end_byte()
            };

            Some(code[start..end].trim().to_owned())
        } else if FULL_TEXT_NODES.contains(&node.kind()) {
            Some(node.utf8_text(code.as_bytes()).ok()?.trim().to_owned())
        } else {
            None
        }
    }
}

#[test]
fn test_rs() {
    let mut code_parser = CodeParser::new(CodeLanguage::RUST).unwrap();

    let code =
        read_to_string("/home/scrom/code/codevia/codevia-rs/src/core/tools/test.rs").unwrap();

    let resutl = code_parser.get_outline(&code).unwrap();

    println!("{resutl}");
}

#[test]
fn test_py() {
    let mut code_parser = CodeParser::new(CodeLanguage::PYTHON).unwrap();

    let code =
        read_to_string("/home/scrom/code/codevia/codevia-rs/src/core/tools/test.py").unwrap();

    let resutl = code_parser.get_outline(&code).unwrap();

    println!("{resutl}");
}

#[test]
fn test_java() {
    let mut code_parser = CodeParser::new(CodeLanguage::JAVA).unwrap();

    let code =
        read_to_string("/home/scrom/code/codevia/codevia-rs/src/core/tools/test.java").unwrap();

    let resutl = code_parser.get_outline(&code).unwrap();

    println!("{resutl}");
}

use std::fs::read_to_string;

use crate::core::tools::WorkspaceErrors;
use tree_sitter::{Language, Node, Parser};

#[derive(Clone, Copy)]
enum Name {
    Field(&'static str),
    RustImpl,
}

#[derive(Clone, Copy)]
enum FnSig {
    Fields {
        params: &'static [&'static str],
        ret: &'static [&'static str],
    },
}

#[derive(Clone, Copy)]
enum Header {
    Body(&'static str),
    WholeNode,
    Name,
}

struct Rule {
    kind: &'static str,
    label: &'static str,
    label_field: Option<&'static str>,
    name: Name,
    body_field: Option<&'static str>,
    fn_sig: Option<FnSig>,
    header: Header,
}

impl Rule {
    const fn new(kind: &'static str, label: &'static str) -> Self {
        Self {
            kind,
            label,
            label_field: None,
            name: Name::Field("name"),
            header: Header::Name,
            body_field: None,
            fn_sig: None,
        }
    }
}

enum CodeLanguage {
    RUST,
}

impl CodeLanguage {
    pub fn lang(&self) -> Result<Language, WorkspaceErrors> {
        let language = match self {
            CodeLanguage::RUST => tree_sitter_rust::LANGUAGE,
        }
        .into();

        Ok(language)
    }
}

const RUST_RULES: &[Rule] = &[
    // Functions
    Rule {
        header: Header::Body("body"),
        body_field: Some("body"),
        fn_sig: Some(FnSig::Fields {
            params: &["parameters"],
            ret: &["return_type"],
        }),
        ..Rule::new("function_item", "fn")
    },
    Rule {
        body_field: None,
        ..Rule::new("function_signature_item", "fn")
    },
    // Types
    Rule::new("struct_item", "struct"),
    Rule::new("enum_item", "enum"),
    Rule::new("union_item", "union"),
    Rule {
        header: Header::Body("body"),
        body_field: Some("body"),
        ..Rule::new("trait_item", "trait")
    },
    Rule {
        header: Header::Body("body"),
        body_field: Some("body"),
        ..Rule::new("mod_item", "mod")
    },
    Rule {
        header: Header::Body("body"),
        body_field: Some("body"),
        ..Rule::new("foreign_mod_item", "extern")
    },
    Rule {
        header: Header::Body("body"),
        name: Name::RustImpl,
        body_field: Some("body"),
        ..Rule::new("impl_item", "impl")
    },
    Rule::new("type_item", "type"),
    // Modules and imports
    Rule {
        body_field: Some("body"),
        ..Rule::new("mod_item", "mod")
    },
    Rule::new("extern_crate_declaration", "extern crate"),
    // Values
    Rule::new("const_item", "const"),
    Rule::new("static_item", "static"),
    // Members
    Rule::new("enum_variant", "variant"),
    Rule::new("field_declaration", "field"),
    // Macros
    Rule::new("macro_definition", "macro"),
    Rule {
        name: Name::Field("argument"),
        ..Rule::new("use_declaration", "use")
    },
    Rule {
        body_field: Some("body"),
        ..Rule::new("foreign_mod_item", "extern")
    },
];

fn describe(node: Node, code: &str, rules: &[Rule]) -> Option<String> {
    let rule = rules.iter().find(|rule| rule.kind == node.kind())?;

    if node.kind() == "struct_item" {
        let end = (0..node.child_count())
            .filter_map(|i| node.child(i))
            .find(|child| {
                matches!(
                    child.kind(),
                    "field_declaration_list" | "ordered_field_declaration_list" | ";"
                )
            })
            .map(|child| child.start_byte())
            .unwrap_or(node.end_byte());

        let header = code[node.start_byte()..end]
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
            .trim_end_matches(',')
            .to_owned();

        return Some(header);
    }

    match rule.header {
        Header::Body(body_field) => {
            let body = node.child_by_field_name(body_field)?;

            let header = code[node.start_byte()..body.start_byte()]
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ")
                .trim_end_matches(',')
                .to_owned();

            Some(header)
        }

        Header::WholeNode => Some(
            node.utf8_text(code.as_bytes())
                .ok()?
                .trim()
                .trim_end_matches(';')
                .to_owned(),
        ),

        Header::Name => {
            if node.kind() == "function_signature_item" {
                return Some(
                    node.utf8_text(code.as_bytes())
                        .ok()?
                        .trim()
                        .trim_end_matches(';')
                        .to_owned(),
                );
            }

            let name = match rule.name {
                Name::Field(field) => node
                    .child_by_field_name(field)?
                    .utf8_text(code.as_bytes())
                    .ok()?,

                Name::RustImpl => {
                    let trait_node = node.child_by_field_name("trait");
                    let type_node = node.child_by_field_name("type")?;

                    let type_name = type_node.utf8_text(code.as_bytes()).ok()?;

                    if let Some(trait_node) = trait_node {
                        let trait_name = trait_node.utf8_text(code.as_bytes()).ok()?;

                        return Some(format!("impl {} for {}", trait_name, type_name));
                    }

                    return Some(format!("impl {}", type_name));
                }
            };

            Some(format!("{} {}", rule.label, name))
        }
    }
}

fn traverse(node: Node, code: &str, rules: &[Rule], depth: usize) {
    let description = describe(node, code, rules);

    let next_depth = if let Some(description) = description {
        println!("{}{}", "  ".repeat(depth), description);
        depth + 1
    } else {
        depth
    };

    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        traverse(child, code, rules, next_depth);
    }
}

#[test]
fn test_rs() {
    let language = CodeLanguage::RUST;

    let mut parser = Parser::new();
    parser.set_language(&language.lang().unwrap()).unwrap();

    let code =
        read_to_string("/home/scrom/code/codevia/codevia-rs/src/core/tools/test.rs").unwrap();

    let tree = parser.parse(&code, None).unwrap();

    let root = tree.root_node();
    traverse(root, &code, RUST_RULES, 0);
}

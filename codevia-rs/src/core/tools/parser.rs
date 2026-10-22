use std::fs::read_to_string;

use tree_sitter::{Language, Node, Parser};

use crate::core::tools::WorkspaceErrors;

pub enum CodeLanguage {
    RUST,
    PYTHON,
    JAVA,
    C,
}

impl CodeLanguage {
    pub fn lang(&self) -> Result<Language, WorkspaceErrors> {
        let language = match self {
            CodeLanguage::RUST => tree_sitter_rust::LANGUAGE,
            CodeLanguage::PYTHON => tree_sitter_python::LANGUAGE,
            CodeLanguage::JAVA => tree_sitter_java::LANGUAGE,
            CodeLanguage::C => tree_sitter_c::LANGUAGE,
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
                CodeLanguage::C => CLanguage::describe_node(child, code),
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

pub struct CLanguage;

fn strip_trailing_semicolon(s: &str) -> &str {
    let trimmed = s.trim();
    if let Some(stripped) = trimmed.strip_suffix(';') {
        stripped.trim_end()
    } else {
        trimmed
    }
}

fn get_function_declarator(node: Node) -> Option<Node> {
    if node.kind() == "function_declarator" {
        return Some(node);
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if let Some(fd) = get_function_declarator(child) {
            return Some(fd);
        }
    }
    None
}

fn get_function_name<'a>(node: Node<'a>, code: &'a str) -> Option<&'a str> {
    let fd = get_function_declarator(node)?;
    let decl = fd.child_by_field_name("declarator")?;
    let mut curr = decl;
    while curr.kind() == "parenthesized_declarator" || curr.kind() == "pointer_declarator" {
        if let Some(inner) = curr.child_by_field_name("declarator") {
            curr = inner;
        } else {
            break;
        }
    }
    if curr.kind() == "identifier" {
        curr.utf8_text(code.as_bytes()).ok()
    } else {
        None
    }
}

fn has_earlier_function_sibling(node: Node, fn_name: &str, code: &str) -> bool {
    let parent = match node.parent() {
        Some(p) if p.kind() == "translation_unit" => p,
        _ => return false,
    };

    let mut cursor = parent.walk();
    for sibling in parent.children(&mut cursor) {
        if sibling.id() == node.id() {
            break;
        }
        if sibling.kind() == "function_definition" || sibling.kind() == "declaration" {
            if let Some(sibling_fn) = get_function_name(sibling, code) {
                if sibling_fn == fn_name {
                    return true;
                }
            }
        }
    }
    false
}

fn has_typedef_for_type(node: Node, name: &str, code: &str) -> bool {
    let root = match node.parent() {
        Some(p) if p.kind() == "translation_unit" => p,
        Some(p) => match p.parent() {
            Some(gp) if gp.kind() == "translation_unit" => gp,
            _ => return false,
        },
        _ => return false,
    };

    let mut cursor = root.walk();
    for child in root.children(&mut cursor) {
        if child.kind() == "type_definition" {
            if let Some(decl) = child.child_by_field_name("declarator") {
                if let Ok(decl_name) = decl.utf8_text(code.as_bytes()) {
                    if decl_name.trim() == name {
                        return true;
                    }
                }
            }
            if let Some(ty) = child.child_by_field_name("type") {
                if let Some(ty_name) = ty.child_by_field_name("name") {
                    if let Ok(n) = ty_name.utf8_text(code.as_bytes()) {
                        if n.trim() == name {
                            return true;
                        }
                    }
                }
            }
        }
    }
    false
}

impl Describe for CLanguage {
    fn describe_node(node: Node, code: &str) -> Option<String> {
        if node.kind() == "preproc_def" {
            let name = node.child_by_field_name("name")?.utf8_text(code.as_bytes()).ok()?.trim();
            return Some(format!("#define {name}"));
        }

        if node.kind() == "preproc_function_def" {
            let name = node.child_by_field_name("name")?.utf8_text(code.as_bytes()).ok()?.trim();
            let params = node.child_by_field_name("parameters")?.utf8_text(code.as_bytes()).ok()?.trim();
            return Some(format!("#define {name}{params}"));
        }

        if node.kind() == "type_definition" {
            if let Some(type_node) = node.child_by_field_name("type") {
                if matches!(
                    type_node.kind(),
                    "struct_specifier" | "union_specifier" | "enum_specifier"
                ) {
                    let keyword = match type_node.kind() {
                        "struct_specifier" => "struct",
                        "union_specifier" => "union",
                        "enum_specifier" => "enum",
                        _ => unreachable!(),
                    };

                    if let Some(declarator) = node.child_by_field_name("declarator") {
                        if let Ok(name) = declarator.utf8_text(code.as_bytes()) {
                            let clean_name = name.trim();
                            return Some(format!("typedef {keyword} {clean_name}"));
                        }
                    }
                }
            }

            let raw = node.utf8_text(code.as_bytes()).ok()?.trim();
            return Some(strip_trailing_semicolon(raw).to_owned());
        }

        if matches!(
            node.kind(),
            "struct_specifier" | "union_specifier" | "enum_specifier"
        ) {
            let is_top_level = match node.parent() {
                Some(p) => {
                    p.kind() == "translation_unit"
                        || (p.kind() == "declaration"
                            && p.parent()
                                .map(|gp| gp.kind() == "translation_unit")
                                .unwrap_or(false))
                }
                None => false,
            };

            if !is_top_level || node.child_by_field_name("body").is_none() {
                return None;
            }

            let name = node.child_by_field_name("name")?.utf8_text(code.as_bytes()).ok()?.trim();
            if has_typedef_for_type(node, name, code) {
                return None;
            }

            let keyword = match node.kind() {
                "struct_specifier" => "struct",
                "union_specifier" => "union",
                "enum_specifier" => "enum",
                _ => unreachable!(),
            };

            return Some(format!("{keyword} {name}"));
        }

        if node.kind() == "function_definition" {
            let fn_name = get_function_name(node, code)?;
            if has_earlier_function_sibling(node, fn_name, code) {
                return None;
            }

            let start = node.start_byte();
            let end = if let Some(body) = node.child_by_field_name("body") {
                body.start_byte()
            } else {
                node.end_byte()
            };

            let sig = code[start..end].trim();
            return Some(strip_trailing_semicolon(sig).to_owned());
        }

        if node.kind() == "declaration" {
            let is_top_level = node
                .parent()
                .map(|p| p.kind() == "translation_unit")
                .unwrap_or(false);

            if !is_top_level {
                return None;
            }

            if let Some(fn_name) = get_function_name(node, code) {
                if has_earlier_function_sibling(node, fn_name, code) {
                    return None;
                }

                let raw = node.utf8_text(code.as_bytes()).ok()?.trim();
                return Some(strip_trailing_semicolon(raw).to_owned());
            }

            if let Some(ty) = node.child_by_field_name("type") {
                if matches!(
                    ty.kind(),
                    "struct_specifier" | "union_specifier" | "enum_specifier"
                ) && node.child_by_field_name("declarator").is_none() {
                    return None;
                }
            }

            if let Some(decl) = node.child_by_field_name("declarator") {
                let inner = if decl.kind() == "init_declarator" {
                    decl.child_by_field_name("declarator").unwrap_or(decl)
                } else {
                    decl
                };

                let start = node.start_byte();
                let end = inner.end_byte();
                let var_decl = code[start..end].trim();
                return Some(strip_trailing_semicolon(var_decl).to_owned());
            }

            let raw = node.utf8_text(code.as_bytes()).ok()?.trim();
            return Some(strip_trailing_semicolon(raw).to_owned());
        }

        None
    }
}

#[test]
fn test_rs() {
    let mut code_parser = CodeParser::new(CodeLanguage::RUST).unwrap();

    let code =
        read_to_string("/home/scrom/code/codevia/codevia-rs/src/core/tools/test_files/test.rs")
            .unwrap();

    let resutl = code_parser.get_outline(&code).unwrap();

    println!("{resutl}");
}

#[test]
fn test_py() {
    let mut code_parser = CodeParser::new(CodeLanguage::PYTHON).unwrap();

    let code =
        read_to_string("/home/scrom/code/codevia/codevia-rs/src/core/tools/test_files/test.py")
            .unwrap();

    let resutl = code_parser.get_outline(&code).unwrap();

    println!("{resutl}");
}

#[test]
fn test_java() {
    let mut code_parser = CodeParser::new(CodeLanguage::JAVA).unwrap();

    let code =
        read_to_string("/home/scrom/code/codevia/codevia-rs/src/core/tools/test_files/test.java")
            .unwrap();

    let resutl = code_parser.get_outline(&code).unwrap();

    println!("{resutl}");
}

#[test]
fn test_c() {
    let mut code_parser = CodeParser::new(CodeLanguage::C).unwrap();

    let code =
        read_to_string("/home/scrom/code/codevia/codevia-rs/src/core/tools/test_files/test.c")
            .unwrap();

    let result = code_parser.get_outline(&code).unwrap();

    println!("{result}");

    // 1. Macros
    assert!(result.contains("#define MAX_ITEMS"));
    assert!(result.contains("#define BUFFER_SIZE"));
    assert!(result.contains("#define SQUARE(x)"));
    assert!(result.contains("#define LOG_INFO(fmt, ...)"));
    assert!(result.contains("#define DECLARE_GETTER(type, name)"));
    assert!(!result.contains("#define MAX_ITEMS 100"));
    assert!(!result.contains("fprintf"));

    // 2. Global variables without initializers
    assert!(result.contains("static const char *APPLICATION_NAME"));
    assert!(result.contains("static const size_t DEFAULT_CAPACITY"));
    assert!(result.contains("static int global_counter"));
    assert!(result.contains("static volatile bool shutdown_requested"));
    assert!(!result.contains("Codevia C Parser"));
    assert!(!result.contains("global_counter = 0"));

    // 3. Normalized typedef structs, unions, and enums
    assert!(result.contains("typedef struct Point"));
    assert!(result.contains("typedef struct Rectangle"));
    assert!(result.contains("typedef union Value"));
    assert!(result.contains("typedef enum Status"));
    assert!(result.contains("typedef struct User"));
    assert!(result.contains("typedef struct Repository"));
    assert!(result.contains("typedef struct Node"));
    assert!(result.contains("typedef struct LinkedList"));
    assert!(result.contains("typedef struct RepositoryOps"));
    assert!(result.contains("typedef struct MemoryRepository"));
    assert!(result.contains("typedef struct Error"));
    assert!(result.contains("typedef struct UserResult"));

    // Internal fields and duplicate standalone structs must NOT appear
    assert!(!result.lines().any(|l| l.trim() == "struct User"));
    assert!(!result.lines().any(|l| l.trim() == "struct Repository"));
    assert!(!result.lines().any(|l| l.trim() == "union"));
    assert!(!result.contains("STATUS_PENDING"));
    assert!(!result.contains("int x;"));

    // 4. Typedef aliases and function pointers
    assert!(result.contains("typedef unsigned long UserId"));
    assert!(result.contains("typedef const char *StringView"));
    assert!(result.contains("typedef int (*Comparator)(const void *, const void *)"));
    assert!(result.contains("typedef void (*Callback)(int status, void *context)"));
    assert!(result.contains("typedef int (*TransformFn)(const User *input, User *output, void *context)"));

    // 5. Function deduplication (only 1 signature for prototypes + definitions)
    assert_eq!(
        result.matches("int user_init(User *user, UserId id, const char *name)").count(),
        1
    );
    assert_eq!(
        result.matches("static void log_message(const char *level, const char *message)").count(),
        1
    );
    assert!(result.contains("int main(int argc, char **argv)"));
    assert!(!result.contains("return EXIT_SUCCESS;"));
    assert!(!result.contains("return 0;"));

    // 6. No trailing semicolons
    for line in result.lines() {
        assert!(!line.trim().ends_with(';'), "Line ends with semicolon: {}", line);
    }
}

#[test]
fn test_c_edge_cases() {
    let mut code_parser = CodeParser::new(CodeLanguage::C).unwrap();

    let code = r#"
#define MULTILINE_MACRO(a, b) \
    do { \
        int temp = (a); \
        (a) = (b); \
        (b) = temp; \
    } while (0)

#define SIMPLE_FLAG

// Forward declaration of struct with no subsequent definition
struct Unimplemented;

// Standalone struct without typedef
struct StandaloneConfig {
    int timeout_ms;
    bool debug_mode;
};

// Standalone enum without typedef
enum LogLevel {
    LOG_TRACE,
    LOG_DEBUG,
    LOG_INFO
};

// Standalone union without typedef
union RawData {
    int i;
    float f;
};

// Typedef with forward declaration
typedef struct ForwardType ForwardType;
struct ForwardType {
    int id;
};

// Extern variable
extern int global_errno;

// Function only declared (prototype only)
int pure_declaration(float x);

// Function only defined (no prototype)
void only_definition(int code) {
    if (code > 0) return;
}

// Function with duplicate prototypes and definition
char *dup_func(void);
char *dup_func(void);
char *dup_func(void) {
    return "test";
}
"#;

    let result = code_parser.get_outline(code).unwrap();
    println!("{result}");

    // Macros
    assert!(result.contains("#define MULTILINE_MACRO(a, b)"));
    assert!(result.contains("#define SIMPLE_FLAG"));
    assert!(!result.contains("int temp"));

    // Forward declaration with no body ignored
    assert!(!result.contains("Unimplemented"));

    // Standalone types without typedef
    assert!(result.contains("struct StandaloneConfig"));
    assert!(result.contains("enum LogLevel"));
    assert!(result.contains("union RawData"));
    assert!(!result.contains("timeout_ms"));
    assert!(!result.contains("LOG_TRACE"));

    // Forward declaration with typedef
    assert!(result.contains("typedef struct ForwardType"));
    assert_eq!(result.matches("ForwardType").count(), 1);

    // Extern variable
    assert!(result.contains("extern int global_errno"));

    // Functions
    assert!(result.contains("int pure_declaration(float x)"));
    assert!(result.contains("void only_definition(int code)"));
    assert_eq!(result.matches("char *dup_func(void)").count(), 1);

    // No trailing semicolons
    for line in result.lines() {
        assert!(!line.trim().ends_with(';'), "Line ends with semicolon: {}", line);
    }
}

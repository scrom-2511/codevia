from codevia.tools.file_system import FileSystem
from tree_sitter import Parser
from tree_sitter import Language
import importlib
class CodeIntelligence:
    def __init__(self, fs: FileSystem):
        self.fs = fs
        
    def _read_and_parse(self, file_path: str, lang_module_name: str, get_language_fn_name: str = "language"):
        path_obj = self.fs.resolve_safe_path(file_path)
        
        try:
            lang_module = importlib.import_module(lang_module_name)
        except ImportError:
            raise ImportError(f"Please install {lang_module_name} to outline this file.")

        with open(path_obj, "r", encoding="utf-8") as file:
            code = file.read()
            
        get_lang_fn = getattr(lang_module, get_language_fn_name)
        LANG = Language(get_lang_fn())
        parser = Parser(LANG)
        tree = parser.parse(bytes(code, "utf8"))
        return code, tree

    def get_file_outline_py(self, file_path: str) -> str | list[str]:
        try:
            code, tree = self._read_and_parse(file_path, "tree_sitter_python")
            outline = []
            
            def traverse(node, depth=0):
                if node.type == "class_definition":
                    name_node = node.child_by_field_name("name")
                    name = code[name_node.start_byte:name_node.end_byte] if name_node else "Anonymous"
                    line_no = node.start_point[0] + 1
                    outline.append("  " * depth + f"[{line_no}] class {name}")
                    depth += 1
                elif node.type == "function_definition":
                    name_node = node.child_by_field_name("name")
                    name = code[name_node.start_byte:name_node.end_byte] if name_node else "anonymous"

                    parameters_node = node.child_by_field_name("parameters")
                    parameters = code[parameters_node.start_byte:parameters_node.end_byte] if parameters_node else "()"

                    return_type_node = node.child_by_field_name("return_type")
                    return_type = code[return_type_node.start_byte:return_type_node.end_byte] if return_type_node else ""

                    line_no = node.start_point[0] + 1
                    outline.append("  " * depth + f"[{line_no}] def {name}{parameters} -> {return_type}")
                    depth += 1
                
                for child in node.children:
                    traverse(child, depth)
                    
            traverse(tree.root_node)
            return "\n".join(outline) if outline else "No classes or functions found."
        except Exception as e:
            return str(e)

    def get_file_outline_js(self, file_path: str) -> str | list[str]:
        try:
            code, tree = self._read_and_parse(file_path, "tree_sitter_javascript")
            outline = []
            
            def traverse(node, depth=0):
                if node.type == "class_declaration":
                    name_node = node.child_by_field_name("name")
                    name = code[name_node.start_byte:name_node.end_byte] if name_node else "Anonymous"
                    line_no = node.start_point[0] + 1
                    outline.append("  " * depth + f"[{line_no}] class {name}")
                    depth += 1
                elif node.type in ["function_declaration", "method_definition"]:
                    name_node = node.child_by_field_name("name")
                    name = code[name_node.start_byte:name_node.end_byte] if name_node else "anonymous"

                    parameters_node = node.child_by_field_name("parameters")
                    parameters = code[parameters_node.start_byte:parameters_node.end_byte] if parameters_node else "()"

                    line_no = node.start_point[0] + 1
                    outline.append("  " * depth + f"[{line_no}] def {name}{parameters}")
                    depth += 1
                
                for child in node.children:
                    traverse(child, depth)
                    
            traverse(tree.root_node)
            return "\n".join(outline) if outline else "No classes or functions found."
        except Exception as e:
            return str(e)

    def get_file_outline_jsx(self, file_path: str) -> str | list[str]:
        return self.get_file_outline_js(file_path)

    def get_file_outline_ts(self, file_path: str) -> str | list[str]:
        try:
            code, tree = self._read_and_parse(file_path, "tree_sitter_typescript", "language_typescript")
            outline = []
            
            def traverse(node, depth=0):
                if node.type == "class_declaration":
                    name_node = node.child_by_field_name("name")
                    name = code[name_node.start_byte:name_node.end_byte] if name_node else "Anonymous"
                    line_no = node.start_point[0] + 1
                    outline.append("  " * depth + f"[{line_no}] class {name}")
                    depth += 1
                elif node.type in ["function_declaration", "method_definition"]:
                    name_node = node.child_by_field_name("name")
                    name = code[name_node.start_byte:name_node.end_byte] if name_node else "anonymous"

                    parameters_node = node.child_by_field_name("parameters")
                    parameters = code[parameters_node.start_byte:parameters_node.end_byte] if parameters_node else "()"

                    return_type_node = node.child_by_field_name("return_type")
                    return_type = code[return_type_node.start_byte:return_type_node.end_byte] if return_type_node else ""

                    line_no = node.start_point[0] + 1
                    outline.append("  " * depth + f"[{line_no}] def {name}{parameters} -> {return_type}")
                    depth += 1
                
                for child in node.children:
                    traverse(child, depth)
                    
            traverse(tree.root_node)
            return "\n".join(outline) if outline else "No classes or functions found."
        except Exception as e:
            return str(e)

    def get_file_outline_tsx(self, file_path: str) -> str | list[str]:
        try:
            code, tree = self._read_and_parse(file_path, "tree_sitter_typescript", "language_tsx")
            outline = []
            
            def traverse(node, depth=0):
                if node.type == "class_declaration":
                    name_node = node.child_by_field_name("name")
                    name = code[name_node.start_byte:name_node.end_byte] if name_node else "Anonymous"
                    line_no = node.start_point[0] + 1
                    outline.append("  " * depth + f"[{line_no}] class {name}")
                    depth += 1
                elif node.type in ["function_declaration", "method_definition"]:
                    name_node = node.child_by_field_name("name")
                    name = code[name_node.start_byte:name_node.end_byte] if name_node else "anonymous"

                    parameters_node = node.child_by_field_name("parameters")
                    parameters = code[parameters_node.start_byte:parameters_node.end_byte] if parameters_node else "()"

                    return_type_node = node.child_by_field_name("return_type")
                    return_type = code[return_type_node.start_byte:return_type_node.end_byte] if return_type_node else ""

                    line_no = node.start_point[0] + 1
                    outline.append("  " * depth + f"[{line_no}] def {name}{parameters} -> {return_type}")
                    depth += 1
                
                for child in node.children:
                    traverse(child, depth)
                    
            traverse(tree.root_node)
            return "\n".join(outline) if outline else "No classes or functions found."
        except Exception as e:
            return str(e)

    def get_file_outline_rust(self, file_path: str) -> str | list[str]:
        try:
            code, tree = self._read_and_parse(file_path, "tree_sitter_rust")
            outline = []
            
            def traverse(node, depth=0):
                if node.type in ["struct_item", "trait_item", "impl_item"]:
                    name_node = node.child_by_field_name("name")
                    if not name_node and node.type == "impl_item":
                        name_node = node.child_by_field_name("type")
                        
                    name = code[name_node.start_byte:name_node.end_byte] if name_node else "Anonymous"
                    type_name = "impl" if node.type == "impl_item" else ("struct" if node.type == "struct_item" else "trait")
                    line_no = node.start_point[0] + 1
                    outline.append("  " * depth + f"[{line_no}] {type_name} {name}")
                    depth += 1
                elif node.type == "function_item":
                    name_node = node.child_by_field_name("name")
                    name = code[name_node.start_byte:name_node.end_byte] if name_node else "anonymous"

                    parameters_node = node.child_by_field_name("parameters")
                    parameters = code[parameters_node.start_byte:parameters_node.end_byte] if parameters_node else "()"

                    return_type_node = node.child_by_field_name("return_type")
                    return_type = code[return_type_node.start_byte:return_type_node.end_byte] if return_type_node else ""

                    line_no = node.start_point[0] + 1
                    outline.append("  " * depth + f"[{line_no}] def {name}{parameters} -> {return_type}")
                    depth += 1
                
                for child in node.children:
                    traverse(child, depth)
                    
            traverse(tree.root_node)
            return "\n".join(outline) if outline else "No classes or functions found."
        except Exception as e:
            return str(e)

    def get_file_outline(self, file_path: str) -> str | list[str]:
        try:
            path_obj = self.fs.resolve_safe_path(file_path)
            ext = path_obj.suffix.lower()
            
            mapping = {
                ".py": self.get_file_outline_py,
                ".js": self.get_file_outline_js,
                ".jsx": self.get_file_outline_jsx,
                ".ts": self.get_file_outline_ts,
                ".tsx": self.get_file_outline_tsx,
                ".rs": self.get_file_outline_rust,
            }
            
            if ext not in mapping:
                return f"Outline is currently not supported for {ext} files."
                
            return mapping[ext](file_path)
            
        except Exception as e:
            return str(e)

    def get_multiple_files_outline(
        self,
        file_paths: list[str]
    ) -> dict[str, str | list[str]]:
        try:
            outline = {}

            for file_path in file_paths:
                outline[file_path] = self.get_file_outline(file_path)
            
            return outline

        except Exception as e:
            return str(e)
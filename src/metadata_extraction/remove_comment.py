import tree_sitter_c as ts_c
from typing import Generator
from tree_sitter import Language, Parser, Tree, Node
import os
import json
import subprocess

c_language = Language(ts_c.language())
c_parser = Parser(c_language)

def get_all_files(src_folder: str, dirs: list[str]) -> dict[str, str]:
    files = {}
    for d in dirs:
        for root, _, fs in os.walk(os.path.join(src_folder, d)):
            for f in fs:
                if f.endswith(".c") or f.endswith(".h") or f.endswith(".ph"):
                    file_name = os.path.join(root, f)
                    file_content = open(file_name).read()
                    replace_content = {
"""#ifdef __cplusplus
#if __cplusplus
extern "C" {
#endif""": """#ifdef __cplusplus
extern "C" {""",
"""#ifdef __cplusplus
#if __cplusplus
}
#endif""": """#ifdef __cplusplus
}""",
                    }
                    for old_content, new_content in replace_content.items():
                        file_content = file_content.replace(old_content, new_content)
                    # if "#if __cplusplus" in file_content:
                    #     file_content = file_content.replace("#if __cplusplus", "#ifdef __cplusplus")
                    new_file_name = os.path.relpath(file_name, src_folder)
                    files[new_file_name] = file_content
    return files

def remove_comments(code: str) -> str:
    root_node = c_parser.parse(bytes(code, "utf-8")).root_node
    return "\n".join(traverse_tree(root_node))

def traverse_tree(node: Node) -> list[str]:
    if node.type == "ERROR":
        raise Exception(f"Error in parsing tree: {node.text.decode('utf-8')}")
    if node.type == "comment":
        return []
    child_cnt = node.child_count
    if child_cnt == 0 or node.type in ["preproc_def", "preproc_function_def", "preproc_call", "preproc_include", "string_literal", "char_literal"]:
        return [node.text.decode("utf-8")]
    if node.type == "preproc_ifdef":
        assert child_cnt >= 2
        ret = [node.child(0).text.decode("utf-8") + " " + node.child(1).text.decode("utf-8")]
        for i in range(2, child_cnt):
            child = node.child(i)
            ret += traverse_tree(child) 
        return ret
    else:
        ret = []
        for i in range(child_cnt):
            child = node.child(i)
            ret += traverse_tree(child) 
        return ret

def recreate_files(files: dict[str, str], location: str):
    import shutil
    shutil.rmtree(location, ignore_errors=True)
    for file_name, file_content in files.items():
        new_file_name = os.path.join(location, file_name)
        os.makedirs(os.path.dirname(new_file_name), exist_ok=True)
        with open(new_file_name, "w") as f:
            f.write(file_content)

def format_all_files(src_folder: str, dirs: list[str]):
    for d in dirs:
        for root, _, fs in os.walk(os.path.join(src_folder, d)):
            for f in fs:
                if f.endswith(".c") or f.endswith(".h"):
                    file_name = os.path.join(root, f)
                    subprocess.run(["clang-format", "-i", file_name])

def preprocess(src_folder: str, dirs: list[str], tgt_folder: str) -> dict[str, str]:
    result = get_all_files(src_folder, dirs)
    for file_name, file_content in result.items():
        try:
            result[file_name] = remove_comments(file_content)
        except Exception as e:
            raise Exception(f"Error in file {file_name}: {e}")
    recreate_files(result, tgt_folder)
    format_all_files(tgt_folder, dirs)
    result = get_all_files(tgt_folder, dirs)
    return result
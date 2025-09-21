from __future__ import annotations
import os
import json

from entity.metadata import *
from entity.project import *

def c_filename_to_rust_filename(name: str) -> str:
    return name.replace("-", "_").replace(".", "_") + ".rs"

def c_filename_to_rust_package_name(name: str) -> str:
    return "crate::" + name.replace("-", "_").replace(".", "_").replace("/", "::")

def resolve_directory(filenames: list[str]) -> dict[str, dict]:
    rust_filenames = [ f.split("/") for f in filenames ]
    directories = {}
    for f in rust_filenames:
        curr_dir = directories
        for name in f:
            if name not in curr_dir:
                curr_dir[name] = {}
            curr_dir = curr_dir[name]
    return directories



def directories_to_paths(name:str, directories: dict[str, dict]) -> RustPath:
    if len(directories) == 0:
        assert name.endswith(".rs"), name
        path = RustFile(name)
        path.declarations = ["use crate::translation_utils::*;"]
    else:
        path = RustFolder(name)
        path.children = { k: directories_to_paths(k, v) for k, v in directories.items() }
    return path

def directories_to_metadata(directories: dict[str, dict]) -> RustProjectMetadata:
    proj = RustProjectMetadata()
    proj.paths = { k: directories_to_paths(k, v) for k, v in directories.items() }
    return proj


def add_mod_rs(folder: RustFolder):
    if "mod.rs" not in folder.children:
        mod_rs = RustFile("mod.rs")
        mods = []
        for k, v in folder.children.items():
            if v.type == "folder":
                mods.append(f"pub mod {k};")
            elif v.type == "file":
                mods.append(f"pub mod {k.split('.')[0]};")
        mod_rs.declarations += mods
        folder.children["mod.rs"] = mod_rs

def recursive_add_mod_rs(path: RustPath):
    if path.type == "folder":
        add_mod_rs(path)
        for k, v in path.children.items():
            recursive_add_mod_rs(v)
    elif path.type == "file":
        pass


def includes_to_declarations(includes: list[str], metadata: RustProjectMetadata, current_path: str, files: dict[str, str]) -> list[str]:
    """
    处理 include 声明
    
    Args:
        includes: include 语句列表
        metadata: Rust 项目元数据
        current_path: 当前文件的完整路径
        files: 所有文件的字典，用于查找包含的文件
    """
    declarations = []
    for i in includes:
        include_filename = ""
        if "<" in i:
            include_filename = i.split("<")[1].split(">")[0]
        elif '"' in i:
            include_filename = i.split('"')[1]
        else:
            raise Exception("Invalid include")        
        
        matching_paths = []
        for file_path in files.keys():
            if file_path.endswith("/" + include_filename) or file_path == include_filename:
                matching_paths.append(file_path)
        
        for matched_path in matching_paths:
            try:
                rust_file = find_rust_file_by_c_filename(metadata, matched_path)
                declarations.append(f"pub use {c_filename_to_rust_package_name(matched_path)}::*;")
                break
            except FileNotFoundError:
                continue
        
    
    return declarations

def fix_numeric_prefixes(files: dict[str, str], declarations: dict[str, list[str]]) -> tuple[dict[str, str], dict[str, list[str]]]:
    """检测并修复以数字开头的目录或文件名"""
    
    number_to_word = {
        '0': 'zero',
        '1': 'one', 
        '2': 'two',
        '3': 'three',
        '4': 'four',
        '5': 'five',
        '6': 'six',
        '7': 'seven',
        '8': 'eight',
        '9': 'nine'
    }
    
    def fix_path(path: str) -> str:
        """修复单个路径中的数字开头部分"""
        parts = path.split('/')
        fixed_parts = []
        
        for part in parts:
            if part and part[0].isdigit():
                i = 0
                while i < len(part) and part[i].isdigit():
                    i += 1
                
                number_part = part[:i]
                remaining_part = part[i:]
                
                english_numbers = []
                for digit in number_part:
                    english_numbers.append(number_to_word[digit])
                
                if remaining_part:
                    fixed_part = '_'.join(english_numbers) + '_' + remaining_part
                else:
                    fixed_part = '_'.join(english_numbers)
                
                fixed_parts.append(fixed_part)
            else:
                fixed_parts.append(part)
        
        return '/'.join(fixed_parts)
    
    fixed_files = {}
    path_mapping = {}
    
    for original_path, content in files.items():
        fixed_path = fix_path(original_path)
        fixed_files[fixed_path] = content
        if original_path != fixed_path:
            path_mapping[original_path] = fixed_path
    
    fixed_declarations = {}
    for name, path_list in declarations.items():
        fixed_path_list = []
        for original_path in path_list:
            fixed_path = path_mapping.get(original_path, original_path)
            if fixed_path == original_path:
                fixed_path = fix_path(original_path)
            fixed_path_list.append(fixed_path)
        fixed_declarations[name] = fixed_path_list
    
    return fixed_files, fixed_declarations


def find_rust_file_by_c_filename(proj: RustProjectMetadata, c_filename: str) -> RustFile:
    """根据C文件名查找对应的Rust文件对象"""
    rust_filename = c_filename_to_rust_filename(c_filename)
    
    result = find_rust_file_by_full_path(proj, rust_filename)
    return result


def find_rust_file_by_full_path(proj: RustProjectMetadata, full_path: str) -> RustFile:
    """根据完整路径查找Rust文件对象"""
    path_parts = full_path.split("/")
    
    current_path = proj.paths
    current_obj = None
    
    for part in path_parts:
        rust_part = c_filename_to_rust_filename(part) if part.endswith(('.c', '.h')) else part
        
        if isinstance(current_path, dict):
            if rust_part in current_path:
                current_obj = current_path[rust_part]
                if hasattr(current_obj, 'children'):
                    current_path = current_obj.children
                else:
                    break
            else:
                raise FileNotFoundError(f"Path part {rust_part} not found in {full_path}")
        else:
            raise FileNotFoundError(f"Invalid path structure at {part} in {full_path}")
    
    if current_obj and current_obj.type == "file":
        return current_obj
    else:
        raise FileNotFoundError(f"File not found at path {full_path}")

def resolve_metadata(files: dict[str, str], declarations: dict[str, list[str]]) -> RustProjectMetadata:
    files, declarations = fix_numeric_prefixes(files, declarations)
    declarations_use = {}
    for func_name, path_list in declarations.items():
        primary_path = path_list[0] if path_list else ""
        if primary_path:
            declarations_use[func_name] = f"pub use {c_filename_to_rust_package_name(primary_path)}::{func_name};"
    all_file_names = [ c_filename_to_rust_filename(f) for f in files ]
    directories = resolve_directory(all_file_names)
    metadata = directories_to_metadata(directories)
    
    root_mods = ["pub(crate) mod translation_utils;"]
    for k, v in metadata.paths.items():
        if v.type == "file":
            assert k.endswith(".rs"), k
            file_name = k.split(".")[0]
            root_mods.append(f"pub(crate) mod {file_name};")
        elif v.type == "folder":
            root_mods.append(f"pub(crate) mod {k};")
    
    metadata.paths["lib.rs"] = RustFile("lib.rs")
    metadata.paths["lib.rs"].declarations = root_mods
    
    for path in metadata.paths.values():
        recursive_add_mod_rs(path)
    
    for path in files:
        try:
            target_path = find_rust_file_by_c_filename(metadata, path)
            target_path.declarations += includes_to_declarations(files[path]["includes"], metadata, path, files)
        except FileNotFoundError as e:
            print(f"Warning: {e}")
    
    for path in files:
        try:
            target_path = find_rust_file_by_c_filename(metadata, path)
            for func_name in files[path]["declarations"]:
                if func_name in declarations_use:
                    path_list = declarations.get(func_name, [])
                    if path not in path_list:
                        target_path.declarations.append(declarations_use[func_name])
        except FileNotFoundError as e:
            print(f"Warning: {e}")
    
    for path in files:
        try:
            target_path = find_rust_file_by_c_filename(metadata, path)
            target_path.macros += [ RustCode(m) for m in files[path]["macros"]]
        except FileNotFoundError as e:
            print(f"Warning: {e}")
    
    for path in files:
        try:
            target_path = find_rust_file_by_c_filename(metadata, path)
            target_path.macro_functions += [ RustCode(mf) for mf in files[path]["macro_functions"]]
        except FileNotFoundError as e:
            print(f"Warning: {e}")
    
    for path in files:
        try:
            target_path = find_rust_file_by_c_filename(metadata, path)
            for t, v in files[path]["types"].items():
                if t != "":
                    code = RustCode(v)
                    code.rust_code = f"pub type {t} = i32;"
                    target_path.definitions.append(code)
                else:
                    for v0 in v:
                        code = RustCode(v0)
                        target_path.definitions.append(code)
        except FileNotFoundError as e:
            print(f"Warning: {e}")
    
    for path in files:
        try:
            target_path = find_rust_file_by_c_filename(metadata, path)
            for f, v in files[path]["global_variables"].items():
                code = RustCode(v)
                code.rust_code = f"pub static {f}: i32 = 0;"
                target_path.definitions.append(code)
        except FileNotFoundError as e:
            print(f"Warning: {e}")
    
    for path in files:
        try:
            target_path = find_rust_file_by_c_filename(metadata, path)
            for f, v in files[path]["functions"].items():
                code = RustCode(v)
                code.rust_code = "pub fn " + f + "() { unimplemented!(); }"
                target_path.functions.append(code)
        except FileNotFoundError as e:
            print(f"Warning: {e}")
    
    return metadata

def c_metadata_to_rust_metadata(global_config):
    proj_name = global_config.project_name
    c_metadata_dir = global_config.c_metadata_dir
    rust_metadata_dir = global_config.rust_metadata_dir
    created_project_dir = global_config.created_project_dir
    template_project_dir = global_config.template_project_dir    
    with open(os.path.join(c_metadata_dir, proj_name, "files.json"), "r") as f:
        files_data = json.load(f)
    with open(
        os.path.join(c_metadata_dir, proj_name, "declarations_location.json"),
        "r",
    ) as f:
        declarations_data = json.load(f)
    metadata = resolve_metadata(files_data, declarations_data)
    os.makedirs(os.path.join(rust_metadata_dir, proj_name), exist_ok=True)
    with open(os.path.join(rust_metadata_dir, proj_name, "metadata.json"), "w") as f:
        json.dump(metadata.__dict__(), f, indent=4)

    with open(os.path.join(rust_metadata_dir, proj_name, "metadata.json"), "r") as f:
        files_data = json.load(f)
    metadata = RustProjectMetadata.from_dict(files_data)
    print(
        f"Rust project `{proj_name}` metadata stored at {os.path.join(c_metadata_dir, proj_name)}"
    )
    proj = RustProject(proj_name, metadata, created_project_dir, template_project_dir)
    print(f"Create rust project `{proj_name}` at {proj.dir_path}")
    success, error_msg = proj.build_project()
    if success:
        print(
            f"Rust skeleton project {proj_name}(at {proj.dir_path}) build succeeded!")
    else:
        raise RustProjectCompilationFailedError(error_msg)
    return metadata
from __future__ import annotations
import os
import json

class RustProjectMetadata:
    def __init__(self):
        self.paths = {}
    
    def __dict__(self):
        return { k: v.__dict__() for k, v in self.paths.items() }
    
    @staticmethod
    def from_dict(data: dict) -> RustProjectMetadata:
        proj = RustProjectMetadata()
        proj.paths = { k: RustPath.from_dict(v) for k, v in data.items() }
        return proj
    
    def get_all(self, typ: str) -> list[RustFile]:
        all_files = []
        for k, v in self.paths.items():
            all_files.extend(v.recursive_get_all(typ))
        return all_files

    def set_mod_rs(self, mod_rs_content):
        self.paths['src'].children['mod.rs'].declarations = [mod_rs_content]

class RustPath:
    def __init__(self, typ: str):
        self.type = typ # folder or file
    
    @staticmethod
    def from_dict(data: dict) -> RustPath:
        if data["type"] == "folder":
            folder = RustFolder(data["name"])
            folder.children = { k: RustPath.from_dict(v) for k, v in data["children"].items() }
            return folder
        elif data["type"] == "file":
            file = RustFile(data["name"])
            file.declarations = data["declarations"]
            file.definitions = [ RustCode.from_dict(d) for d in data["definitions"] ]
            file.macros = [ RustCode.from_dict(m) for m in data["macros"] ]
            file.macro_functions = [ RustCode.from_dict(mf) for mf in data["macro_functions"] ]
            file.functions = [ RustCode.from_dict(f) for f in data["functions"] ]
            return file
        else:
            raise Exception("Invalid type")

    def recursive_get_all(self, typ: str) -> list[RustFile]:
        if typ == "definition":
            if self.type == "file":
                return self.definitions
            else:
                definitions = []
                for k, v in self.children.items():
                    definitions.extend(v.recursive_get_all(typ))
                return definitions
        elif typ == "macro":
            if self.type == "file":
                return self.macros
            else:
                macros = []
                for k, v in self.children.items():
                    macros.extend(v.recursive_get_all(typ))
                return macros
        elif typ == "macro_function":
            if self.type == "file":
                return self.macro_functions
            else:
                macro_functions = []
                for k, v in self.children.items():
                    macro_functions.extend(v.recursive_get_all(typ))
                return macro_functions
        elif typ == "function":
            if self.type == "file":
                return self.functions
            else:
                functions = []
                for k, v in self.children.items():
                    functions.extend(v.recursive_get_all(typ))
                return functions
        else:
            raise Exception("Invalid type")

class RustFolder(RustPath):
    def __init__(self, name: str):
        super().__init__("folder")
        self.name = name
        self.children = {}
    
    def __dict__(self):
        return {
            "name": self.name,
            "type": self.type,
            "children": { k: v.__dict__() for k, v in self.children.items() }
        }

class RustFile(RustPath):
    def __init__(self, name: str):
        super().__init__("file")
        self.name = name
        self.declarations = []
        self.definitions = []
        self.macros = []
        self.macro_functions = []
        self.functions = []
    
    def __dict__(self):
        return {
            "name": self.name,
            "type": self.type,
            "declarations": self.declarations,
            "definitions": [ d.__dict__() for d in self.definitions ],
            "macros": [ m.__dict__() for m in self.macros ],
            "macro_functions": [ mf.__dict__() for mf in self.macro_functions ],
            "functions": [ f.__dict__() for f in self.functions ]
        }

class RustCode:
    def __init__(self, c_code: str):
        self.c_code = c_code
        self.rust_code = ""
    
    def __dict__(self):
        return {
            "c_code": self.c_code,
            "rust_code": self.rust_code
        }

    @staticmethod
    def from_dict(data: dict) -> RustCode:
        code = RustCode(data["c_code"])
        code.rust_code = data["rust_code"]
        return code
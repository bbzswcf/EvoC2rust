from __future__ import annotations
import os
import json
import shutil
import time
import subprocess

from entity.metadata import *
from entity.exceptions import RustProjectCompilationFailedError

def create_under_current_dir(dir_path: str, rpath: RustPath):
    if rpath.type == "folder":
        os.makedirs(f"{dir_path}/{rpath.name}", exist_ok=True)
        for k, v in rpath.children.items():
            create_under_current_dir(f"{dir_path}/{rpath.name}", v)
    elif rpath.type == "file":
        with open(f"{dir_path}/{rpath.name}", "w") as f:
            f.write("\n".join(rpath.declarations) + "\n\n")
            for k in rpath.definitions:
                f.write(k.rust_code + "\n\n")
            for k in rpath.macros:
                f.write(k.rust_code + "\n\n")
            for k in rpath.macro_functions:
                f.write(k.rust_code + "\n\n")
            for k in rpath.functions:
                f.write(k.rust_code + "\n\n")

def create_under_current_dir_full(dir_path: str, rpath: RustPath):
    if rpath.type == "folder":
        os.makedirs(f"{dir_path}/{rpath.name}", exist_ok=True)
        for k, v in rpath.children.items():
            create_under_current_dir_full(f"{dir_path}/{rpath.name}", v)
    elif rpath.type == "file":
        with open(f"{dir_path}/{rpath.name}", "w") as f:
            f.write("\n".join(rpath.declarations) + "\n\n")
            for k in rpath.definitions:
                f.write(k.rust_code + "\n\n")
            for k in rpath.macros:
                f.write(k.rust_code + "\n\n")
            for k in rpath.macro_functions:
                f.write(k.rust_code + "\n\n")
            for k in rpath.functions:
                f.write(k.rust_code + "\n\n")


class RustProject:
    def __init__(self, name: str, metadata: RustProjectMetadata, parent_dir="./created_project", template_project_dir="./template_project", is_final=False):
        self.is_final = is_final
        if is_final:
            self.dir_path = os.path.join(parent_dir, f"{name}")
        else:
            self.dir_path = os.path.join(parent_dir, f"{name}_{int(time.time() * 1000)}")
        self.template_project_dir = template_project_dir
        self.metadata = metadata
        self.create_project()    
    
    def create_project(self):
        if not self.is_final:
            if os.path.exists(self.dir_path):
                shutil.rmtree(self.dir_path)
            os.makedirs(self.dir_path)
            for item_name in os.listdir(self.template_project_dir):
                if item_name == "src":
                    src_target = os.path.join(self.dir_path, "src")
                    os.makedirs(os.path.join(self.dir_path, "src"), exist_ok=True)
                    for item_name_1 in os.listdir(os.path.join(self.template_project_dir, "src")):
                        link_path = os.path.join(self.dir_path, "src", item_name_1)
                        absolute_item = os.path.abspath(os.path.join(self.template_project_dir, "src", item_name_1))
                        os.symlink(absolute_item, link_path)
                else:
                    link_path = os.path.join(self.dir_path, item_name)
                    absolute_item = os.path.abspath(os.path.join(self.template_project_dir, item_name))
                    os.symlink(absolute_item, link_path)
            paths = self.metadata.paths
            for k, v in paths.items():
                create_under_current_dir(os.path.join(self.dir_path, "src"), v)
        else:
            if os.path.exists(self.dir_path):
                shutil.rmtree(self.dir_path)
            os.makedirs(self.dir_path)
            for item_name in os.listdir(self.template_project_dir):
                if item_name == "src":
                    src_target = os.path.join(self.dir_path, "src")
                    os.makedirs(os.path.join(self.dir_path, "src"), exist_ok=True)
                    for item_name_1 in os.listdir(os.path.join(self.template_project_dir, "src")):
                        src_item_path = os.path.join(self.template_project_dir, "src", item_name_1)
                        target_item_path = os.path.join(self.dir_path, "src", item_name_1)
                        if os.path.isdir(src_item_path):
                            shutil.copytree(src_item_path, target_item_path)
                        else:
                            shutil.copy2(src_item_path, target_item_path)
                else:
                    src_item_path = os.path.join(self.template_project_dir, item_name)
                    target_item_path = os.path.join(self.dir_path, item_name)
                    if os.path.isdir(src_item_path):
                        shutil.copytree(src_item_path, target_item_path)
                    else:
                        shutil.copy2(src_item_path, target_item_path)
            paths = self.metadata.paths
            for k, v in paths.items():
                create_under_current_dir_full(os.path.join(self.dir_path, "src"), v)

    def build_project(self):
        result = subprocess.run(["RUSTFLAGS=-Awarnings cargo check"], shell=True, cwd=self.dir_path, timeout=10, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        if result.returncode == 0:
            return True, ""
        else:
            error_msg = result.stderr.decode("utf-8")
            return False, error_msg
    
    def test_project(self):
        try:
            result = subprocess.run(["RUSTFLAGS=-Awarnings cargo test --tests -- --test-threads=1"], shell=True, cwd=self.dir_path, timeout=300, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
            if result.returncode == 0:
                return True, ""
            else:
                error_msg = result.stderr.decode("utf-8")
                return False, error_msg
        except subprocess.TimeoutExpired:
            return False, "Timeout"

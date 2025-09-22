from config.global_config import GlobalConfig
from metadata_extraction.rust_metadata import c_metadata_to_rust_metadata
from llm.client_qwen import GenerationClient
from cache.cache import ProjectCache
from code_optim.code_gen import code_generation, code_verification
from entity.project import RustProject

import argparse
import os

if __name__ == "__main__":
    final_report = {}
    projects = ["c-algorithms"]

    config = GlobalConfig()
    work_dir = os.path.join(config.final_project_dir, "evo-c2rust-v2-qwen")
    
    # 注：运行此脚本需要先运行increment_fill_rate.py脚本产生cache结果

    for project_name in projects:          
        config = GlobalConfig()
        config.project_dir = "./data/increment/project"
        config.c_metadata_dir = "./data/increment/c_metadata"
        config.rust_metadata_dir = "./data/increment/rust_metadata"
        config.project_name = project_name
        rust_metadata = c_metadata_to_rust_metadata(config)
        cache = ProjectCache(config, cache_dir=f"./data/increment/cache/evo-c2rust-v2-qwen-gen-only")
        client = GenerationClient(config)
        code_generation(config, rust_metadata, cache, client)
        project = RustProject(name=config.project_name, metadata=rust_metadata, parent_dir=work_dir, template_project_dir=config.template_project_dir, is_final=True)
    
    unsafe_api_cnt = """tree-grepper -q rust '((function_item (function_modifiers) @_m)@f (#match? @_m "unsafe"))' | grep ':f:' | wc -l"""
    all_api_cnt = """tree-grepper -q rust '((function_item)@f)' | grep ':f:' | wc -l"""
    unsafe_code_cnt1 = """tree-grepper -q rust '((unsafe_block)@b)' | sed -e 's/^.*:b://' | wc -c"""
    unsafe_code_cnt2 = """tree-grepper -q rust '((function_item (function_modifiers) @_m)@f (#match? @_m "unsafe"))' | sed -e 's/^.*:f://' | wc -c"""
    all_code_cnt = """tree-grepper -q rust '((function_item))' | sed -e 's/^.*:f://' | wc -c"""

    import subprocess
    
    unsafe_api_cnt = subprocess.check_output(unsafe_api_cnt, shell=True, cwd=work_dir).decode().strip()
    all_api_cnt = subprocess.check_output(all_api_cnt, shell=True, cwd=work_dir).decode().strip()
    unsafe_code_cnt1 = subprocess.check_output(unsafe_code_cnt1, shell=True, cwd=work_dir).decode().strip()
    unsafe_code_cnt2 = subprocess.check_output(unsafe_code_cnt2, shell=True, cwd=work_dir).decode().strip()
    all_code_cnt = subprocess.check_output(all_code_cnt, shell=True, cwd=work_dir).decode().strip()
    print(f"Unsafe API Count: {unsafe_api_cnt}, All API Count: {all_api_cnt}, Unsafe Code Count 1: {unsafe_code_cnt1}, Unsafe Code Count 2: {unsafe_code_cnt2}, All Code Count: {all_code_cnt}")

    unsafe_api_rate = int(unsafe_api_cnt) / int(all_api_cnt) if int(all_api_cnt) > 0 else 0
    unsafe_code_rate = (int(unsafe_code_cnt1) + int(unsafe_code_cnt2)) / int(all_code_cnt) if int(all_code_cnt) > 0 else 0

    method_report = {
        "safe_api_rate": 1 - unsafe_api_rate,
        "safe_code_rate": 1 - unsafe_code_rate
    }
    print(f"Safe API Rate: {1 - unsafe_api_rate}, safe Code Rate: {1 - unsafe_code_rate}")
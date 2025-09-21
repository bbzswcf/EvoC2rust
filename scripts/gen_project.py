from config.global_config import GlobalConfig
from metadata_extraction.rust_metadata import c_metadata_to_rust_metadata
from llm.client import GenerationClient
from cache.cache import ProjectCache
from code_optim.code_gen import code_generation, code_verification
from entity.project import RustProject

import argparse

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Run code verification on a project.")
    parser.add_argument("--project_name", type=str, required=True, help="Name of the project to run code verification on.")
    args = parser.parse_args()
    
    config = GlobalConfig()
    config.project_dir = "./data/default/project"
    config.c_metadata_dir = "./data/default/c_metadata"
    config.rust_metadata_dir = "./data/default/rust_metadata"
    
    config.project_name = args.project_name
    rust_metadata = c_metadata_to_rust_metadata(config)
    cache = ProjectCache(config, cache_dir="./data/default/cache/evo-c2rust-v2-ds-llm-repair")
    client = GenerationClient(config)
    
    # 这一步只将cache中的内容填充到rust_metadata中!
    #使用code_generation会直接填充，不进行验证
    code_generation(config, rust_metadata, cache, client)

    # 使用code_verification只填充能通过编译的片段，其他以placeholder代替
    # code_verification(config, rust_metadata, cache)

    # 在final_project目录下生成Rust项目
    project = RustProject(name=config.project_name, metadata=rust_metadata, parent_dir=config.final_project_dir, template_project_dir=config.template_project_dir, is_final=True)

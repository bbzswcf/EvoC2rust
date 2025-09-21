from config.global_config import GlobalConfig
from metadata_extraction.rust_metadata import c_metadata_to_rust_metadata
from llm.client import GenerationClient
from cache.cache import ProjectCache
from code_optim.code_gen import code_generation

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
    client = GenerationClient(config)

    rust_metadata = c_metadata_to_rust_metadata(config)
    cache = ProjectCache(config, cache_dir="./data/default/cache/evo-c2rust-v2-ds-gen-only")
    
    # 多线程
    code_generation(config, rust_metadata, cache, client, multi_process=True, threads_num=5)
    # # 单线程
    # code_generation(config, rust_metadata, cache, client, multi_process=False)
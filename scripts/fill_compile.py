from config.global_config import GlobalConfig
from metadata_extraction.rust_metadata import c_metadata_to_rust_metadata
from cache.cache import ProjectCache
from code_optim.code_gen import blankfill_compilation_verification
from entity.project import RustProject
import os
import argparse
import json

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Run code verification on a project.")
    parser.add_argument("--project_name", type=str, required=True, help="Name of the project to run code verification on.")
    args = parser.parse_args()
    
    config = GlobalConfig()
    config.project_dir = "./data/fill/project"
    config.c_metadata_dir = "./data/fill/c_metadata"
    config.rust_metadata_dir = "./data/fill/rust_metadata"

    config.project_name = args.project_name
    rust_metadata = c_metadata_to_rust_metadata(config)
    gold_cache = ProjectCache(config, cache_dir="./data/fill/cache/gold")
    cache = ProjectCache(config, cache_dir="./data/fill/cache/evo-c2rust-v2-ds-llm-repair")
    report = blankfill_compilation_verification(config, rust_metadata, gold_cache, cache)
    with open("report.json", "w") as f:
        json.dump(report, f, indent=4)
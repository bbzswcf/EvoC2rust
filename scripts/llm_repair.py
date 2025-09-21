from config.global_config import GlobalConfig
from metadata_extraction.rust_metadata import c_metadata_to_rust_metadata
from llm.client import GenerationClient
from cache.cache import ProjectCache
from code_optim.code_optim import code_optimization
from code_optim.predefined_agents import get_llm_repair_agent

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
    old_cache = ProjectCache(config, cache_dir="./data/default/cache/evo-c2rust-v2-ds-rule-fix")
    new_cache = ProjectCache(config, cache_dir="./data/default/cache/evo-c2rust-v2-ds-llm-repair")
    
    # 设置优化Agent
    LLM_REPAIR_AGENT = get_llm_repair_agent(config, rust_metadata, client)

    report = code_optimization(config, rust_metadata, old_cache, new_cache, client,
        optimizations={
            "function": [LLM_REPAIR_AGENT],
        })
    
    # 输出结果报告
    with open("report.json", "w") as f:
        import json
        json.dump(report, f, indent=4)
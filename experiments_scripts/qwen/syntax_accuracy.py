from config.global_config import GlobalConfig
from metadata_extraction.c_metadata import extract_c_metadata_from_project
from metadata_extraction.rust_metadata import c_metadata_to_rust_metadata
from cache.cache import ProjectCache
from code_optim.code_gen import  code_generation, code_verification, blankfill_compilation_verification
from code_optim.code_optim import code_optimization
from code_optim.predefined_agents import (
    get_fix_mismatched_delim_agent,
    get_implicit_casting_removal_agent,
    get_as_bool_removal_agent,
    get_struct_index_advancement_agent,
    get_definition_replace_agent,
    get_llm_repair_agent,
)

from llm.client_qwen import GenerationClient


import argparse

if __name__ == "__main__":
    projects = ["c-algorithms"]
    method_report = {}
    all_proj_total_cnt = 0
    all_proj_passed_cnt = 0
    all_proj_function_cnt = 0
    all_proj_function_passed_cnt = 0

    for project_name in projects:          
        config = GlobalConfig()
        config.project_dir = "./data/fill/project"
        config.c_metadata_dir = "./data/fill/c_metadata"
        config.rust_metadata_dir = "./data/fill/rust_metadata"
        config.project_name = project_name

        extract_c_metadata_from_project(config)
        rust_metadata = c_metadata_to_rust_metadata(config)
        client = GenerationClient(config)

        cache_0 = ProjectCache(config, cache_dir="./data/fill/cache/evo-c2rust-v2-qwen-gen-only")
        cache_1 = ProjectCache(config, cache_dir="./data/fill/cache/evo-c2rust-v2-qwen-delim-fix")
        cache_2 = ProjectCache(config, cache_dir="./data/fill/cache/evo-c2rust-v2-qwen-rule-fix")
        cache_3 = ProjectCache(config, cache_dir="./data/fill/cache/evo-c2rust-v2-qwen-llm-repair")
        gold_cache = ProjectCache(config, cache_dir="./data/fill/gold_cache")

        code_generation(config, rust_metadata, cache_0, client, multi_process=False)
        rust_metadata = c_metadata_to_rust_metadata(config)
        report_0 = code_verification(config, rust_metadata, cache_0)

        rust_metadata = c_metadata_to_rust_metadata(config)
        FIX_MISMATCHED_DELIM_AGENT = get_fix_mismatched_delim_agent(config, rust_metadata, client)
        report_1 = code_optimization(config, rust_metadata, cache_0, cache_1, client,
            optimizations={
                "macro": [FIX_MISMATCHED_DELIM_AGENT],
                "macro_function": [FIX_MISMATCHED_DELIM_AGENT],
                "definition": [FIX_MISMATCHED_DELIM_AGENT],
                "function": [FIX_MISMATCHED_DELIM_AGENT],
            })

        rust_metadata = c_metadata_to_rust_metadata(config)
        IMPLICIT_CASTING_REMOVAL_AGENT = get_implicit_casting_removal_agent(config, rust_metadata)
        AS_BOOL_REMOVAL_AGENT = get_as_bool_removal_agent(config, rust_metadata)
        STRUCT_INDEX_ADVANCEMENT_AGENT = get_struct_index_advancement_agent(config, rust_metadata)
        DEFINITION_REPLACE_AGENT = get_definition_replace_agent(config, rust_metadata)
        report_2 = code_optimization(config, rust_metadata, cache_1, cache_2, client,
            optimizations={
                "definition": [DEFINITION_REPLACE_AGENT],
                "function": [IMPLICIT_CASTING_REMOVAL_AGENT, AS_BOOL_REMOVAL_AGENT, STRUCT_INDEX_ADVANCEMENT_AGENT],
            })
        
        rust_metadata = c_metadata_to_rust_metadata(config)
        LLM_REPAIR_AGENT = get_llm_repair_agent(config, rust_metadata, client)
        report_3 = code_optimization(config, rust_metadata, cache_2, cache_3, client,
            optimizations={
                "function": [LLM_REPAIR_AGENT],
            })
        
        rust_metadata = c_metadata_to_rust_metadata(config)
        report_4 = blankfill_compilation_verification(config, rust_metadata, gold_cache, cache_3)

        items = ["macro", "macro_function", "definition", "function"]
        calcs = ["all_cnt", "passed_cnt", "pass_rate"]
        proj_report = {item: {calc: report_4[item][calc] for calc in calcs} for item in items}
        total_all_cnt = sum(proj_report[item]["all_cnt"] for item in items)
        total_passed_cnt = sum(proj_report[item]["passed_cnt"] for item in items)
        total_passed_rate = total_passed_cnt / total_all_cnt
        all_proj_total_cnt += total_all_cnt
        all_proj_passed_cnt += total_passed_cnt
        all_proj_function_cnt += proj_report["function"]["all_cnt"]
        all_proj_function_passed_cnt += proj_report["function"]["passed_cnt"]
        proj_report["total"] = {"all_cnt": total_all_cnt, "passed_cnt": total_passed_cnt, "pass_rate": total_passed_rate}
        method_report[project_name] = proj_report
    all_proj_total_pass_rate = all_proj_passed_cnt / all_proj_total_cnt
    all_proj_function_pass_rate = all_proj_function_passed_cnt / all_proj_function_cnt
    method_report["all"] = {"total" : {"all_cnt": all_proj_total_cnt, "passed_cnt": all_proj_passed_cnt, "pass_rate": all_proj_total_pass_rate}, "function": {"all_cnt": all_proj_function_cnt, "passed_cnt": all_proj_function_passed_cnt, "pass_rate": all_proj_function_pass_rate}}
    
    with open("report.json", "w") as f:
        import json
        json.dump(method_report, f, indent=4)
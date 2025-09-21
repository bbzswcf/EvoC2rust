from config.global_config import GlobalConfig
from llm.client import GenerationClient
from llm.generation import update_codes
from cache.cache import ProjectCache
from entity.metadata import RustProjectMetadata
from entity.project import RustProject
from tqdm import tqdm

import os

def code_generation(
    config: GlobalConfig,
    metadata: RustProjectMetadata,
    cache: ProjectCache,
    client: GenerationClient,
    multi_process: bool = False,
    threads_num: int = 10,
):
    project_name = config.project_name
    for typ in ["macro", "macro_function", "definition", "dummy_function", "function"]:
        # single_type_code_generation(config, metadata, typ, cache=cache, client=client)
        print(f"Project {project_name}: Start {typ} code generation.")
        codes = metadata.get_all(typ) if typ != "dummy_function" else metadata.get_all("function")
        update_codes(client, typ, codes, cache, multi_process=multi_process, threads_num=threads_num)

def code_verification(
    config: GlobalConfig,
    metadata: RustProjectMetadata,
    cache: ProjectCache
):  
    report = {}
    for typ in ["macro", "macro_function", "definition", "dummy_function", "function"]:
        typ_report = {
            "all_cnt": 0,
            "passed_cnt": 0,
            "pass_rate": 0.0,
            "messages": [],
        }
        print(f"Project {config.project_name}: Start {typ} compilation verification.")
        codes = metadata.get_all(typ) if typ != "dummy_function" else metadata.get_all("function")
        all_cnt = len(codes)
        failed_cnt = 0
        for c in tqdm(codes):            
            placeholder_code = c.rust_code # The placeholder code to use if compilation fails
            c.rust_code = cache.get(typ, c.c_code)
            curr_cache_path = os.path.join(cache.caches[typ].path, cache.caches[typ].cache_index[c.c_code], "result.rs")
            proj = RustProject(config.project_name, metadata, config.created_project_dir, config.template_project_dir)
            success, error_msg = proj.build_project()
            if not success:                
                # print(f"Compilation failed for {typ} at {curr_cache_path}")
                failed_cnt += 1
                failed_message = {
                    "c_code": c.c_code,
                    "rust_code": c.rust_code,
                    "error_msg": error_msg,
                }
                c.rust_code = placeholder_code
                typ_report["messages"].append(failed_message)
        typ_report["all_cnt"] = all_cnt
        typ_report["passed_cnt"] = all_cnt - failed_cnt
        if all_cnt > 0:
            typ_report["pass_rate"] = (all_cnt - failed_cnt) / all_cnt
        else:
            typ_report["pass_rate"] = 100.0
        print(f"{typ} compilation verification completed. Passed: {typ_report['passed_cnt']}/{typ_report['all_cnt']} ({typ_report['pass_rate'] * 100:.2f}%)")
        report[typ] = typ_report
    return report

def blankfill_compilation_verification(
    config: GlobalConfig,
    metadata: RustProjectMetadata,
    gold_cache: ProjectCache,
    cache: ProjectCache,
):  
    report = {}
    for typ in ["macro", "macro_function", "definition", "function"]:
        codes = metadata.get_all(typ)
        # fill codes with gold cache
        for c in tqdm(codes):
            c.rust_code = gold_cache.get(typ, c.c_code)
    # verify gold answer
    proj = RustProject(config.project_name, metadata, config.created_project_dir, config.template_project_dir)
    success, error_msg = proj.test_project()
    if not success:
        print(f"Gold answer test failed: {error_msg}")
        return None
    for typ in ["macro", "macro_function", "definition", "function"]:
        typ_report = {
            "all_cnt": 0,
            "passed_cnt": 0,
            "pass_rate": 0.0,
            "messages": [],
        }
        print(f"Project {config.project_name}: Start {typ} fill testing.")
        codes = metadata.get_all(typ)
        all_cnt = len(codes)
        failed_cnt = 0
        for c in tqdm(codes):            
            original_code = c.rust_code
            c.rust_code = cache.get(typ, c.c_code)
            curr_cache_path = os.path.join(cache.caches[typ].path, cache.caches[typ].cache_index[c.c_code], "result.rs")
            proj = RustProject(config.project_name, metadata, config.created_project_dir, config.template_project_dir)
            success, error_msg = proj.build_project()
            if not success:
                failed_cnt += 1
                failed_message = {
                    "c_code": c.c_code,
                    "rust_code": c.rust_code,
                    "error_msg": error_msg,
                }
                typ_report["messages"].append(failed_message)
            c.rust_code = original_code
        typ_report["all_cnt"] = all_cnt
        typ_report["passed_cnt"] = all_cnt - failed_cnt
        if all_cnt > 0:
            typ_report["pass_rate"] = (all_cnt - failed_cnt) / all_cnt
        else:
            typ_report["pass_rate"] = 100.0
        print(f"{typ} fill testing completed. Passed: {typ_report['passed_cnt']}/{typ_report['all_cnt']} ({typ_report['pass_rate'] * 100:.2f}%)")
        report[typ] = typ_report
    return report


def blankfill_test(
    config: GlobalConfig,
    metadata: RustProjectMetadata,
    gold_cache: ProjectCache,
    cache: ProjectCache,
):  
    report = {}
    for typ in ["macro", "macro_function", "definition", "function"]:
        codes = metadata.get_all(typ)
        # fill codes with gold cache
        for c in tqdm(codes):
            c.rust_code = gold_cache.get(typ, c.c_code)
    # verify gold answer
    proj = RustProject(config.project_name, metadata, config.created_project_dir, config.template_project_dir)
    success, error_msg = proj.test_project()
    if not success:
        print(f"Gold answer verification failed: {error_msg}")
        return None
    for typ in ["macro", "macro_function", "definition", "function"]:
        typ_report = {
            "all_cnt": 0,
            "passed_cnt": 0,
            "pass_rate": 0.0,
            "messages": [],
        }
        print(f"Project {config.project_name}: Start {typ} fill testing.")
        codes = metadata.get_all(typ)
        all_cnt = len(codes)
        failed_cnt = 0
        for c in tqdm(codes):            
            original_code = c.rust_code
            c.rust_code = cache.get(typ, c.c_code)
            curr_cache_path = os.path.join(cache.caches[typ].path, cache.caches[typ].cache_index[c.c_code], "result.rs")
            proj = RustProject(config.project_name, metadata, config.created_project_dir, config.template_project_dir)
            success, error_msg = proj.test_project()
            if not success:
                failed_cnt += 1
                failed_message = {
                    "c_code": c.c_code,
                    "rust_code": c.rust_code,
                    "error_msg": error_msg,
                }
                typ_report["messages"].append(failed_message)
            c.rust_code = original_code
        typ_report["all_cnt"] = all_cnt
        typ_report["passed_cnt"] = all_cnt - failed_cnt
        if all_cnt > 0:
            typ_report["pass_rate"] = (all_cnt - failed_cnt) / all_cnt
        else:
            typ_report["pass_rate"] = 100.0
        print(f"{typ} fill testing completed. Passed: {typ_report['passed_cnt']}/{typ_report['all_cnt']} ({typ_report['pass_rate'] * 100:.2f}%)")
        report[typ] = typ_report
    return report
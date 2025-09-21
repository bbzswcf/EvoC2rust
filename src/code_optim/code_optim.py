from config.global_config import GlobalConfig
from llm.client import GenerationClient
from llm.generation import update_codes
from cache.cache import ProjectCache
from entity.metadata import RustProjectMetadata
from entity.project import RustProject
from code_optim.optimization_agent import OptimizationAgent
from tqdm import tqdm

import os

def code_optimization(
    config: GlobalConfig,
    metadata: RustProjectMetadata,
    old_cache: ProjectCache,
    new_cache: ProjectCache,
    client: GenerationClient,
    optimizations: dict[str, list[OptimizationAgent]],
):  
    report = {}
    for typ in ["macro", "macro_function", "definition", "dummy_function", "function"]:
        typ_report = {
            "all_cnt": 0,
            "passed_cnt": 0,
            "pass_rate": 0.0,
            "messages": [],
        }
        print(f"Project {config.project_name}: Start {typ} optimization and compilation verification.")
        codes = metadata.get_all(typ) if typ != "dummy_function" else metadata.get_all("function")
        all_cnt = len(codes)
        failed_cnt = 0
        for c in tqdm(codes):            
            placeholder_code = c.rust_code # The placeholder code to use if compilation fails
            original_rust_code = old_cache.get(typ, c.c_code)
            if not new_cache.find(typ, c.c_code):
                c.rust_code = original_rust_code
                curr_cache_path = os.path.join(old_cache.caches[typ].path, old_cache.caches[typ].cache_index[c.c_code], "result.rs")
                proj = RustProject(config.project_name, metadata, config.created_project_dir, config.template_project_dir)
                success, original_error_msg = proj.build_project()
                if success:
                    new_cache.update(typ, c.c_code, c.rust_code)
                    continue
                else:
                    curr_code, status, error_msg = c.rust_code, False, original_error_msg
                    if typ in optimizations:
                        for o in optimizations[typ]:
                            curr_code, status, error_msg = o.try_optimize(c)
                    new_cache.update(typ, c.c_code, curr_code)
                    if not status:
                        failed_cnt += 1
                        failed_message = {
                            "c_code": c.c_code,
                            "original_rust_code": original_rust_code,
                            "optimized_rust_code": curr_code,
                            "original_error_msg": original_error_msg,
                            "optimized_error_msg": error_msg,
                        }
                        c.rust_code = placeholder_code
                        typ_report["messages"].append(failed_message)
            else:
                # no need to optimize, just verify the cached code
                c.rust_code = new_cache.get(typ, c.c_code)
                curr_cache_path = os.path.join(new_cache.caches[typ].path, new_cache.caches[typ].cache_index[c.c_code], "result.rs")
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
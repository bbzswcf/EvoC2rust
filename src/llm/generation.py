from tqdm import tqdm
from entity.metadata import RustCode
from entity.exceptions import CallLLMTimeoutError
from llm.client import GenerationClient
from cache.cache import ProjectCache

def merge_prompt(prompt, c_code):
    return prompt + f"""\
Source:
```c
{c_code.strip()}
```

Translation:
```rust
"""

def merge_repair_prompt(prompt, c_code, compiler_msg):
    return prompt + f"""\
Source:
```rust
{c_code.strip()}
```

Error Message:{compiler_msg}

Fixed Code:
```rust
"""

def get_delim_repair_candidates(client, code, compiler_msg):
    text = merge_repair_prompt(client.config.delim_repair_prompt, code, compiler_msg)
    response = client.get_response(text)
    return [response]


def get_repair_candidates(client, code, compiler_msg):
    text = merge_repair_prompt(client.config.repair_prompt, code, compiler_msg)
    response = client.get_response(text)
    return [response]

def get_llm_gen_result(client, code, prompt):
    text = merge_prompt(prompt, code)
    response = client.get_response(text)
    return response

def get_llm_gen_result_with_cache(client, c_code, prompt, cache, typ):
    if cache.find(typ, c_code):
        return cache.get(typ, c_code)
    else:
        rust_code = get_llm_gen_result(client, c_code, prompt)
        return rust_code

def update_codes(client, typ, codes: list[RustCode], cache: ProjectCache,
multi_process=False, threads_num=10):
    prompts = {
        "definition": client.config.definition_prompt,
        "macro": client.config.macro_prompt,
        "macro_function": client.config.macro_function_prompt,
        "dummy_function": client.config.dummy_function_prompt,
        "function": client.config.function_prompt,
    }
    prompt = prompts[typ]
    if not multi_process:
        for c in tqdm(codes):
            if cache.find(typ, c.c_code):
                c.rust_code = cache.get(typ, c.c_code)
                continue
            c.rust_code = get_llm_gen_result(client, c.c_code, prompt)
            if cache is not None:
                cache.update(typ, c.c_code, c.rust_code)
        return
    else:
        from pebble import ProcessPool
        with ProcessPool(max_workers=threads_num) as pool:
            futures = []
            for c in codes:
                future = pool.schedule(
                    get_llm_gen_result_with_cache, args=[client, c.c_code, prompt, cache, typ], timeout=300
                )
                futures.append((c, future))
            for c, future in tqdm(futures):
                try:
                    rust_code = future.result()
                    c.rust_code = rust_code
                    cache.update(typ, c.c_code, c.rust_code)
                except Exception as e:
                    raise CallLLMTimeoutError(e)

def translate_code(client, typ, code: str):
    prompts = {
        "definition": client.config.definition_prompt,
        "macro": client.config.macro_prompt,
        "macro_function": client.config.macro_function_prompt,
        "dummy_function": client.config.dummy_function_prompt,
        "function": client.config.function_prompt,
    }
    prompt = prompts[typ]
    return get_llm_gen_result(client, code, prompt)
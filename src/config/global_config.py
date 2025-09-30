import configparser
import os

from config.prompt_config import *

def read_config():
    config = configparser.ConfigParser()
    # Ensure the path is correct relative to this file
    config_path = os.path.join(os.path.dirname(__file__), 'llm_config.ini')
    if os.path.exists(config_path):
        config.read(config_path)
    return config

class GlobalConfig:
    def __init__(self):
        config = read_config()
        self.project_name = None
        self.project_dir = "./data/default/project"
        self.created_project_dir = "./.tmp/created_project"
        self.c_metadata_dir = "./data/default/c_metadata"
        # self.src_folders = ["include", "src"]
        self.src_folders = [""]
        self.macros = {
            "INI_API": "",
            "_Nonnull": "",
            "_Nullable": "",
            "ZIP_EXTERN": "",
            "LUALIB_API": "",
            "LINMATH_H_FUNC": "static",
            "DECLSPEC_IMPORT": "",
            "WINUSERAPI": "",
            "WINAPI": "",
            "WSAAPI": "",
            "MSVCRT$": "",
            "LUALIB_API": "",
            "ALWAYS_INLINE": "inline",
            "ALWAYS_NO_INLINE": "",
            "STATIC": "static",
            "HIDDEN": "",
            "CMPTLZ_HIDDEN": "",
            "TARGET_ATTRIBUTE_AUTO": "",
            "RAPIDLZ_ALWAYS_INLINE": "inline",
            "CSTL_STATIC": "static",
            "DT_EXPORT": ""
        }
        self.replacements = {
            "__uint(type, BPF_MAP_TYPE_HASH)": 'int type;',
            "__uint(max_entries, 8192);": 'int max_entries;', 
            "__type(key, size_t);": 'int key_type;',
            "__type(value, unsigned int);": 'int value_type;',
            "MORTON_PURE;": ";",
            "__asm": "asm",
            "args...": "...",
            "##args": "__VA_ARGS__",
            "#if __cplusplus\nextern \"C\" {\n#endif": "extern \"C\" {\n",
            "#if __cplusplus\n}\n#endif": "}\n",
            "#if __cplusplus\n}\n\n#endif": "}\n",
            "__attribute__((aligned(16)))": "",
            "__attribute__((aligned(1)))": ""
        }
        self.rust_metadata_dir = "./data/default/rust_metadata"
        self.template_project_dir = "./data/project_template/safelevel-0"

        self.definition_prompt = definition_prompt
        self.macro_prompt = macro_prompt
        self.macro_function_prompt = macro_function_prompt
        self.dummy_function_prompt = dummy_function_prompt
        self.function_prompt = function_prompt
        self.delim_repair_prompt = delim_repair_prompt
        self.repair_prompt = repair_prompt

        self.api_key = config.get('llm', 'api_key')
        self.base_url = config.get('llm', 'base_url')
        self.model_name = config.get('llm', 'model_name')

        self.cache_dir = "./data/default/cache"
        self.final_project_dir = "./final_project"
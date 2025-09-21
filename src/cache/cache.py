import os
import json
from pathlib import Path

class Cache:
    def __init__(self, cache_dir, name):
        self.path = os.path.join(cache_dir, name)
        self.cache_index = {}
        self.cache = {}
        if not os.path.exists(self.path):
            os.makedirs(self.path)
            with open(os.path.join(self.path, f"index.json"), "w") as f:
                json.dump({}, f)
        else:
            with open(os.path.join(self.path, f"index.json"), "r") as f:
                self.cache_index = json.load(f)
            for k, path in self.cache_index.items():
                with open(os.path.join(self.path, path, "result.rs"), "r") as f:
                    self.cache[k] = f.read()

    def update(self, key, value):
        if key in self.cache_index:
            new_idx = self.cache_index[key]
        else:
            new_idx = str(len(self.cache))
            os.makedirs(os.path.join(self.path, str(new_idx)))
        self.cache_index[key] = new_idx
        self.cache[key] = value
        with open(os.path.join(self.path, str(new_idx), "result.rs"), "w") as f:
            f.write(value)
        with open(os.path.join(self.path, f"index.json"), "w") as f:
            json.dump(self.cache_index, f)

class ProjectCache:
    def __init__(self, config, cache_dir = None):
        self.config = config
        self.project_name = config.project_name
        if cache_dir is None:
            cache_dir = config.cache_dir
        self.caches = {
            "macro": Cache(Path(cache_dir, config.project_name), "macro"),
            "macro_function": Cache(Path(cache_dir, config.project_name), "macro_function"),
            "definition": Cache(Path(cache_dir, config.project_name), "definition"),
            "dummy_function": Cache(Path(cache_dir, config.project_name), "dummy_function"),
            "function": Cache(Path(cache_dir, config.project_name), "function"),
        }

    def get(self, type, key):
        if type in self.caches:
            return self.caches[type].cache.get(key, None)
        else:
            raise ValueError(f"Cache type '{type}' not recognized.")

    def find(self, type, key):
        if type in self.caches:
            return True if key in self.caches[type].cache else False
        else:
            raise ValueError(f"Cache type '{type}' not recognized.")    

    def update(self, type, key, value):
        if type in self.caches:
            self.caches[type].update(key, value)
        else:
            raise ValueError(f"Cache type '{type}' not recognized.")
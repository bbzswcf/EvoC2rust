from config.global_config import GlobalConfig
from metadata_extraction.c_metadata import extract_c_metadata_from_project
from metadata_extraction.rust_metadata import c_metadata_to_rust_metadata

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
    extract_c_metadata_from_project(config)
    rust_metadata = c_metadata_to_rust_metadata(config)
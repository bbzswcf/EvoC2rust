import logging
from logging.handlers import RotatingFileHandler
import sys

def setup_logger(name):
    logger = logging.getLogger(name)
    logger.setLevel(logging.DEBUG)  

    console_handler = logging.StreamHandler(sys.stdout)
    console_handler.setLevel(logging.INFO)  

    console_format = logging.Formatter(
        "%(levelname)s - %(message)s", 
    )
    console_handler.setFormatter(console_format)

    file_handler = RotatingFileHandler(
        filename="Evoc2rust.log",          
        maxBytes=5 * 1024 * 1024,    
        backupCount=3,               
        encoding="utf-8"
    )
    file_handler.setLevel(logging.DEBUG)  

    file_format = logging.Formatter(
        "%(asctime)s - %(name)s - %(lineno)d - %(levelname)s - %(message)s",
        datefmt="%Y-%m-%d %H:%M:%S"
    )
    file_handler.setFormatter(file_format)

    logger.addHandler(console_handler)
    logger.addHandler(file_handler)

    return logger

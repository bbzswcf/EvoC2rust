from code_optim.optimization_agent import OptimizationAgent, OptimizationAgentWithCompilerFeedback

from llm.generation import get_repair_candidates, get_delim_repair_candidates

def get_implicit_casting_removal_agent(config, metadata):
    def implicit_casting_removal(code):
        ret = []
        sub = ".cast()"
        start = 0
        while True:
            start = code.find(sub, start)
            if start == -1:
                break
            new_code = code[:start] + code[start + len(sub):]
            ret.append(new_code)
            start += len(sub)
        return ret
    proj_name = config.project_name
    created_project_dir = config.created_project_dir
    template_project_dir = config.template_project_dir
    agent = OptimizationAgent(proj_name, metadata, implicit_casting_removal, override=True, created_project_dir=created_project_dir, template_project_dir=template_project_dir)
    return agent

def get_as_bool_removal_agent(config, metadata):
    def as_bool_removal(code):
        ret = []
        sub = ".as_bool()"
        start = 0
        while True:
            start = code.find(sub, start)
            if start == -1:
                break
            new_code = code[:start] + code[start + len(sub):]
            ret.append(new_code)
            start += len(sub)
        return ret
    proj_name = config.project_name
    created_project_dir = config.created_project_dir
    template_project_dir = config.template_project_dir
    agent = OptimizationAgent(proj_name, metadata, as_bool_removal, override=True, created_project_dir=created_project_dir, template_project_dir=template_project_dir)
    return agent

def get_struct_index_advancement_agent(config, metadata):
    def struct_index_advancement(code):
        import re
        code_lines = code.split("\n")
        ret = []
        for i1, line in enumerate(code_lines):
            match = list(re.finditer(r"\[.*?\..*?\]", line))
            if len(match) > 0:
                new_code_lines = []
                left_spaces = len(line) - len(line.lstrip())
                new_line = ""
                curr_start = 0
                for idx, x in enumerate(match):
                    start, end = x.start(), x.end()
                    new_line += line[curr_start:start]
                    new_line += f"[tmp{idx}]"
                    curr_start = end
                    word = line[start:end]
                    index_word = word.split("[", 1)[1].split("]")[0]
                    new_code_lines.append(
                        " " * left_spaces + f"let tmp{idx} = " + index_word + ";"
                    )
                new_line += line[curr_start:]
                new_code_lines.append(new_line)
                ret.append(
                    "\n".join(code_lines[:i1] +
                            new_code_lines + code_lines[i1 + 1:])
                )
        return ret
    proj_name = config.project_name
    created_project_dir = config.created_project_dir
    template_project_dir = config.template_project_dir
    agent = OptimizationAgent(proj_name, metadata, struct_index_advancement, override=False, created_project_dir=created_project_dir, template_project_dir=template_project_dir)
    return agent

def get_fix_mismatched_delim_agent(config, metadata, client):
    def fix_mismatched_delim(code, compiler_msg):
        if "unclosed delimiter" not in compiler_msg:
            return []
        else:
            return get_delim_repair_candidates(client, code, compiler_msg)
    proj_name = config.project_name
    created_project_dir = config.created_project_dir
    template_project_dir = config.template_project_dir
    agent = OptimizationAgentWithCompilerFeedback(proj_name, metadata, fix_mismatched_delim, max_trial=5, override=False, created_project_dir=created_project_dir, template_project_dir=template_project_dir)
    return agent

def get_llm_repair_agent(config, metadata, client):
    def llm_try_repair(code, compiler_msg):
        if compiler_msg == "":
            return []
        return get_repair_candidates(client, code, compiler_msg)
    proj_name = config.project_name
    created_project_dir = config.created_project_dir
    template_project_dir = config.template_project_dir
    agent = OptimizationAgentWithCompilerFeedback(proj_name, metadata, llm_try_repair, max_trial=3, override=False, created_project_dir=created_project_dir, template_project_dir=template_project_dir)
    return agent

def get_definition_replace_agent(config, metadata):
    def definition_replace(code):
        if "#[derive(Default, Clone, Copy)]" in code:
            return [
                code.replace(
                    "#[derive(Default, Clone, Copy)]",
                    "#[derive(Default)]",
                )
            ]
        else:
            return []
    proj_name = config.project_name
    created_project_dir = config.created_project_dir
    template_project_dir = config.template_project_dir
    agent = OptimizationAgent(proj_name, metadata, definition_replace, override=True, created_project_dir=created_project_dir, template_project_dir=template_project_dir)
    return agent
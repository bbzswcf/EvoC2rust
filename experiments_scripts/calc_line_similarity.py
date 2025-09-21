def count_common_lines_diff(file1_path, file2_path):
    with open(file1_path, 'r') as file1, open(file2_path, 'r') as file2:
        file1_lines = [l.strip() for l in file1.readlines() if l.strip() != ""]
        file2_lines = [l.strip() for l in file2.readlines() if l.strip() != ""]

        counter_1 = {}
        counter_2 = {}

        for l in file1_lines:
            counter_1[l] = counter_1.get(l, 0) + 1
        for l in file2_lines:
            counter_2[l] = counter_2.get(l, 0) + 1

        counter_s = {}
        for l in counter_1.keys():
            counter_s[l] = min(counter_1.get(l, 0), counter_2.get(l, 0))
        for l in counter_2.keys():
            counter_s[l] = min(counter_1.get(l, 0), counter_2.get(l, 0))
        
        accept_lines = sum(counter_s.values())
        total_lines_generated = sum(counter_1.values())
        total_lines_gold = sum(counter_2.values())

        recall = accept_lines / total_lines_gold
        precision = accept_lines / total_lines_generated
        return {
            "total_gold_lines": total_lines_gold,
            "total_generated_lines": total_lines_generated,
            "accept_lines": accept_lines,
            "recall": recall,
            "precision": precision
        }

if __name__ == "__main__":
    final_report = {}
    projects = ["c-algorithms"]
    final_report = {}
    generated_folder = "data"
    gold_folder = "data"
    total = {
        "total_gold_lines": 0,
        "total_generated_lines": 0,
        "accept_lines": 0,
        "recall": None,
        "precision": None
    }

    for project_name in projects:
        generated_path = f"{generated_folder}/{project_name}.rs"
        gold_path = f"{gold_folder}/{project_name}.rs"

        result = count_common_lines_diff(generated_path, gold_path)
        final_report[project_name] = result

        total["accept_lines"] += result["accept_lines"]
        total["total_gold_lines"] += result["total_gold_lines"]
        total["total_generated_lines"] += result["total_generated_lines"]
    
    total["precision"] = total["accept_lines"] / total["total_generated_lines"]
    total["recall"] = total["accept_lines"] / total["total_gold_lines"]
    final_report["total"] = total

    with open("report.json", "w") as f:
        import json
        json.dump(final_report, f, indent=4)
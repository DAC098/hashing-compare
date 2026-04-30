import matplotlib.pyplot as plt
import scipy.stats as stats
import numpy as np
import math
import os
import sys

STD_DEV_OUTLIER = 3.0
P_VALUES = [95, 99, 99.9]

def remove_outliers(values):
    n = len(values)

    np_mean = np.mean(values)
    np_std = np.std(values)
    np_sem = np_std / np.sqrt(n)

    without = []

    for v in values:
        z_score = (v - np_mean) / np_std

        if math.fabs(z_score) <= STD_DEV_OUTLIER:
            without.append(v)

    return without

Kibi = 1024
Mebi = Kibi * 1024
Gibi = Mebi * 1024
Tebi = Gibi * 1024

UNITS = ["Ki", "Mi", "Gi", "Ti"]

def calc_base_2(value):
    if value < Kibi:
        return value, ""

    length = len(UNITS)
    rolling = value

    for name in UNITS:
        rolling = rolling >> 10

        if rolling < Kibi:
            return rolling, name

    return rolling, UNITS[length - 1]

def fmt_base_2(value, prefix):
    (reduced, unit) = calc_base_2(value)

    return f"{reduced}{unit}{prefix}"

def calc_stats(root_dir, name, values, create_graphs = True):
    n = len(values)

    np_mean = np.mean(values)
    np_std = np.std(values)
    np_sem = np_std / np.sqrt(n)
    np_min = np.min(values)
    np_max = np.max(values)

    msg  = f"{name} | total: {n}\n"
    msg += f"    average: {np_mean:.20f} std_dev: {np_std:.20f} sem: {np_sem:.20f}\n"
    msg += f"    minimum: {np_min:.20f} maximum: {np_max:.20f}\n"

    outliers = 0

    for v in values:
        z_score = (v - np_mean) / np_std

        if math.fabs(z_score) > STD_DEV_OUTLIER:
            outliers += 1

    msg += f"    outliers: {outliers} / {n} {(outliers / n * 100):.2f} %\n"

    p_checks = []

    for p in P_VALUES:
        p_percent = p / 100

        z_value = stats.t.ppf((1 + p_percent) / 2, df=n - 1)
        z_std_dev = z_value * np_std
        moe = z_value * np_std / np.sqrt(n)

        msg += f"    P: {p}%={p_percent} ({1 - p_percent}) {z_value:.20f}\n"
        msg += f"        Z * std_dev: {z_std_dev:.20f}\n"
        msg += f"                MoE: {moe:.20f}\n"

    print(msg)

    with open(os.path.join(root_dir, "stats.txt"), "w") as stats_file:
        stats_file.write(msg)

    if not create_graphs:
        return

    plt.plot(range(0, len(values)), values)

    plt.title(name)
    plt.xlabel("Runs")
    plt.ylabel("Time (Seconds)")
    plt.savefig(os.path.join(root_dir, "jitter.png"))
    plt.clf()

    hist_counts, hist_bins = np.histogram(values, bins=(len(values) // 10))
    plt.stairs(hist_counts, hist_bins)
    plt.title(name)
    plt.xlabel("Time (Seconds)")
    plt.ylabel("Amount")
    plt.savefig(os.path.join(root_dir, "hist.png"))
    plt.clf()

    for p in P_VALUES:
        moe_values = []

        for index in range(3, len(values)):
            subset = values[0:index]

            subset_n = len(subset)

            subset_mean = np.mean(subset)
            subset_std = np.std(subset)

            subset_z_value = stats.t.ppf((1 + (p / 100)) / 2, df=subset_n - 1)

            moe_values.append(subset_z_value * subset_std / np.sqrt(subset_n))

        plt.plot(range(3, len(values)), moe_values)

        plt.title(f"{name} MOE {p}%")
        plt.xlabel("Runs")
        plt.ylabel("MoE")
        plt.savefig(os.path.join(root_dir, f"moe_{p}.png"))
        plt.clf()

def parse_test(line):
    split = line.split(",")

    env = split[0]
    algo = split[1]
    total_bytes = int(split[2])
    chunk_size = int(split[3])

    return env, algo, total_bytes, chunk_size

def load_perf_file(data_file):
    file = open(data_file, "r")

    lines = file.readlines();
    results = []

    (env, algo, total_bytes, chunk_size) = parse_test(lines[1])

    for line in lines[3:]:
        split = line.split(",")

        time = float(split[0])

        results.append(time)

    return {
        "env": env,
        "algo": algo,
        "total_bytes": total_bytes,
        "chunk_size": chunk_size,
        "results": results
    }

def calc_perf_file(data_file, output_dir, create_graphs = True):
    data = load_perf_file(data_file)

    data_dir = os.path.join(output_dir, f"{data["env"]}_{data["algo"]}_{data["total_bytes"]}_{data["chunk_size"]}")

    if not os.path.exists(data_dir):
        os.makedirs(data_dir)

    calc_stats(
        data_dir,
        f"{data["env"]} {data["algo"]} {fmt_base_2(data["total_bytes"], "B")} {fmt_base_2(data["chunk_size"], "B")}",
        data["results"],
        create_graphs
    )

    return data

def calc_perf_outliers(data, output_dir, create_graphs = True):
    data_dir = os.path.join(output_dir, f"{data["env"]}_{data["algo"]}_{data["total_bytes"]}_{data["chunk_size"]}_wo")

    if not os.path.exists(data_dir):
        os.makedirs(data_dir)

    outliers = remove_outliers(data["results"])

    calc_stats(
        data_dir,
        f"{data["env"]} {data["algo"]} {fmt_base_2(data["total_bytes"], "B")} {fmt_base_2(data["chunk_size"], "B")} WO",
        outliers,
        create_graphs
    )

    return {
        "env": data["env"],
        "algo": data["algo"],
        "total_bytes": data["total_bytes"],
        "chunk_size": data["chunk_size"],
        "results": outliers
    }

def calc_perf_dir(input_dir, output_dir, create_graphs = True):
    env_comparisons = {}

    for entry in sorted(os.listdir(input_dir)):
        full_path = os.path.join(input_dir, entry)
        ext = os.path.splitext(os.path.basename(full_path))[1]

        if not os.path.isfile(full_path):
            continue

        if ext != ".csv":
            continue

        data = calc_perf_file(full_path, output_dir, False)
        wo = calc_perf_outliers(data, output_dir, False)

        test_name = f"{data["algo"]}_{data["total_bytes"]}_{data["chunk_size"]}"
        test_name_wo = f"{data["algo"]}_{data["total_bytes"]}_{data["chunk_size"]}_wo"

        if test_name not in env_comparisons:
            env_comparisons[test_name] = {
                "name": f"{data["algo"]} {fmt_base_2(data["total_bytes"], "B")} {fmt_base_2(data["chunk_size"], "B")}",
                "data": {}
            }

        if test_name_wo not in env_comparisons:
            env_comparisons[test_name_wo] = {
                "name": f"{data["algo"]} {fmt_base_2(data["total_bytes"], "B")} {fmt_base_2(data["chunk_size"], "B")} WO",
                "data": {}
            }

        env_comparisons[test_name]["data"][data["env"]] = data["results"]
        env_comparisons[test_name_wo]["data"][data["env"]] = wo["results"]

    comparisons_dir = os.path.join(output_dir, "comparisons")

    if not create_graphs:
        return

    if not os.path.exists(comparisons_dir):
        os.makedirs(comparisons_dir)

    for (key, data) in env_comparisons.items():
        for (name, results) in data["data"].items():
            plt.plot(range(0, len(results)), results, label=name)

        plt.title(data["name"])
        plt.xlabel("Runs")
        plt.ylabel("Time (Seconds)")
        plt.legend()
        plt.savefig(os.path.join(comparisons_dir, f"{key}.png"))
        plt.clf()

if __name__ == "__main__":
    root_dir = os.path.join(os.path.dirname(os.path.realpath(__file__)))
    perf_input = os.path.join(root_dir, "output/perf")
    perf_output = os.path.join(root_dir, "results/perf")

    plt.figure(figsize=(16,9), dpi=80)

    if not os.path.exists(perf_output):
        os.makedirs(perf_output)

    calc_perf_dir(perf_input, perf_output, True)

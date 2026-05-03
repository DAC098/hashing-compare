import matplotlib.pyplot as plt
import scipy.stats as stats
import numpy as np
import math
import os
import sys
import pprint

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

UNIT_SUFFIX = ["", "Ki", "Mi", "Gi", "Ti"]
UNIT_VALUES = [1, Kibi, Mebi, Gibi, Tebi]

def calc_base_2(given):
    length = len(UNIT_SUFFIX)

    for value in range(0, length):
        to_shift = 10 * value
        calc = given >> to_shift

        if calc < Kibi:
            return calc, UNIT_SUFFIX[value], to_shift

    to_shift = 10 * (length - 1)

    return given >> to_shift, UNIT_SUFFIX[length - 1], to_shift

def fmt_base_2(value, prefix):
    (reduced, unit, to_shift) = calc_base_2(value)

    return f"{reduced}{unit}{prefix}"

def calc_base_2_div(given):
    length = len(UNIT_VALUES)

    for value in range(0, length):
        calc = given / UNIT_VALUES[value]

        if calc < Kibi:
            return calc, UNIT_SUFFIX[value], UNIT_VALUES[value]

    last = legnth - 1

    return given / UNIT_VALUES[last], UNIT_SUFFIX[last], UNIT_VALUES[value]

def fmt_base_2_div(given, prefix):
    (reduced, unit, to_div) = calc_base_2_div(given)

    return f"{reduced:.2f}{unit}{prefix}"

def calc_stats(root_dir, name, values, units, label, create_graphs = True):
    n = len(values)

    np_mean = np.mean(values)
    np_std = np.std(values)
    np_sem = np_std / np.sqrt(n)
    np_min = np.min(values)
    np_max = np.max(values)

    msg  = f"{name} | total: {n} | units: {units}\n"
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

    #print(msg)

    with open(os.path.join(root_dir, "stats.txt"), "w") as stats_file:
        stats_file.write(msg)

    if not create_graphs:
        return {
            "mean": np_mean,
            "std": np_std,
            "sem": np_sem,
            "min": np_min,
            "max": np_max,
            "unit": units,
        }

    plt.plot(range(0, len(values)), values)

    plt.title(name)
    plt.xlabel("Runs")
    plt.ylabel(label)
    plt.savefig(os.path.join(root_dir, "jitter.png"))
    plt.clf()

    hist_counts, hist_bins = np.histogram(values, bins=(len(values) // 10))
    plt.stairs(hist_counts, hist_bins)
    plt.title(name)
    plt.xlabel(label)
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

    return {
        "mean": np_mean,
        "std": np_std,
        "sem": np_sem,
        "min": np_min,
        "max": np_max,
        "unit": units,
    }

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
    time_values = []
    speed_values = []

    (env, algo, total_bytes, chunk_size) = parse_test(lines[1])

    for line in lines[3:]:
        split = line.split(",")

        time = float(split[0])

        speed = total_bytes / time

        time_values.append(time)
        speed_values.append(speed)

    return {
        "env": env,
        "algo": algo,
        "total_bytes": total_bytes,
        "chunk_size": chunk_size,
        "time": time_values,
        "speed": speed_values,
    }

def load_perf_outliers(data):
    time_values = remove_outliers(data["time"])
    speed_values = []

    for value in time_values:
        speed_values.append(data["total_bytes"] / value)

    return {
        "env": data["env"],
        "algo": data["algo"],
        "total_bytes": data["total_bytes"],
        "chunk_size": data["chunk_size"],
        "time": time_values,
        "speed": speed_values,
    }

def outlier_prefixs(is_outliers):
    if is_outliers:
        return "_wo", " WO"
    else:
        return "", ""


def calc_perf_file(data, output_dir, is_outliers = False, create_graphs = True):
    (_, unit, to_div) = calc_base_2_div(np.min(data["speed"]))
    adjusted = []

    for value in data["speed"]:
        adjusted.append(value / to_div)

    (file_prefix, name_prefix) = outlier_prefixs(is_outliers)

    data_dir = os.path.join(output_dir, f"{data["env"]}_{data["algo"]}_{data["total_bytes"]}_{data["chunk_size"]}{file_prefix}")

    if not os.path.exists(data_dir):
        os.makedirs(data_dir)

    return calc_stats(
        data_dir,
        f"{data["env"]} {data["algo"]} {fmt_base_2(data["total_bytes"], "B")} {fmt_base_2(data["chunk_size"], "B")}{name_prefix}",
        adjusted,
        unit,
        f"Speed ({unit}B/Sec)",
        create_graphs
    )

def update_comparisons(data, data_stats, is_outliers, env_comparisons, chunk_comparisons, algo_comparisons):
    (key_prefix, name_prefix) = outlier_prefixs(is_outliers)
    env_test_name = f"{data["algo"]}_{data["total_bytes"]}_{data["chunk_size"]}{key_prefix}"
    chunk_test_name = f"{data["env"]}_{data["algo"]}_{data["chunk_size"]}{key_prefix}"
    algo_test_name = f"{data["env"]}_{data["total_bytes"]}_{data["chunk_size"]}{key_prefix}"

    if chunk_test_name not in chunk_comparisons:
        chunk_comparisons[chunk_test_name] = {
            "name": f"{data["env"]} {data["algo"]} {fmt_base_2(data["chunk_size"], "B")} chunk size{name_prefix}",
            "time": {},
            "speed": {},
        }

    chunk_comparisons[chunk_test_name]["time"][data["total_bytes"]] = {
        "results": data["time"],
        "stats": data_stats,
    }
    chunk_comparisons[chunk_test_name]["speed"][data["total_bytes"]] = {
        "results": data["speed"],
        "stats": data_stats,
    }

    if env_test_name not in env_comparisons:
        env_comparisons[env_test_name] = {
            "name": f"{data["algo"]} {fmt_base_2(data["total_bytes"], "B")} bytes {fmt_base_2(data["chunk_size"], "B")} chunk size{name_prefix}",
            "time": {},
            "speed": {},
        }

    env_comparisons[env_test_name]["time"][data["env"]] = {
        "results": data["time"],
        "stats": data_stats,
    }
    env_comparisons[env_test_name]["speed"][data["env"]] = {
        "results": data["speed"],
        "stats": data_stats,
    }

    if algo_test_name not in algo_comparisons:
        algo_comparisons[algo_test_name] = {
            "name": f"{data["env"]} {fmt_base_2(data["total_bytes"], "B")} bytes {fmt_base_2(data["chunk_size"], "B")} chunk size{name_prefix}",
            "time": {},
            "speed": {},
        }

    algo_comparisons[algo_test_name]["time"][data["algo"]] = {
        "results": data["time"],
        "stats": data_stats,
    }
    algo_comparisons[algo_test_name]["speed"][data["algo"]] = {
        "results": data["speed"],
        "stats": data_stats,
    }

def update_dual_comparisons(data, data_stats, is_outliers, speed_chunk_comparisons):
    (key_prefix, name_prefix) = outlier_prefixs(is_outliers)

    speed_chunk_name = f"{data["env"]}_{data["total_bytes"]}{key_prefix}"

    if speed_chunk_name not in speed_chunk_comparisons:
        speed_chunk_comparisons[speed_chunk_name] = {
            "name": f"{data["env"]} {fmt_base_2(data["total_bytes"], "B")} bytes{name_prefix}",
            "x_label": "Chunk Size",
            "speed": {}
        }

    if data["chunk_size"] not in speed_chunk_comparisons[speed_chunk_name]["speed"]:
        speed_chunk_comparisons[speed_chunk_name]["speed"][data["chunk_size"]] = {}

    speed_chunk_comparisons[speed_chunk_name]["speed"][data["chunk_size"]][data["algo"]] = np.mean(data["speed"])

def create_speed_comparison(root_dir, comparisons, create_graphs = True):
    if not os.path.exists(root_dir):
        os.makedirs(root_dir)

    for (key, data) in comparisons.items():
        print(f"creating comparisons {key}")

        text_output = f"{key} average results:\n"
        minimum = None

        for (name, info) in data["speed"].items():
            average = np.mean(info["results"])

            (reduced, unit, _) = calc_base_2_div(average)

            text_output += f"    {name}: {reduced:.2f} {unit}B/Sec\n"

            if minimum is None or average < minimum:
                minimum = average

        with open(os.path.join(root_dir, f"{key}.txt"), "w") as file:
            file.write(text_output)

        if not create_graphs:
            continue

        (_, unit, to_div) = calc_base_2_div(minimum)

        for (name, info) in data["speed"].items():
            speed = []

            for value in info["results"]:
                speed.append(value / to_div)

            plt.plot(range(0, len(speed)), speed, label=name)

        plt.title(data["name"])
        plt.xlabel("Runs")
        plt.ylabel(f"Speed ({unit}B/Sec)")
        plt.legend()
        plt.savefig(os.path.join(root_dir, f"{key}.png"))
        plt.clf()

def create_speed_dual_comparison(root_dir, comparisons, create_graphs = True):
    if not os.path.exists(root_dir):
        os.makedirs(root_dir)

    for (key, data) in comparisons.items():
        x_values = []
        y_values = {}

        for (x_value, x_data) in data["speed"].items():
            x_values.append(x_value)

            for (algo, mean) in x_data.items():
                if algo in y_values:
                    y_values[algo].append(mean)
                else:
                    y_values[algo] = [mean]

        if len(x_values) < 2:
            continue

        for (algo, values) in y_values.items():
            plt.plot(range(len(x_values)), values, label=algo)

        plt.title(data["name"])
        plt.xlabel(data["x_label"])
        plt.xticks(range(len(x_values)), labels=x_values)
        plt.ylabel("Speed (Bytes/Sec)")
        plt.legend()
        plt.savefig(os.path.join(root_dir, f"{key}.png"))
        plt.clf()

def calc_perf_dir(input_dir, output_dir, create_graphs = True):
    final_results = {}
    final_results_wo = {}
    env_comparisons = {}
    chunk_comparisons = {}
    algo_comparisons = {}

    speed_chunk_comparisons = {}

    key_order = ["algo", "chunk_size", "env"]

    for entry in sorted(os.listdir(input_dir)):
        full_path = os.path.join(input_dir, entry)
        ext = os.path.splitext(os.path.basename(full_path))[1]

        if not os.path.isfile(full_path):
            continue

        if ext != ".csv":
            continue

        data = load_perf_file(full_path)
        data_stats = calc_perf_file(data, output_dir, False, False)

        ref = final_results

        for index in range(len(key_order)):
            key = key_order[index]

            if index != len(key_order) - 1:
                if data[key] not in ref:
                    ref[data[key]] = {}
            else:
                if data[key] not in ref:
                    ref[data[key]] = []

            ref = ref[data[key]]

        ref.append(np.mean(data["speed"]))

        update_comparisons(data, data_stats, False, env_comparisons, chunk_comparisons, algo_comparisons)
        update_dual_comparisons(data, data_stats, False, speed_chunk_comparisons)

        wo = load_perf_outliers(data)
        wo_stats = calc_perf_file(wo, output_dir, True, False)

        ref_wo = final_results_wo

        for index in range(len(key_order)):
            key = key_order[index]

            if index != len(key_order) - 1:
                if data[key] not in ref_wo:
                    ref_wo[data[key]] = {}
            else:
                if data[key] not in ref_wo:
                    ref_wo[data[key]] = []

            ref_wo = ref_wo[data[key]]

        ref_wo.append(np.mean(data["speed"]))

        update_comparisons(wo, wo_stats, True, env_comparisons, chunk_comparisons, algo_comparisons)
        update_dual_comparisons(data, data_stats, True, speed_chunk_comparisons)

    comparisons_dir = os.path.join(output_dir, "comparisons")
    env_dir = os.path.join(comparisons_dir, "env")
    chunk_dir = os.path.join(comparisons_dir, "chunk")
    algo_dir = os.path.join(comparisons_dir, "algo")
    speed_chunk_dir = os.path.join(comparisons_dir, "speed_chunk")

    if not os.path.exists(comparisons_dir):
        os.makedirs(comparisons_dir)

    create_speed_comparison(chunk_dir, chunk_comparisons, False)
    create_speed_comparison(env_dir, env_comparisons, False)
    create_speed_comparison(algo_dir, algo_comparisons, False)

    create_speed_dual_comparison(speed_chunk_dir, speed_chunk_comparisons, False)

    return final_results, final_results_wo

def load_power_file(file_path):
    file = open(file_path, "r")

    filename = os.path.splitext(os.path.basename(file_path))[0]
    filename_split = filename.split('_')
    env = filename_split[-3]
    algo = filename_split[-2]
    chunk_size = int(filename_split[-1])

    lines = file.readlines()

    idle_total = []
    idle_cpu = []
    run_total = []
    run_cpu = []
    usage_total = []
    usage_cpu = []

    for line in lines[1:]:
        split = line.split(",")

        total_volts = int(split[1])
        total_amps = int(split[2])
        cpu_volts = int(split[3])
        cpu_amps = int(split[4])

        total_watts = total_volts * total_amps
        cpu_watts = cpu_volts * cpu_amps

        if split[0] == "idle":
            idle_total.append(total_watts)
            idle_cpu.append(cpu_watts)
        elif split[0] == "run":
            run_total.append(total_watts)
            run_cpu.append(cpu_watts)

    total_idle_mean = np.mean(idle_total)
    cpu_idle_mean = np.mean(idle_cpu)

    for value in run_total:
        usage_total.append(value - total_idle_mean)

    for value in run_cpu:
        usage_cpu.append(value - cpu_idle_mean)

    return {
        "env": env,
        "algo": algo,
        "chunk_size": chunk_size,
        "total": {
            "idle": idle_total,
            "run": run_total,
            "usage": usage_total,
        },
        "cpu": {
            "idle": idle_cpu,
            "run": run_cpu,
            "usage": usage_cpu,
        },
    }

def calc_power_stats(output_dir, data, create_graphs = True):
    root_dir = os.path.join(output_dir, f"{data["env"]}_{data["algo"]}_{data["chunk_size"]}")

    if not os.path.exists(root_dir):
        os.makedirs(root_dir)

    name = f"{data["env"]} {data["algo"]} {fmt_base_2(data["chunk_size"], "B")} chunk size"
    msg  = f"{name}\n"

    data_keys = ["total", "cpu"]
    std_dev_values = {}
    p_values = {}

    for p in P_VALUES:
        p_values[p] = {}

    for key in data_keys:
        std_dev_values[key] = {}

        for p in P_VALUES:
            p_values[p][key] = {}

        for (sample_type, samples) in data[key].items():
            n = len(samples)
            np_mean = np.mean(samples)
            np_std = np.std(samples)
            np_sem = np_std / np.sqrt(n)
            np_min = np.min(samples)
            np_max = np.max(samples)

            std_dev_values[key][sample_type] = np_std

            msg += f"    {key}:{sample_type} | MicroWatts | total: {n}\n"
            msg += f"        average: {np_mean:.20f} std_dev: {np_std:.20f} sem: {np_sem:.20f}\n"
            msg += f"        minimum: {np_min:.20f} maximum: {np_max:.20f}\n"

            outliers = 0

            for v in samples:
                z_score = (v - np_mean) / np_std

                if math.fabs(z_score) > STD_DEV_OUTLIER:
                    outliers += 1

            msg += f"        outliers: {outliers} / {n} {(outliers / n * 100):.2f} %\n"

            for p in P_VALUES:
                p_percent = p / 100

                z_value = stats.t.ppf((1 + p_percent) / 2, df=n - 1)
                z_std_dev = z_value * np_std
                moe = z_value * np_std / np.sqrt(n)

                p_values[p][key][sample_type] = moe

                msg += f"        P: {p}%={p_percent} ({1 - p_percent}) {z_value:.20f}\n"
                msg += f"            Z * std_dev: {z_std_dev:.20f}\n"
                msg += f"                    MoE: {moe:.20f}\n"

    with open(os.path.join(root_dir, "stats.txt"), "w") as stats_file:
        stats_file.write(msg)

    if not create_graphs:
        return

    for key in data_keys:
        std_dev_high = []
        std_dev_low = []
        joined = []

        for value in data[key]["idle"]:
            milli = value
            high = milli + std_dev_values[key]["idle"]
            low = milli - std_dev_values[key]["idle"]

            joined.append(milli / 1000)
            std_dev_high.append(high / 1000)
            std_dev_low.append(low / 1000)

        for value in data[key]["run"]:
            milli = value
            high = milli + std_dev_values[key]["run"]
            low = milli - std_dev_values[key]["run"]

            joined.append(milli / 1000)
            std_dev_high.append(high / 1000)
            std_dev_low.append(low / 1000)

        p = plt.plot(range(0, len(joined)), joined, label=key)[0]
        plt.plot(range(0, len(joined)), std_dev_high, color=p.get_color(), linestyle="dashed")
        plt.plot(range(0, len(joined)), std_dev_low, color=p.get_color(), linestyle="dashed")

    plt.title(f"{name} Idle/Run Samples")
    plt.xlabel("Samples")
    plt.ylabel("MilliWatts")
    plt.legend()
    plt.savefig(os.path.join(root_dir, "std_dev_jitter.png"))
    plt.clf()

    for key in data_keys:
        std_dev_high = []
        std_dev_low = []
        adjusted = []

        for value in data[key]["usage"]:
            milli = value
            high = milli + std_dev_values[key]["usage"]
            low = milli - std_dev_values[key]["usage"]

            adjusted.append(milli / 1000)
            std_dev_high.append(high / 1000)
            std_dev_low.append(low / 1000)

        p = plt.plot(range(0, len(adjusted)), adjusted, label=key)[0]
        plt.plot(range(0, len(adjusted)), std_dev_high, color=p.get_color(), linestyle="dashed")
        plt.plot(range(0, len(adjusted)), std_dev_low, color=p.get_color(), linestyle="dashed")

    plt.title(f"{name} Power Usage")
    plt.xlabel("Samples")
    plt.ylabel("MilliWatts")
    plt.legend()
    plt.savefig(os.path.join(root_dir, "std_dev_usage.png"))
    plt.clf()

    for (p, moe) in p_values.items():
        for key in data_keys:
            highs = []
            lows = []
            joined = []

            for value in data[key]["idle"]:
                milli = value
                high = milli + moe[key]["idle"]
                low = milli - moe[key]["idle"]

                joined.append(milli / 1000)
                highs.append(high / 1000)
                lows.append(low / 1000)

            for value in data[key]["run"]:
                milli = value
                high = milli + moe[key]["run"]
                low = milli - moe[key]["run"]

                joined.append(milli / 1000)
                highs.append(high / 1000)
                lows.append(low / 1000)

            line = plt.plot(range(0, len(joined)), joined, label=key)[0]
            plt.plot(range(0, len(highs)), highs, color=line.get_color(), linestyle="dashed")
            plt.plot(range(0, len(lows)), lows, color=line.get_color(), linestyle="dashed")

        plt.title(f"{name} p{p} Idle/Run Samples")
        plt.xlabel("Samples")
        plt.ylabel("MilliWatts")
        plt.legend()
        plt.savefig(os.path.join(root_dir, f"p{p}_jitter.png"))
        plt.clf()

    for (p, moe) in p_values.items():
        for key in data_keys:
            highs = []
            lows = []
            joined = []

            for value in data[key]["usage"]:
                milli = value
                high = milli + moe[key]["usage"]
                low = milli - moe[key]["usage"]

                joined.append(milli / 1000)
                highs.append(high / 1000)
                lows.append(low / 1000)

            line = plt.plot(range(0, len(joined)), joined, label=key)[0]
            plt.plot(range(0, len(joined)), highs, color=line.get_color(), linestyle="dashed")
            plt.plot(range(0, len(joined)), lows, color=line.get_color(), linestyle="dashed")

        plt.title(f"{name} p{p} Power Usage")
        plt.xlabel("Samples")
        plt.ylabel("MilliWatts")
        plt.legend()
        plt.savefig(os.path.join(root_dir, f"p{p}_usage.png"))
        plt.clf()


def create_power_comparison(root_dir, comparisons, create_graphs = True):
    if not os.path.exists(root_dir):
        os.makedirs(root_dir)

    for (key, data) in comparisons.items():
        text_output = f"{key} average results:\n"

        for (name, info) in data["results"].items():
            np_mean = np.mean(info["cpu"]["usage"])
            np_std = np.std(info["cpu"]["usage"])

            text_output += f"    {name}: {np_mean / 1000:.3f}+-{np_std / 1000:.3f} MilliWatts\n"

        with open(os.path.join(root_dir, f"{key}.txt"), "w") as file:
            file.write(text_output)

        if not create_graphs:
            continue

        for (name, info) in data["results"].items():
            speed = []

            for value in info["cpu"]["usage"]:
                speed.append(value / 1000)

            plt.plot(range(0, len(speed)), speed, label=name)

        plt.title(data["name"])
        plt.xlabel("Runs")
        plt.ylabel("MilliWatts")
        plt.legend()
        plt.savefig(os.path.join(root_dir, f"{key}.png"))
        plt.clf()

def calc_power_dir(input_dir, output_dir, create_graphs = True):
    key_order = ["algo", "chunk_size", "env"]
    final_results = {}
    indexs = {
        "algo_chunk": {},
        "env_chunk": {}
    }

    for entry in sorted(os.listdir(input_dir)):
        full_path = os.path.join(input_dir, entry)
        ext = os.path.splitext(os.path.basename(full_path))[1]

        if not os.path.isfile(full_path):
            continue

        if ext != ".csv":
            continue

        data = load_power_file(full_path)

        ref = final_results

        for index in range(len(key_order)):
            key = key_order[index]

            if index != len(key_order) - 1:
                if data[key] not in ref:
                    ref[data[key]] = {}
            else:
                if data[key] not in ref:
                    ref[data[key]] = []

            ref = ref[data[key]]

        ref.append(np.mean(data["cpu"]["usage"]))

        calc_power_stats(output_dir, data, False)

        algo_chunk_name = f"{data["algo"]}_{data["chunk_size"]}"

        if algo_chunk_name not in indexs["algo_chunk"]:
            indexs["algo_chunk"][algo_chunk_name] = {
                "name": f"{data["algo"]} {fmt_base_2(data["chunk_size"], "B")} Chunk Size",
                "results": {}
            }

        indexs["algo_chunk"][algo_chunk_name]["results"][data["env"]] = data

        env_chunk_name = f"{data["env"]}_{data["chunk_size"]}"

        if env_chunk_name not in indexs["env_chunk"]:
            indexs["env_chunk"][env_chunk_name] = {
                "name": f"{data["env"]} {fmt_base_2(data["chunk_size"], "B")} Chunk Size",
                "results": {}
            }

        indexs["env_chunk"][env_chunk_name]["results"][data["algo"]] = data

    comp_dir = os.path.join(output_dir, "comparisons")
    algo_chunk_dir = os.path.join(comp_dir, "algo_chunk")
    env_chunk_dir = os.path.join(comp_dir, "env_chunk")

    create_power_comparison(algo_chunk_dir, indexs["algo_chunk"], False)
    create_power_comparison(env_chunk_dir, indexs["env_chunk"], False)

    return final_results

if __name__ == "__main__":
    root_dir = os.path.join(os.path.dirname(os.path.realpath(__file__)))
    perf_input = os.path.join(root_dir, "output/perf")
    perf_output = os.path.join(root_dir, "results/perf")
    power_input = os.path.join(root_dir, "output/power")
    power_output = os.path.join(root_dir, "results/power")

    plt.figure(figsize=(16,9), dpi=120)

    if not os.path.exists(perf_output):
        os.makedirs(perf_output)

    if not os.path.exists(power_output):
        os.makedirs(power_output)

    (speed_results, speed_results_wo) = calc_perf_dir(perf_input, perf_output, True)
    usage_results = calc_power_dir(power_input, power_output, True)

    final_results_csv = "Algorithm,Environment,\"Chunk Size\",\"Avg Perf (per sec)\",\"Avg Perf WO (per sec)\",\"Power (mW)\",\"Avg Power Consumption (mJ/B)\"\n";

    known_chunks = []
    speed_final = {}
    usage_final = {}
    joule_per_byte = {}

    for (algo, chunks) in speed_results.items():
        algo_alt = algo.replace('_', '-')

        if len(known_chunks) == 0:
            for value in chunks.keys():
                known_chunks.append(value)

        for (chunk, envs) in chunks.items():
            for (env, results) in envs.items():
                avg_speed = np.mean(results)
                avg_speed_wo = np.mean(speed_results_wo[algo][chunk][env])
                avg_watts = np.mean(usage_results[algo_alt][chunk][env]) / 1000

                final_results_csv += f"\"{algo_alt}\",\"{env}\",\"{fmt_base_2(chunk, "B")}\",\"{fmt_base_2_div(avg_speed, "B")}\",\"{fmt_base_2_div(avg_speed_wo, "B")}\",\"{avg_watts:.3f}\",\"{avg_watts / avg_speed:.10f}\"\n";

                if env not in speed_final:
                    speed_final[env] = {
                        algo: [],
                    }
                    usage_final[env] = {
                        algo: [],
                    }
                    joule_per_byte[env] = {
                        algo: [],
                    }

                if algo not in speed_final[env]:
                    speed_final[env][algo] = []
                    usage_final[env][algo] = []
                    joule_per_byte[env][algo] = []

                speed_final[env][algo].append(avg_speed)
                usage_final[env][algo].append(avg_watts)
                joule_per_byte[env][algo].append(avg_watts / avg_speed)

    with open(os.path.join(root_dir, "results", "final.csv"), "w") as file:
        file.write(final_results_csv)

    for (env, algos) in speed_final.items():
        key = f"{env}_speed"

        for (algo, values) in algos.items():
            plt.plot(range(0, len(values)), values, label=algo)

        plt.title(f"{env} Performance")
        plt.xlabel("Chunk Size")
        plt.xticks(range(len(known_chunks)), labels=known_chunks)
        plt.ylabel("Speed (Bytes/Sec)")
        plt.legend()
        plt.savefig(os.path.join(root_dir,"results", f"{key}.png"))
        plt.clf()

    for (env, algos) in usage_final.items():
        key = f"{env}_usage"

        for (algo, values) in algos.items():
            plt.plot(range(0, len(values)), values, label=algo)

        plt.title(f"{env} Power Usage")
        plt.xlabel("Chunk Size")
        plt.xticks(range(len(known_chunks)), labels=known_chunks)
        plt.ylabel("CPU (mW)")
        plt.legend()
        plt.savefig(os.path.join(root_dir,"results", f"{key}.png"))
        plt.clf()

    for (env, algos) in joule_per_byte.items():
        key = f"{env}_joule_per_byte"

        for (algo, values) in algos.items():
            plt.plot(range(0, len(values)), values, label=algo)

        plt.title(f"{env} Joules Per Bytes")
        plt.xlabel("Chunk Sizes")
        plt.xticks(range(len(known_chunks)), labels=known_chunks)
        plt.ylabel("Average Power Consumption (mJ/B)")
        plt.legend()
        plt.savefig(os.path.join(root_dir,"results", f"{key}.png"))
        plt.clf()

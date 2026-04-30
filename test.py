import os
import subprocess
import sys

def get_size(path):
    try:
        size = os.path.getsize(path)

        return size
    except OSError:
        return 0

def run_native(algo, size, input_path, iterations, warmup):
    print(f"### native {algo} {size} \"{input_path}\" {iterations} {warmup}")

    subprocess.run(
        [
            "./target/release/native",
            "-w", str(warmup),
            "-i", str(iterations),
            "--output", "./output",
            "--chunk-size", str(size),
            algo,
            "file",
            "--path", input_path
        ]
    )

def run_wasm(algo, size, input_path, iterations, warmup):
    print(f"~~~ wasm   {algo} {size} \"{input_path}\" {iterations} {warmup}")

    subprocess.run(
        [
            "node",
            "./modules/node/main.js",
            "-w", str(warmup),
            "-i", str(iterations),
            "--chunk-size", str(size),
            "--output", "./output/",
            algo,
            input_path,
        ]
    )

if __name__ == "__main__":
    env = "native"

    if len(sys.argv) >= 2:
        print(sys.argv)

        env = sys.argv[1];

    if env != "native" and env != "wasm" and env != "both":
        print("unknown env")

        sys.exit(1)

    if not os.path.exists("./output"):
        os.makedirs("./output")

    warmup = 500
    iterations = 10000
    hash_algo = [
        "md5",
        "sha1",
        "sha2-256",
        "sha2-384",
        "sha2-512",
        "sha3-256",
        "sha3-384",
        "sha3-512",
        "blake3"
    ]
    chunk_size = [64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384]

    for algo in hash_algo:
        for data in range(1, 15):
            input_path = f"./input/test_{data:02}.data"
            input_size = get_size(input_path)

            if input_size == 0:
                continue

            for size in chunk_size:
                if input_size < size:
                    continue

                if input_size > (size * 32):
                    continue

                if env == "native":
                    run_native(algo, size, input_path, iterations, warmup)
                elif env == "wasm":
                    run_wasm(algo, size, input_path, iterations, warmup)
                else:
                    run_native(algo, size, input_path, iterations, warmup)
                    run_wasm(algo, size, input_path, iterations, warmup)

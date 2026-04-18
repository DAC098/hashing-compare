const fs = require("node:fs");
const path = require("node:path");
const process = require("node:process");

const { Command, Option } = require("commander");
const lib = require("wasm");

const NANO = 1000000000;
const NANO_BIG = 1000000000n;

function main() {
    const program = new Command();

    program.option("-w, --warmup <number>", "number of warmup iterations to run", parseBase10, 50)
        .option("-i, --iterations <number>", "number of test iterations to run", parseBase10, 100)
        .option("--chunk-size <number>", "the chunk size to use when calculating a hash", parseBase10, 512)
        .argument("<algo>", "the hash algorithm to test")
        .argument("<input>", "the input file to use as data")
        .argument("<output>", "directory or file to output test results to");

    program.parse();

    let opts = program.opts();

    console.log(opts);

    let chunk_size = opts.chunkSize;
    let iterations = opts.iterations;
    let warmup = opts.warmup;
    let input = program.args[1];
    let output = program.args[2];

    let name;
    let cb;

    switch (program.args[0]) {
        case "md5":
            name = "md5";
            cb = run_md5;
            break;
        case "sha1":
            name = "sha1";
            cb = run_sha1;
            break;
        case "sha2-256":
            name = "sha2_256";
            cb = run_sha2_256;
            break;
        case "sha2-384":
            name = "sha2_384";
            cb = run_sha2_384;
            break;
        case "sha2-512":
            name = "sha2_512";
            cb = run_sha2_512;
            break;
        case "sha3-256":
            name = "sha3_256";
            cb = run_sha3_256;
            break;
        case "sha3-384":
            name = "sha3_384";
            cb = run_sha3_384;
            break;
        case "sha3-512":
            name = "sha3_512";
            cb = run_sha3_512;
            break;
        case "blake3":
            name = "blake3";
            cb = run_blake3;
            break;
        default:
            console.log("unknown hash");
            return;
    }

    let settings = {
        name,
        chunk_size,
        iterations,
        warmup,
        input,
        output,
    };

    run_test(settings, cb);
}

function parseBase10(v) {
    return parseInt(v, 10);
}

function run_test({name, chunk_size, iterations, warmup, input, output}, cb) {
    let [data, bytes] = get_input(input, chunk_size);
    let results = [];
    let total = iterations + warmup;

    let csv_output = `env,hash,bytes,chunk_size,iterations,warmup\nwasm,${name},${bytes},${chunk_size},${iterations},${warmup}\ntimes\n`;
    console.log(`starting test: ${name} ${chunk_size} ${iterations} ${warmup}`);

    let last_notify = process.hrtime.bigint();
    let notify_dur = NANO_BIG * 10n;

    for (let index = 0; index < total; index += 1) {
        let start = process.hrtime.bigint();

        cb(data);

        let end = process.hrtime.bigint();
        let duration = end - start;

        if (index >= warmup) {
            let value = parseFloat(duration.toString()) / NANO;

            csv_output += value + "\n";

            results.push(value);
        }

        if (end - last_notify > notify_dur) {
            let percent_complete = (index / total) * 100;

            console.log(`${name} ${index} / ${total} ${percent_complete.toFixed(1)}`);

            last_notify = end;
        }
    }

    log_results(results, bytes);

    fs.writeFileSync(get_output(name, output), csv_output);
}

function run_md5(chunks) {
    let hasher = new lib.Md5();

    for (let chunk of chunks) {
        hasher.update(chunk);
    }

    return hasher.finalize();
}

function run_sha1(chunks) {
    let hasher = new lib.Sha1();

    for (let chunk of chunks) {
        hasher.update(chunk);
    }

    return hasher.finalize();
}

function run_sha2_256(chunks) {
    let hasher = new lib.Sha2_256();

    for (let chunk of chunks) {
        hasher.update(chunk);
    }

    return hasher.finalize();
}

function run_sha2_384(chunks) {
    let hasher = new lib.Sha2_384();

    for (let chunk of chunks) {
        hasher.update(chunk);
    }

    return hasher.finalize();
}

function run_sha2_512(chunks) {
    let hasher = new lib.Sha2_512();

    for (let chunk of chunks) {
        hasher.update(chunk);
    }

    return hasher.finalize();
}

function run_sha3_256(chunks) {
    let hasher = new lib.Sha3_256();

    for (let chunk of chunks) {
        hasher.update(chunk);
    }

    return hasher.finalize();
}

function run_sha3_384(chunks) {
    let hasher = new lib.Sha3_384();

    for (let chunk of chunks) {
        hasher.update(chunk);
    }

    return hasher.finalize();
}

function run_sha3_512(chunks) {
    let hasher = new lib.Sha3_512();

    for (let chunk of chunks) {
        hasher.update(chunk);
    }

    return hasher.finalize();
}

function run_blake3(chunks) {
    let hasher = new lib.Blake3();

    for (let chunk of chunks) {
        hasher.update(chunk);
    }

    return hasher.finalize();
}

function get_input(input, chunk_size) {
    let buffer = fs.readFileSync(input);
    let rtn = [];

    for (let index = 0; index < buffer.length; index += chunk_size) {
        rtn.push(buffer.subarray(index, index + chunk_size));
    }

    console.log("loaded input:", buffer.length);

    return [rtn, buffer.length];
}

function get_output_path(output) {
    let cwd = process.cwd();

    if (path.isAbsolute(output)) {
        return output;
    } else {
        return path.join(cwd, output);
    }
}

function get_output(name, output) {
    let output_path = get_output_path(output);

    let stats = fs.statSync(output_path, {throwIfNoEntry: false});

    if (stats != null) {
        if (stats.isDirectory()) {
            return path.join(output_path, `wasm_${name}_${get_time()}.csv`);
        }
    }

    // does not exist so create it and treat it like a file or it does exist in
    // which case just overwrite it
    return output_path;
}

function get_time() {
    let now = new Date();

    return Math.floor(now.getTime() / 1000);
}

const KIBI = 1024;

const UNIT_VALUES = [
    [1, ""],
    [KIBI, "Ki"],
    [KIBI ** 2, "Mi"],
    [KIBI ** 3, "Gi"],
];

function calc_unit(bytes, suffix) {
    let index = 0;
    let len = UNIT_VALUES.length;

    while (index < len) {
        let calc = bytes / UNIT_VALUES[index][0];

        if (calc < KIBI) {
            return "" + calc.toFixed(2) + UNIT_VALUES[index][1] + suffix;
        } else {
            index += 1;
        }
    }

    return "" + (bytes / UNIT_VALUES[len - 1][0]).toFixed(2) + UNIT_VALUES[len - 1][1] + suffix;
}

function log_results(results, total_bytes) {
    let total_time = 0.0;
    let min = Infinity;
    let max = -Infinity;

    for (let value of results) {
        total_time += value;

        if (value < min) {
            min = value;
        }

        if (value > max) {
            max = value;
        }
    }

    let average = total_time / results.length;
    let variance = 0.0;

    for (let value of results) {
        variance += Math.pow(value - average, 2);
    }

    variance /= results.length;

    let std_dev = Math.sqrt(variance);
    let sem = std_dev / results.length;
    let outliers = 0;

    for (let value of results) {
        let score = Math.abs((value - average) / std_dev);

        if (score > 3.0) {
            outliers += 1;
        }
    }

    let outlier_percent = (outliers / results.length) * 100.0;
    let hashing_speed = total_bytes / average;

    console.log(`results: ~${average}+-${std_dev}`);
    console.log(`    sem: ${sem} min: ${min} max: ${max}`);
    console.log(`    outliers: ${outliers} / ${results.length} ${outlier_percent}%`);
    console.log(`    speed: ${calc_unit(hashing_speed, "B")}/s`);
}

main();

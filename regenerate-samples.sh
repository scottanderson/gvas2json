#!/usr/bin/env bash
for SAMPLE in sample{1,2}; do
    for FORMAT in json toml yaml; do
        cargo run --bin gvas2${FORMAT} -- resources/test/${SAMPLE}.sav -o resources/test/${SAMPLE}.${FORMAT}
    done
done

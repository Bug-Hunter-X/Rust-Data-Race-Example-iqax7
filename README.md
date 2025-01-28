This repository demonstrates a classic data race bug in Rust. Two mutable references point to the same variable, leading to undefined behavior due to the lack of guaranteed order in assignments. The solution demonstrates how to avoid data races using techniques like mutexes or other synchronization primitives.
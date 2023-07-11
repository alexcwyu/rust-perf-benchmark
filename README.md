## Introduction

compare the performance of various decimal library in rust

the test is based on my local laptop

| library                               | add       | sub       | mul       | div       | all ops   | list of ops |
|:--------------------------------------|:----------|:----------|:----------|:----------|:----------|:------------|
| f64 (64bit, native)                   | 627.29 ps | 628.17 ps | 627.28 ps | 1.1150 ns | 1.1555 ns | 45.528 µs   |
| fixed (64bit, I36F28)                 | 558.22 ps | 559.76 ps | 557.36 ps | 12.176 ns | 12.978 ns | 129.19 µs   |
| fixed_decimal (64bit, self implement) | 1.6757 ns | 1.6723 ns | 1.6717 ns | 11.223 ns | 16.770 ns | 180.04 µs   |
| fixed (128bit, I64F64)                | 1.1549 ns | 1.1147 ns | 2.3724 ns | 76.839 ns | 80.534 ns | 915.31 µs   |
| fpdec (128bit)                        | 1.6839 ns | 1.6722 ns | 45.222 ns | 49.575 ns | 99.551 ns | 1.1969 ms   |
| rust_decimal (96bit)                  | 13.985 ns | 14.027 ns | 48.775 ns | 83.785 ns | 205.72 ns | 3.8191 ms   |
| bigdecimal                            | 35.426 ns | 28.751 ns | 625.35 ns | 28.112 µs | 29.619 µs | 307.07 ms   |

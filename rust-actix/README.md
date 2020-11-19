In `cargo run --release`:

```
(base) ➜  tracks-http git:(master) ✗ make performance
autocannon localhost:8080/tracks/test_key_path
Running 10s test @ http://localhost:8080/tracks/test_key_path
10 connections

┌─────────┬──────┬───────┬───────┬───────┬──────────┬─────────┬───────┐
│ Stat    │ 2.5% │ 50%   │ 97.5% │ 99%   │ Avg      │ Stdev   │ Max   │
├─────────┼──────┼───────┼───────┼───────┼──────────┼─────────┼───────┤
│ Latency │ 8 ms │ 10 ms │ 14 ms │ 15 ms │ 10.33 ms │ 2.71 ms │ 68 ms │
└─────────┴──────┴───────┴───────┴───────┴──────────┴─────────┴───────┘
┌───────────┬────────┬────────┬────────┬────────┬────────┬─────────┬────────┐
│ Stat      │ 1%     │ 2.5%   │ 50%    │ 97.5%  │ Avg    │ Stdev   │ Min    │
├───────────┼────────┼────────┼────────┼────────┼────────┼─────────┼────────┤
│ Req/Sec   │ 815    │ 815    │ 934    │ 963    │ 922.73 │ 38.42   │ 815    │
├───────────┼────────┼────────┼────────┼────────┼────────┼─────────┼────────┤
│ Bytes/Sec │ 134 kB │ 134 kB │ 153 kB │ 158 kB │ 151 kB │ 6.29 kB │ 134 kB │
└───────────┴────────┴────────┴────────┴────────┴────────┴─────────┴────────┘

Req/Bytes counts sampled once per second.

10k requests in 11.01s, 1.66 MB read
(base) ➜  tracks-http git:(master) ✗ make performance
autocannon localhost:8080/tracks/test_key_path
Running 10s test @ http://localhost:8080/tracks/test_key_path
10 connections

┌─────────┬──────┬───────┬───────┬───────┬──────────┬─────────┬───────┐
│ Stat    │ 2.5% │ 50%   │ 97.5% │ 99%   │ Avg      │ Stdev   │ Max   │
├─────────┼──────┼───────┼───────┼───────┼──────────┼─────────┼───────┤
│ Latency │ 8 ms │ 10 ms │ 23 ms │ 23 ms │ 11.92 ms │ 4.26 ms │ 40 ms │
└─────────┴──────┴───────┴───────┴───────┴──────────┴─────────┴───────┘
┌───────────┬────────┬────────┬────────┬────────┬────────┬─────────┬────────┐
│ Stat      │ 1%     │ 2.5%   │ 50%    │ 97.5%  │ Avg    │ Stdev   │ Min    │
├───────────┼────────┼────────┼────────┼────────┼────────┼─────────┼────────┤
│ Req/Sec   │ 672    │ 672    │ 795    │ 965    │ 805.1  │ 101.56  │ 672    │
├───────────┼────────┼────────┼────────┼────────┼────────┼─────────┼────────┤
│ Bytes/Sec │ 110 kB │ 110 kB │ 130 kB │ 158 kB │ 132 kB │ 16.7 kB │ 110 kB │
└───────────┴────────┴────────┴────────┴────────┴────────┴─────────┴────────┘

Req/Bytes counts sampled once per second.

8k requests in 10.02s, 1.32 MB read
(base) ➜  tracks-http git:(master) ✗ make performance
autocannon localhost:8080/tracks/test_key_path
Running 10s test @ http://localhost:8080/tracks/test_key_path
10 connections

┌─────────┬──────┬───────┬───────┬───────┬─────────┬─────────┬───────┐
│ Stat    │ 2.5% │ 50%   │ 97.5% │ 99%   │ Avg     │ Stdev   │ Max   │
├─────────┼──────┼───────┼───────┼───────┼─────────┼─────────┼───────┤
│ Latency │ 8 ms │ 10 ms │ 15 ms │ 17 ms │ 10.3 ms │ 1.93 ms │ 28 ms │
└─────────┴──────┴───────┴───────┴───────┴─────────┴─────────┴───────┘
┌───────────┬────────┬────────┬────────┬────────┬────────┬─────────┬────────┐
│ Stat      │ 1%     │ 2.5%   │ 50%    │ 97.5%  │ Avg    │ Stdev   │ Min    │
├───────────┼────────┼────────┼────────┼────────┼────────┼─────────┼────────┤
│ Req/Sec   │ 824    │ 824    │ 916    │ 1003   │ 925.55 │ 48.56   │ 824    │
├───────────┼────────┼────────┼────────┼────────┼────────┼─────────┼────────┤
│ Bytes/Sec │ 135 kB │ 135 kB │ 150 kB │ 165 kB │ 152 kB │ 7.98 kB │ 135 kB │
└───────────┴────────┴────────┴────────┴────────┴────────┴─────────┴────────┘

Req/Bytes counts sampled once per second.

10k requests in 11.01s, 1.67 MB read
```

```
(base) ➜  tracks-gin make wrk
wrk -c 128 -d 10 -t 4 --latency http://localhost:8080/tracks/test_key_path
Running 10s test @ http://localhost:8080/tracks/test_key_path
  4 threads and 128 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   166.97ms  180.12ms   1.69s    88.45%
    Req/Sec   236.96     55.86   380.00     68.75%
  Latency Distribution
     50%  141.44ms
     75%  250.27ms
     90%  371.30ms
     99%  849.46ms
  9457 requests in 10.02s, 1.48MB read
Requests/sec:    943.54
Transfer/sec:    151.11KB
(base) ➜  tracks-gin make wrk
wrk -c 128 -d 10 -t 4 --latency http://localhost:8080/tracks/test_key_path
Running 10s test @ http://localhost:8080/tracks/test_key_path
  4 threads and 128 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   128.73ms  146.12ms   1.12s    84.44%
    Req/Sec   238.40     65.82   434.00     69.13%
  Latency Distribution
     50%   62.28ms
     75%  209.54ms
     90%  332.08ms
     99%  590.58ms
  7099 requests in 10.07s, 1.11MB read
  Socket errors: connect 0, read 0, write 0, timeout 127
Requests/sec:    704.88
Transfer/sec:    112.89KB
(base) ➜  tracks-gin make wrk
wrk -c 128 -d 10 -t 4 --latency http://localhost:8080/tracks/test_key_path
Running 10s test @ http://localhost:8080/tracks/test_key_path
  4 threads and 128 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   186.52ms  216.40ms   1.74s    88.35%
    Req/Sec   242.92     62.28   450.00     73.04%
  Latency Distribution
     50%  142.44ms
     75%  263.26ms
     90%  439.52ms
     99%    1.01s
  9285 requests in 10.03s, 1.45MB read
Requests/sec:    925.30
Transfer/sec:    148.19KB
```
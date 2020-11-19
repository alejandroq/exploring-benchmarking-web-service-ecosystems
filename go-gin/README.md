While running `go run main.go`:

```
(base) ➜  tracks-gin make performance
autocannon localhost:8080/tracks/test_key_path
Running 10s test @ http://localhost:8080/tracks/test_key_path
10 connections

┌─────────┬──────┬──────┬───────┬───────┬─────────┬─────────┬───────┐
│ Stat    │ 2.5% │ 50%  │ 97.5% │ 99%   │ Avg     │ Stdev   │ Max   │
├─────────┼──────┼──────┼───────┼───────┼─────────┼─────────┼───────┤
│ Latency │ 4 ms │ 9 ms │ 23 ms │ 28 ms │ 10.4 ms │ 5.02 ms │ 60 ms │
└─────────┴──────┴──────┴───────┴───────┴─────────┴─────────┴───────┘
┌───────────┬────────┬────────┬────────┬────────┬────────┬────────┬────────┐
│ Stat      │ 1%     │ 2.5%   │ 50%    │ 97.5%  │ Avg    │ Stdev  │ Min    │
├───────────┼────────┼────────┼────────┼────────┼────────┼────────┼────────┤
│ Req/Sec   │ 706    │ 706    │ 965    │ 1034   │ 916.82 │ 114.11 │ 706    │
├───────────┼────────┼────────┼────────┼────────┼────────┼────────┼────────┤
│ Bytes/Sec │ 130 kB │ 130 kB │ 178 kB │ 190 kB │ 169 kB │ 21 kB  │ 130 kB │
└───────────┴────────┴────────┴────────┴────────┴────────┴────────┴────────┘

Req/Bytes counts sampled once per second.

10k requests in 11.01s, 1.86 MB read
(base) ➜  tracks-gin make performance
autocannon localhost:8080/tracks/test_key_path
Running 10s test @ http://localhost:8080/tracks/test_key_path
10 connections

┌─────────┬──────┬──────┬───────┬───────┬─────────┬─────────┬───────┐
│ Stat    │ 2.5% │ 50%  │ 97.5% │ 99%   │ Avg     │ Stdev   │ Max   │
├─────────┼──────┼──────┼───────┼───────┼─────────┼─────────┼───────┤
│ Latency │ 4 ms │ 8 ms │ 20 ms │ 24 ms │ 9.34 ms │ 4.25 ms │ 41 ms │
└─────────┴──────┴──────┴───────┴───────┴─────────┴─────────┴───────┘
┌───────────┬────────┬────────┬────────┬────────┬─────────┬─────────┬────────┐
│ Stat      │ 1%     │ 2.5%   │ 50%    │ 97.5%  │ Avg     │ Stdev   │ Min    │
├───────────┼────────┼────────┼────────┼────────┼─────────┼─────────┼────────┤
│ Req/Sec   │ 990    │ 990    │ 1007   │ 1080   │ 1015.46 │ 25.91   │ 990    │
├───────────┼────────┼────────┼────────┼────────┼─────────┼─────────┼────────┤
│ Bytes/Sec │ 182 kB │ 182 kB │ 185 kB │ 199 kB │ 187 kB  │ 4.76 kB │ 182 kB │
└───────────┴────────┴────────┴────────┴────────┴─────────┴─────────┴────────┘

Req/Bytes counts sampled once per second.

11k requests in 11.01s, 2.06 MB read
(base) ➜  tracks-gin
```

```
(base) ➜  tracks-gin make wrk
wrk -c 128 -d 10 -t 4 --latency http://localhost:8080/tracks/test_key_path
Running 10s test @ http://localhost:8080/tracks/test_key_path
  4 threads and 128 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   158.02ms  256.71ms   1.98s    90.23%
    Req/Sec   273.56     71.55   480.00     70.25%
  Latency Distribution
     50%   60.53ms
     75%  173.54ms
     90%  406.52ms
     99%    1.39s
  10915 requests in 10.02s, 1.92MB read
  Socket errors: connect 0, read 0, write 0, timeout 86
Requests/sec:   1089.07
Transfer/sec:    195.69KB
(base) ➜  tracks-gin make wrk
wrk -c 128 -d 10 -t 4 --latency http://localhost:8080/tracks/test_key_path
Running 10s test @ http://localhost:8080/tracks/test_key_path
  4 threads and 128 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   197.80ms  280.99ms   1.96s    88.59%
    Req/Sec   269.11     63.11   520.00     69.00%
  Latency Distribution
     50%   85.27ms
     75%  235.57ms
     90%  527.95ms
     99%    1.40s
  10733 requests in 10.02s, 1.88MB read
  Socket errors: connect 0, read 0, write 0, timeout 8
Requests/sec:   1071.52
Transfer/sec:    192.54KB
(base) ➜  tracks-gin make wrk
wrk -c 128 -d 10 -t 4 --latency http://localhost:8080/tracks/test_key_path
Running 10s test @ http://localhost:8080/tracks/test_key_path
  4 threads and 128 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   181.20ms  265.02ms   2.00s    89.18%
    Req/Sec   259.57     61.30   434.00     69.75%
  Latency Distribution
     50%   71.85ms
     75%  211.73ms
     90%  469.67ms
     99%    1.33s
  10362 requests in 10.03s, 1.82MB read
  Socket errors: connect 0, read 0, write 0, timeout 37
Requests/sec:   1033.15
Transfer/sec:    185.64KB
```
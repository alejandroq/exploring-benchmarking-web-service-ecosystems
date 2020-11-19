.PHOMY: run
run:
	go run main.go

.PHONY: performance
performance:
	autocannon localhost:8080/tracks/test_key_path

.PHOMY: wrk
wrk:
	wrk -c 128 -d 10 -t 4 --latency http://localhost:8080/tracks/test_key_path

.PHONY: mycli
mycli:
	mycli -h localhost -p 3306 -u user -p pass -D db

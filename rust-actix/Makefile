.PHONY: integration
integration:
	cargo test -- --ignored

.PHONY: performance
performance:
	autocannon localhost:8080/tracks/test_key_path

.PHONY: mycli
mycli:
	mycli -h localhost -p 3306 -u user -p pass -D db
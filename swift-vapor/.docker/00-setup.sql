CREATE TABLE IF NOT EXISTS registrations (
    id BINARY(16) NOT NULL PRIMARY KEY,
    service_id VARCHAR(255) NOT NULL,
    node_id VARCHAR(255) NOT NULL
);

CREATE TABLE IF NOT EXISTS tracks (
    id BINARY(16) NOT NULL PRIMARY KEY,
    key_path VARCHAR(255) NOT NULL,
    service_id VARCHAR(255) NOT NULL,
    output_value VARCHAR(255) NOT NULL
);

CREATE TABLE IF NOT EXISTS traces (
    id BINARY(16) NOT NULL PRIMARY KEY,
    service_id varchar(255) NOT NULL,
    client_id varchar(255),
    node_id varchar(255),
    request_id varchar(255),
    key_path varchar(255) NOT NULL,
    output_value varchar(255) NOT NULL,
    created_at DATETIME NOT NULL
);

INSERT INTO
    tracks
VALUES
    (
        1,
        "test_key_path",
        "test_service_name",
        "test_output_value"
    );

INSERT INTO
    traces
VALUES
    (
        1,
        "test_service_name",
        "test_client_id",
        "test_node_id",
        "test_request_id",
        "test_key_path",
        "test_output_value",
        NOW()
    );

INSERT INTO
    registrations
VALUES
    (1, "test_service_name", "test_node_id");

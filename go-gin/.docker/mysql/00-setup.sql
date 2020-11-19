CREATE TABLE IF NOT EXISTS registrations (
    service_id VARCHAR(255) NOT NULL,
    node_id VARCHAR(255) NOT NULL,
    CONSTRAINT pk_access_id PRIMARY KEY (service_id, node_id)
);

CREATE TABLE IF NOT EXISTS tracks (
    key_path VARCHAR(255) NOT NULL,
    service_id VARCHAR(255) NOT NULL,
    output_value VARCHAR(255) NOT NULL,
    CONSTRAINT pk_access_id PRIMARY KEY (service_id, key_path) -- consider how rules, etc will work
);

CREATE TABLE IF NOT EXISTS traces (
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
        "test_key_path",
        "test_service_name",
        "test_output_value"
    );

INSERT INTO
    traces
VALUES
    (
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
    ("test_service_name", "test_node_id");

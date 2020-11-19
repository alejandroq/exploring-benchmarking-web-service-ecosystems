use log::error;
use sqlx::MySqlPool;
use tracks_core::tracks::Error;

pub struct Options {
    pub service_name: String,
    pub node_id: String,
    pub client_id: Option<String>,
    pub request_id: Option<String>,
}

#[derive(Clone)]
pub struct Envs {
    pub service_name: String,
    pub node_id: String,
}

pub struct TracksService {}

impl TracksService {
    pub async fn register(conn: &MySqlPool, envs: Envs) -> Result<(), Error> {
        match sqlx::query!(
            r#"
                INSERT INTO 
                    registrations 
                VALUES 
                    (?, ?)
            "#,
            envs.service_name,
            envs.node_id
        )
        .execute(conn)
        .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::Sql(err)),
        }
    }

    pub async fn read(conn: &MySqlPool, key_path: String, opts: Options) -> Result<String, Error> {
        match TracksService::read_value(conn, key_path.clone(), opts.service_name.clone()).await {
            Ok(result) => {
                match TracksService::write_trace(&conn, key_path, result.clone(), opts).await {
                    Err(err) => {
                        error!("{}", err);
                    }
                    _ => {}
                };

                Ok(result)
            }
            Err(err) => Err(err),
        }
    }

    async fn write_trace(
        conn: &MySqlPool,
        key_path: String,
        value: String,
        opts: Options,
    ) -> Result<(), Error> {
        match sqlx::query!(
            r#"
                INSERT INTO
                    traces
                VALUES
                (
                    ?,
                    ?,
                    ?,
                    ?,
                    ?,
                    ?,
                    NOW()
                );
            "#,
            opts.service_name,
            opts.client_id,
            opts.node_id,
            opts.request_id,
            key_path,
            value
        )
        .execute(conn)
        .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::Sql(err)),
        }
    }

    async fn write_value(
        conn: &MySqlPool,
        service_name: String,
        key_path: String,
        value: String,
    ) -> Result<(), Error> {
        match sqlx::query!(
            r#"
                INSERT INTO
                    tracks
                VALUES
                    (
                        ?,
                        ?,
                        ?
                    );
            "#,
            key_path,
            service_name,
            value
        )
        .execute(conn)
        .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::Sql(err)),
        }
    }

    async fn read_value(
        conn: &MySqlPool,
        key_path: String,
        service_name: String,
    ) -> Result<String, Error> {
        match sqlx::query!(
            r#"
                SELECT
                    output_value
                FROM
                    tracks
                WHERE
                    key_path = ?
                    AND service_id = ?
            "#,
            key_path,
            service_name
        )
        .fetch_one(conn)
        .await
        {
            Ok(rec) => Ok(rec.output_value),
            Err(err) => {
                if format!("{}", err).contains("found no row when we expected at least one") {
                    Err(Error::NotFound)
                } else {
                    Err(Error::Sql(err))
                }
            }
        }
    }
}

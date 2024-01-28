use sqlx::postgres::PgPool;

use crate::models::query::QueryModel;

pub async fn get_data(pool: &PgPool, state: &str) -> Vec<QueryModel> {
    let rows = sqlx::query!(
        r#"SELECT state,code 
        FROM  callback
        WHERE state = $1"#,
        state
    )
    .fetch_all(pool)
    .await
    .unwrap();

    rows.iter()
        .map(|r| QueryModel {
            code: r.code.as_ref().unwrap().clone(),
            state: r.state.as_ref().unwrap().clone(),
        })
        .collect()
}

pub async fn new_data(pool: &PgPool, data: QueryModel) -> QueryModel {
    let res = sqlx::query!(
        r#"SELECT state,code,id 
        FROM callback 
        WHERE state = $1
        "#,
        data.state
    )
    .fetch_one(pool)
    .await;
    match res {
        Err(err) => {
            match err {
                sqlx::Error::RowNotFound => {
                    // Handle the RowNotFound error here

                    let row = sqlx::query!(
                        r#"INSERT INTO callback (state,code)
                                    VALUES ($1,$2)
                                    RETURNING state,code,id"#,
                        data.state,
                        data.code
                    )
                    .fetch_one(pool)
                    .await
                    .unwrap();
                    QueryModel {
                        code: row.code.unwrap(),
                        state: row.state.unwrap(),
                    }
                }
                _ => QueryModel {
                    code: "".to_string(),
                    state: "".to_string(),
                },
            }
        }

        Ok(res) => {
            let row = sqlx::query!(
                r#"UPDATE callback SET code=$1
                    WHERE id=$2
                    RETURNING state,code"#,
                data.code,
                res.id
            )
            .fetch_one(pool)
            .await
            .unwrap();
            QueryModel {
                code: row.code.unwrap(),
                state: row.state.unwrap(),
            }
        }
    }
}

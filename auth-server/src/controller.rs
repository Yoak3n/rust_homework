use std::thread;

use sqlx::postgres::PgPool;

use crate::models::query::QueryModel;

pub async fn get_data(pool: &PgPool, state: &str) -> QueryModel {
    let mut qs = QueryModel {
        code: "".to_string(),
        state: "".to_string(),
        time: None,
    };
    println!("init :{:?}", qs);
    let mut err_count = 0;
    loop {
        let rows = sqlx::query!(
            r#"SELECT state,code,time
            FROM  callback
            WHERE state = $1"#,
            state
        )
        .fetch_one(pool)
        .await;
        match rows {
            Ok(rows) => {
                qs = QueryModel {
                        code: rows.code.as_ref().unwrap().clone(),
                        state: rows.state.as_ref().unwrap().clone(),
                        time: rows.time.clone(),
                    };
                println!("ok?");
                break;
            }
            Err(err) => {
                println!("err:{}", err);
                err_count += 1;
                println!("err_count:{}", err_count);
                if err_count > 30 {
                    return QueryModel {
                        code: "失败次数过多".to_string(),
                        state: "".to_string(),
                        time: None,
                    };
                }
                thread::sleep(std::time::Duration::from_millis(1000));
                continue;
            }
        }
    }
    println!("qs:{:?}", qs);
    return qs;
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
                        r#"INSERT INTO callback (state,code,time)
                                    VALUES ($1,$2,$3)
                                    RETURNING state,code,id,time"#,
                        data.state,
                        data.code,
                        data.time
                    )
                    .fetch_one(pool)
                    .await
                    .unwrap();

                    QueryModel {
                        code: row.code.unwrap(),
                        state: row.state.unwrap(),
                        time: row.time,
                    }
                }
                _ => QueryModel {
                    code: "".to_string(),
                    state: "".to_string(),
                    time: None,
                },
            }
        }

        Ok(res) => {
            let row = sqlx::query!(
                r#"UPDATE callback SET code=$1,time=$2
                    WHERE id=$3
                    RETURNING state,code"#,
                data.code,
                data.time,
                res.id,
            )
            .fetch_one(pool)
            .await
            .unwrap();
            QueryModel {
                code: row.code.unwrap(),
                state: row.state.unwrap(),
                time: data.time,
            }
        }
    }
}

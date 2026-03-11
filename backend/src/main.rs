use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool, FromRow};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct CountResponse {
    count: i64,
}

#[derive(Debug, Deserialize)]
struct IncrementRequest {
    amount: Option<i64>,
}

#[derive(Debug, Serialize)]
struct MessageResponse {
    message: String,
    count: i64,
}

// 初始化数据库
async fn init_db(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS counts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            count INTEGER NOT NULL DEFAULT 0,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
    )
    .execute(pool)
    .await?;

    // 插入初始记录
    sqlx::query("INSERT OR IGNORE INTO counts (id, count) VALUES (1, 0)")
        .execute(pool)
        .await?;

    Ok(())
}

// 获取当前计数
async fn get_count(pool: web::Data<SqlitePool>) -> HttpResponse {
    match sqlx::query_as::<_, CountResponse>("SELECT count FROM counts WHERE id = 1")
        .fetch_one(pool.get_ref())
        .await
    {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Failed to get count"
        })),
    }
}

// 增加计数
async fn increment_count(
    pool: web::Data<SqlitePool>,
    body: web::Json<IncrementRequest>,
) -> HttpResponse {
    let amount = body.amount.unwrap_or(1);

    match sqlx::query("UPDATE counts SET count = count + ?1, updated_at = CURRENT_TIMESTAMP WHERE id = 1")
        .bind(amount)
        .execute(pool.get_ref())
        .await
    {
        Ok(_) => {
            // 获取更新后的计数
            match sqlx::query_as::<_, CountResponse>("SELECT count FROM counts WHERE id = 1")
                .fetch_one(pool.get_ref())
                .await
            {
                Ok(result) => HttpResponse::Ok().json(MessageResponse {
                    message: "Count incremented successfully".to_string(),
                    count: result.count,
                }),
                Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Failed to get updated count"
                })),
            }
        }
        Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Failed to increment count"
        })),
    }
}

// 重置计数
async fn reset_count(pool: web::Data<SqlitePool>) -> HttpResponse {
    match sqlx::query("UPDATE counts SET count = 0, updated_at = CURRENT_TIMESTAMP WHERE id = 1")
        .execute(pool.get_ref())
        .await
    {
        Ok(_) => HttpResponse::Ok().json(MessageResponse {
            message: "Count reset successfully".to_string(),
            count: 0,
        }),
        Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Failed to reset count"
        })),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 使用当前目录的数据库文件
    let db_path = PathBuf::from("./hello_claw.db").canonicalize().unwrap_or_else(|_| PathBuf::from("./hello_claw.db"));
    let database_url = format!("sqlite://{}", db_path.display());
    
    println!("📊 Database path: {}", db_path.display());

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    init_db(&pool)
        .await
        .expect("Failed to initialize database");

    let pool_data = web::Data::new(pool);

    println!("🚀 Server starting at http://0.0.0.0:10000");
    println!("📊 API Endpoints:");
    println!("   GET  /api/count     - 获取当前计数");
    println!("   POST /api/increment - 增加计数");
    println!("   POST /api/reset     - 重置计数");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .app_data(pool_data.clone())
            .route("/api/count", web::get().to(get_count))
            .route("/api/increment", web::post().to(increment_count))
            .route("/api/reset", web::post().to(reset_count))
    })
    .bind("0.0.0.0:10000")?
    .run()
    .await
}

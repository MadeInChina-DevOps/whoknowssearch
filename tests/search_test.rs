#[cfg(test)]
mod tests {
    use rocket::{local::asynchronous::Client, routes};
    use sqlx::{PgPool, Executor};
    use rocket::http::Status;
    use whoknows_nooneknows::routes::api::search::{search, SearchResult};

    // Set up a test database
    async fn setup_test_db() -> PgPool {
        let db_url = "postgres://postgres:postgres@localhost:5432/postgres"; // Replace with your actual database URL
        let pool = PgPool::connect(db_url)
            .await
            .expect("Failed to connect to the database");

        pool.execute(
            r#"
            CREATE TABLE IF NOT EXISTS pages (
                title TEXT PRIMARY KEY,
                url TEXT NOT NULL UNIQUE,
                language TEXT NOT NULL DEFAULT 'en' CHECK (language IN ('en', 'da')),
                last_updated TIMESTAMP,
                content TEXT NOT NULL
            );
            TRUNCATE TABLE pages;
            INSERT INTO pages (title, url, language, content)
            VALUES
            ('Rust Documentation', 'https://doc.rust-lang.org/', 'en', 'Rust is a programming language used for systems programming.');
            "#
        )
        .await
        .expect("Failed to set up test database");

        pool
    }

    #[rocket::async_test]
    async fn test_search_with_results() {
        let pool = setup_test_db().await;

        let rocket = rocket::build()
            .manage(pool.clone())
            .mount("/", routes![search]);

        let client = Client::tracked(rocket).await.unwrap();

        let response = client
            .get("/search?q=Rust&language=en")
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::Ok);

        let body = response.into_json::<Vec<SearchResult>>().await.unwrap();
        assert!(!body.is_empty(), "Expected results, but got an empty response.");
        assert_eq!(body[0].title, "Rust Documentation");
    }

    #[rocket::async_test]
    async fn test_search_no_results() {
        let pool = setup_test_db().await;

        let rocket = rocket::build()
            .manage(pool.clone())
            .mount("/", routes![search]);

        let client = Client::tracked(rocket).await.unwrap();

        let response = client
            .get("/search?q=NonExistent&language=en")
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::Ok);

        let body = response.into_json::<Vec<SearchResult>>().await.unwrap();
        assert!(body.is_empty(), "Expected no results, but got some.");
    }

    #[rocket::async_test]
    async fn test_search_with_no_query() {
        let pool = setup_test_db().await;

        let rocket = rocket::build()
            .manage(pool.clone())
            .mount("/", routes![search]);

        let client = Client::tracked(rocket).await.unwrap();

        let response = client
            .get("/search?language=en")
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::BadRequest);
    }
}
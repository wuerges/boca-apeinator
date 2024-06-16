use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use sqlx::Pool;

	// $sql = "select distinct r.runnumber as number, r.rundatediff as timestamp, " .
	// 	"p.problemname as problem, r.runstatus as status, " .
	// 	"a.yes as yes, u.username as username, " .
	// 	"a.runanswer as answer " .
	// 	"from runtable as r, problemtable as p, answertable as a, usertable as u " .
	// 	"where r.contestnumber=$contest and p.contestnumber=r.contestnumber and u.contestnumber=r.contestnumber and " .

#[derive(sqlx::FromRow, Debug)]
struct Run {
    number: i64,
    timestamp: String,
    problem: String,
    status: String,
    yes: bool,
    username: String,
}

async fn select_runs(pool: &Pool<sqlx::Postgres>) -> Result<Vec<Run>, sqlx::Error> {
    let query = r#"select 
                    distinct r.runnumber as number,
                    r.rundatediff as timestamp,
                    p.problemname as problem,
                    r.runstatus as status,
                    a.yes as yes,
                    u.username as username
                    from runtable as r 
                    join problemtable as p on r.contestnumber = p.contestnumber 
                    join answertable as a on a.answernumber = r.runanswer 
                    join usertable as u on u.contestnumber = r.contestnumber 
                    order by r.rundatediff desc"#;

    sqlx::query_as(query).fetch_all(pool).await
}

#[actix_web::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("must set database url");
    // Create a connection pool
    //  for MySQL/MariaDB, use MySqlPoolOptions::new()
    //  for SQLite, use SqlitePoolOptions::new()
    //  etc.
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let rows = select_runs(&pool).await?;
    println!("rows: {:?}", rows);

    // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL/MariaDB)
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;

    assert_eq!(row.0, 150);

    Ok(())
}

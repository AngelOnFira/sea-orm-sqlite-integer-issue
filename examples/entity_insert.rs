use entity_insert::entities::number_table;
use sea_orm::{ActiveModelTrait, Database, DatabaseConnection, EntityTrait, Set};

#[tokio::main]
async fn main() {
    let db: DatabaseConnection = Database::connect("sqlite://./db.sqlite3").await.unwrap();

    let big_number: i64 = 999_999_999_999_999;

    // Insert a large number into the database
    let _test_number = number_table::ActiveModel {
        some_number: Set(big_number),
        ..Default::default()
    }
    .insert(&db)
    .await
    .unwrap();

    // Get it back out
    let test_number = number_table::Entity::find_by_id(1).one(&db).await.unwrap();

    println!(
        "Inserted number: {}, retrieved number: {}",
        big_number,
        test_number.unwrap().some_number
    );
}

use entity_remove::entities::number_table;
use sea_orm::{Database, DatabaseConnection, EntityTrait};

#[tokio::main]
async fn main() {
    let db: DatabaseConnection = Database::connect("sqlite://./db.sqlite3").await.unwrap();

    // Get a number from the database
    let test_number = number_table::Entity::find_by_id(1).one(&db).await.unwrap();
    println!(
        "Getting number out from generated entities: {:?}",
        test_number.unwrap().some_number
    );
}

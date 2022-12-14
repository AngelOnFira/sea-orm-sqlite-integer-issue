# SeaORM Sqlite Bug PoC

This example shows how generating entites from an Sqlite database with SeaORM
might result in data being extracted incorrectly.

## Steps

1. Create a new Sqlite database.
2. Run migrations. A migration consists of the following line:

`.col(ColumnDef::new(NumberTable::SomeNumber).big_integer().not_null())`

3. Generate entities. This will create a model with the `SomeNumber` field, but
   it will be of type `i32` instead of `i64`:

```rust
pub struct Model {
    pub id: i32,
    pub some_number: i32,
}
```

4. Run code using entities that can insert number `999_999_999_999_999`
   (>i32::MAX) using altered entity files:

```rust
pub struct Model {
    pub id: i32,
    pub some_number: i64, // <--- Changed from i32 to i64
}
```

`Inserted number: 999999999999999, retrieved number: 999999999999999`

5. Try to remove this number using the generated entity. This will extract a 32
   bit number from the database, which will be negative and thus not match the
   inserted number.

`Getting number out from generated entities: -1530494977`

## Running

The following *should* work and show the same behaviour:

```bash
rm db.sqlite3
sqlite3 db.sqlite3 "VACUUM;"

sea-orm-cli migrate

sea-orm-cli generate entity \
    -o entity_remove/src/entities

cargo build --example entity_insert
cargo build --example entity_remove
```

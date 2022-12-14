```bash
rm db.sqlite3
sqlite3 db.sqlite3 "VACUUM;"

sea-orm-cli migrate

sea-orm-cli generate entity \
    -o entity_remove/src/entities

cargo build --example entity_insert
cargo build --example entity_remove
```

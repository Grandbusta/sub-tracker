# Sub Tracker


## Setup

Add migration:
```bash
sqlx migrate add migration_name --source src/db/migrations
```

Run migration:
```bash
sqlx migrate run --source src/db/migrations
```
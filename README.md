# TODOs Example

## Setup

1. Declare the database URL

    ```
    export DATABASE_URL="postgres://postgres:password@localhost/todos"
    ```

2. Create the database.

    ```
    $ sqlx db create
    ```

3. Run sql migrations

    ```
    $ sqlx migrate run
    ```

## Usage

```
cargo run
```

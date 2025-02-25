# idb

A futures based crate for interacting with IndexedDB on browsers using webassembly.

## Usage

To use `idb`, you need to add the following to your `Cargo.toml`:

```toml
[dependencies]
idb = "0.3"
```

### Example

To create a new database, you can use `Factory::open`:

```rust
use idb::{Database, Error, Factory};

async fn create_database() -> Result<Database, Error> {
    // Get a factory instance from global scope
    let factory = Factory::new()?;

    // Create an open request for the database
    let mut open_request = factory.open("test", 1).unwrap();

    // Add an upgrade handler for database
    open_request.on_upgrade_needed(|event| {
        // Get database instance from event
        let database = event.database().unwrap();

        // Prepare object store params
        let mut store_params = ObjectStoreParams::new();
        store_params.auto_increment(true);
        store_params.key_path(Some(KeyPath::new_single("id")));

        // Create object store
        let store = database
            .create_object_store("employees", store_params)
            .unwrap();

        // Prepare index params
        let mut index_params = IndexParams::new();
        index_params.unique(true);

        // Create index on object store
        store
            .create_index("email", KeyPath::new_single("email"), Some(index_params))
            .unwrap();
    });

    // `await` open request
    open_request.await
}
```

To add data to an object store, you can use `ObjectStore::add`:

```rust
use idb::{Database, Error};
use serde::Serialize;
use serde_wasm_bindgen::Serializer;

async fn add_data(database: &Database) -> Result<JsValue, Error> {
    // Create a read-write transaction
    let transaction = database.transaction(&["employees"], TransactionMode::ReadWrite)?;

    // Get the object store
    let store = transaction.object_store("employees").unwrap();

    // Prepare data to add
    let employee = serde_json::json!({
        "name": "John Doe",
        "email": "john@example.com",
    });

    // Add data to object store
    let id = store
        .add(
            &employee.serialize(&Serializer::json_compatible()).unwrap(),
            None,
        )
        .await?;

    // Commit the transaction
    transaction.commit().await?;

    Ok(id)
}
```

To get data from an object store, you can use `ObjectStore::get`:

```rust
async fn get_data(database: &Database, id: JsValue) -> Result<Option<serde_json::Value>, Error> {
    // Create a read-only transaction
    let transaction = database
        .transaction(&["employees"], TransactionMode::ReadOnly)
        .unwrap();

    // Get the object store
    let store = transaction.object_store("employees").unwrap();

    // Get the stored data
    let stored_employee: Option<JsValue> = store.get(id).await?;

    // Deserialize the stored data
    let stored_employee: Option<serde_json::Value> = stored_employee
        .map(|stored_employee| serde_wasm_bindgen::from_value(stored_employee).unwrap());

    // Wait for the transaction to complete
    transaction.done().await?;

    Ok(stored_employee)
}
```

For more examples on using other functionality, see the
[tests](https://github.com/devashishdxt/idb/tree/main/idb/tests) directory.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as
defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

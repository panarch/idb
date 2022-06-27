use idb::Factory;
use wasm_bindgen_test::wasm_bindgen_test;

#[wasm_bindgen_test]
fn test_factory_new_pass() {
    let factory = Factory::new();
    assert!(
        factory.is_ok(),
        "Factory::new() should be Ok(): {}",
        factory.unwrap_err()
    );
}

#[wasm_bindgen_test]
async fn test_factory_open_delete_pass() {
    let factory = Factory::new().unwrap();

    let open_request = factory.open("test", 1);
    assert!(
        open_request.is_ok(),
        "Factory::open() should be Ok(): {}",
        open_request.unwrap_err()
    );

    let database = open_request.unwrap().execute().await;
    assert!(
        database.is_ok(),
        "OpenRequest::execute() should be Ok(): {}",
        database.unwrap_err()
    );
    let database = database.unwrap();

    database.close();

    let delete = factory.delete("test").await;
    assert!(
        delete.is_ok(),
        "Factory::delete() should be Ok(): {}",
        delete.unwrap_err()
    );
}
mod key_path;
mod object_store_params;

pub use self::{key_path::KeyPath, object_store_params::ObjectStoreParams};

use std::ops::Deref;

use wasm_bindgen::{JsCast, JsValue};
use web_sys::IdbObjectStore;

use crate::{
    utils::dom_string_list_to_vec, CursorDirection, Error, Index, IndexParams, Query, StoreRequest,
    Transaction,
};

/// Represents an object store in a database.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectStore {
    inner: IdbObjectStore,
}

impl ObjectStore {
    /// Returns the name of the store.
    pub fn name(&self) -> String {
        self.inner.name()
    }

    /// Updates the name of the store to newName. Returns and [`Error`] if not called within an upgrade transaction.
    pub fn set_name(&self, name: &str) {
        self.inner.set_name(name)
    }

    /// Returns the key path of the store.
    pub fn key_path(&self) -> Result<Option<KeyPath>, Error> {
        let inner_key_path = self.inner.key_path().map_err(Error::KeyPathNotFound)?;

        if inner_key_path.is_null() {
            Ok(None)
        } else {
            Some(inner_key_path.try_into()).transpose()
        }
    }

    /// Returns a list of the names of indexes in the store.
    pub fn index_names(&self) -> Vec<String> {
        dom_string_list_to_vec(&self.inner.index_names())
    }

    /// Returns the associated [`Transaction`].
    pub fn transaction(&self) -> Transaction {
        self.inner.transaction().into()
    }

    /// Returns `true` if the store has a key generator, and `false` otherwise.
    pub fn auto_increment(&self) -> bool {
        self.inner.auto_increment()
    }

    /// Adds or updates a record in store with the given value and key.
    pub fn put(&self, value: &JsValue, key: Option<&JsValue>) -> Result<StoreRequest, Error> {
        match key {
            None => self.inner.put(value),
            Some(key) => self.inner.put_with_key(value, key),
        }
        .map(Into::into)
        .map_err(Error::UpdateFailed)
    }

    /// Adds a record in store with the given value and key.
    pub fn add(&self, value: &JsValue, key: Option<&JsValue>) -> Result<StoreRequest, Error> {
        match key {
            None => self.inner.add(value),
            Some(key) => self.inner.add_with_key(value, key),
        }
        .map(Into::into)
        .map_err(Error::AddFailed)
    }

    /// Deletes records in store with the given key or in the given key range in query.
    pub fn delete(&self, query: impl Into<Query>) -> Result<StoreRequest, Error> {
        self.inner
            .delete(&query.into().into())
            .map(Into::into)
            .map_err(Error::DeleteFailed)
    }

    /// Deletes all records in store.
    pub fn clear(&self) -> Result<StoreRequest, Error> {
        self.inner
            .clear()
            .map(Into::into)
            .map_err(Error::ClearFailed)
    }

    /// Retrieves the value of the first record matching the given key or key range in query.
    pub fn get(&self, query: impl Into<Query>) -> Result<StoreRequest, Error> {
        self.inner
            .get(&query.into().into())
            .map(Into::into)
            .map_err(Error::GetFailed)
    }

    /// Retrieves the key of the first record matching the given key or key range in query.
    pub fn get_key(&self, query: impl Into<Query>) -> Result<StoreRequest, Error> {
        self.inner
            .get_key(&query.into().into())
            .map(Into::into)
            .map_err(Error::GetKeyFailed)
    }

    /// Retrieves the values of the records matching the given key or key range in query (up to limit if given).
    pub fn get_all(
        &self,
        query: Option<impl Into<Query>>,
        limit: Option<u32>,
    ) -> Result<StoreRequest, Error> {
        match (query, limit) {
            (Some(query), Some(limit)) => self
                .inner
                .get_all_with_key_and_limit(&query.into().into(), limit)
                .map(Into::into)
                .map_err(Error::GetAllKeysFailed),
            (Some(query), None) => self
                .inner
                .get_all_with_key(&query.into().into())
                .map(Into::into)
                .map_err(Error::GetAllKeysFailed),
            (None, Some(limit)) => self
                .inner
                .get_all_with_key_and_limit(&JsValue::null(), limit)
                .map(Into::into)
                .map_err(Error::GetAllKeysFailed),
            (None, None) => self
                .inner
                .get_all()
                .map(Into::into)
                .map_err(Error::GetAllKeysFailed),
        }
    }

    /// Retrieves the keys of records matching the given key or key range in query (up to limit if given).
    pub fn get_all_keys(
        &self,
        query: Option<impl Into<Query>>,
        limit: Option<u32>,
    ) -> Result<StoreRequest, Error> {
        match (query, limit) {
            (Some(query), Some(limit)) => self
                .inner
                .get_all_keys_with_key_and_limit(&query.into().into(), limit)
                .map(Into::into)
                .map_err(Error::GetAllKeysFailed),
            (Some(query), None) => self
                .inner
                .get_all_keys_with_key(&query.into().into())
                .map(Into::into)
                .map_err(Error::GetAllKeysFailed),
            (None, Some(limit)) => self
                .inner
                .get_all_keys_with_key_and_limit(&JsValue::null(), limit)
                .map(Into::into)
                .map_err(Error::GetAllKeysFailed),
            (None, None) => self
                .inner
                .get_all_keys()
                .map(Into::into)
                .map_err(Error::GetAllKeysFailed),
        }
    }

    /// Retrieves the number of records matching the given key or key range in query.
    pub fn count(&self, query: Option<impl Into<Query>>) -> Result<StoreRequest, Error> {
        match query {
            None => self
                .inner
                .count()
                .map(Into::into)
                .map_err(Error::CountFailed),
            Some(query) => self
                .inner
                .count_with_key(&query.into().into())
                .map(Into::into)
                .map_err(Error::CountFailed),
        }
    }

    /// Opens a [`Cursor`](crate::Cursor) over the records matching query, ordered by direction. If query is `None`,
    /// all records in store are matched.
    pub fn open_cursor(
        &self,
        query: Option<impl Into<Query>>,
        cursor_direction: Option<CursorDirection>,
    ) -> Result<StoreRequest, Error> {
        match (query, cursor_direction) {
            (Some(query), Some(cursor_direction)) => self
                .inner
                .open_cursor_with_range_and_direction(&query.into().into(), cursor_direction.into())
                .map(Into::into)
                .map_err(Error::OpenCursorFailed),
            (Some(query), None) => self
                .inner
                .open_cursor_with_range(&query.into().into())
                .map(Into::into)
                .map_err(Error::OpenCursorFailed),
            (None, Some(cursor_direction)) => self
                .inner
                .open_cursor_with_range_and_direction(&JsValue::null(), cursor_direction.into())
                .map(Into::into)
                .map_err(Error::OpenCursorFailed),
            (None, None) => self
                .inner
                .open_cursor()
                .map(Into::into)
                .map_err(Error::OpenCursorFailed),
        }
    }

    /// Opens a [`KeyCursor`](crate::KeyCursor) over the records matching query, ordered by direction. If query is
    /// `None`, all records in store are matched.
    pub fn open_key_cursor(
        &self,
        query: Option<impl Into<Query>>,
        cursor_direction: Option<CursorDirection>,
    ) -> Result<StoreRequest, Error> {
        match (query, cursor_direction) {
            (Some(query), Some(cursor_direction)) => self
                .inner
                .open_key_cursor_with_range_and_direction(
                    &query.into().into(),
                    cursor_direction.into(),
                )
                .map(Into::into)
                .map_err(Error::OpenCursorFailed),
            (Some(query), None) => self
                .inner
                .open_key_cursor_with_range(&query.into().into())
                .map(Into::into)
                .map_err(Error::OpenCursorFailed),
            (None, Some(cursor_direction)) => self
                .inner
                .open_key_cursor_with_range_and_direction(&JsValue::null(), cursor_direction.into())
                .map(Into::into)
                .map_err(Error::OpenCursorFailed),
            (None, None) => self
                .inner
                .open_key_cursor()
                .map(Into::into)
                .map_err(Error::OpenCursorFailed),
        }
    }

    /// Returns an [`Index`] for the index named name in store.
    pub fn index(&self, name: &str) -> Result<Index, Error> {
        self.inner
            .index(name)
            .map(Into::into)
            .map_err(Error::IndexNotFound)
    }

    /// Creates a new index in store with the given name, key path and options and returns a new [`Index`]. Returns an
    /// [`Error`] if not called within an upgrade transaction.
    pub fn create_index(
        &self,
        name: &str,
        key_path: KeyPath,
        params: Option<IndexParams>,
    ) -> Result<Index, Error> {
        match params {
            None => self
                .inner
                .create_index_with_str_sequence(name, &key_path.into()),
            Some(params) => self
                .inner
                .create_index_with_str_sequence_and_optional_parameters(
                    name,
                    &key_path.into(),
                    &params,
                ),
        }
        .map(Into::into)
        .map_err(Error::IndexCreateFailed)
    }

    /// Deletes the index in store with the given name. Returns an [`Error`] if not called within an upgrade
    /// transaction.
    pub fn delete_index(&self, name: &str) -> Result<(), Error> {
        self.inner
            .delete_index(name)
            .map_err(Error::IndexDeleteFailed)
    }
}

impl Deref for ObjectStore {
    type Target = IdbObjectStore;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<IdbObjectStore> for ObjectStore {
    fn from(inner: IdbObjectStore) -> Self {
        Self { inner }
    }
}

impl From<ObjectStore> for IdbObjectStore {
    fn from(object_store: ObjectStore) -> Self {
        object_store.inner
    }
}

impl TryFrom<JsValue> for ObjectStore {
    type Error = Error;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        value
            .dyn_into::<IdbObjectStore>()
            .map(Into::into)
            .map_err(|value| Error::UnexpectedJsType("IdbObjectStore", value))
    }
}

impl From<ObjectStore> for JsValue {
    fn from(value: ObjectStore) -> Self {
        value.inner.into()
    }
}

use std::cell::RefCell;

use dashmap::DashMap;
use sea_orm::DatabaseConnection;

thread_local! {
    static THREAD_MYSQL_POOL: RefCell<Option<DatabaseConnection>> = RefCell::new(None)
}

pub fn set_mysql_pool(pool: DatabaseConnection) {
    THREAD_MYSQL_POOL.with(|p| *p.borrow_mut() = Some(pool))
}

pub fn get_mysql_pool() -> Option<DatabaseConnection> {
    THREAD_MYSQL_POOL.with(|p| p.borrow().clone())
}

pub fn clear_mysql_pool() {
    THREAD_MYSQL_POOL.with(|p| *p.borrow_mut() = None)
}

type PoolMap = DashMap<String, DatabaseConnection>;

lazy_static::lazy_static! {
    pub static ref POOL_MAP: PoolMap = DashMap::new();
}

pub fn get_map_mysql_pool(server_config_id: &str) -> Option<DatabaseConnection> {
    POOL_MAP
        .get(server_config_id)
        .map(|value| value.value().clone())
}

pub fn set_map_mysql_pool(server_config_id: String, pool: DatabaseConnection) {
    POOL_MAP.insert(server_config_id, pool);
}

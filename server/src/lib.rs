//! patic server

/// models
pub mod models {
    pub mod db_connection;
    pub mod post;
}

/// routes
pub mod routes {
    pub mod post;
}

/// services
pub mod services {
    pub mod post;
}

pub mod schema;

#[macro_use]
extern crate diesel;

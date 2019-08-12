use crate::schema::remote_servers;
use serde::Serialize;

/// Server struct holds the schema for a querable database entity to the remote_servers table.
#[derive(Debug, Queryable, Serialize)]
pub struct Server {
    pub server_name: String,
    pub ip_address: String,
}

/// NewServer struct holds the schema for a insertable database entity to the remote_servers table.
#[derive(Debug, Insertable)]
#[table_name = "remote_servers"]
pub struct NewServer {
    pub servername: String,
    pub ip_address: String,
}

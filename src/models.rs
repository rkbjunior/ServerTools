use crate::schema::remote_servers;
use serde::Serialize;

#[derive(Debug, Queryable, Serialize)]
pub struct Server {
	pub server_name: String,
	pub ip_address: String,
}

#[derive(Debug, Insertable)]
#[table_name="remote_servers"]
pub struct NewServer {
	pub servername: String,
	pub ip_address: String,
}
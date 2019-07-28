use schema::remote_servers;

#[derive(Debug, Queryable)]
pub struct Server {
	pub server_name: String,
	pub ip_address: String,
}

#[derive(Debug, Insertable)]
#[table_name="remote_servers"]
pub struct NewServer {
	pub server_name: String,
	pub ip_address: String,
}
extern crate dotenve;
use dotenv::dotenv;
use std::env;

infer_schema!("dotenv:DATABASE_URL");

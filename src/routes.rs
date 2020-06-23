extern crate rocket;

use rocket::Route;

mod transactions;

pub fn transaction_routes() -> Vec<Route> {
    routes![transactions::details, transactions::about, transactions::all]
}

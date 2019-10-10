extern crate diesel;

use diesel::prelude::*;
use diesel_test::establish_connection;
use diesel_test::models::Commodity;
use diesel::mysql::Mysql;


fn main() {
    use diesel_test::schema::commodity::dsl::*;

    let connection = establish_connection();
    let q = commodity
        .filter(used.is_null())
        .limit(5);
    println!("{}", diesel::debug_query::<Mysql, _>(&q).to_string());
    let results = commodity
        .filter(used.is_null())
        .limit(5)
        .get_results::<Commodity>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for c in results {
        println!("{}", c.commodity_name);
        println!("-----------\n");
        println!("{:?}", c.lastPutInPrice);
    }

    dbg!(diesel::update(commodity.filter(used.is_null())).set(commodity_name.eq("abcd")).execute(&connection));
}
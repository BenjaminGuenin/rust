extern crate diesel_demo;
extern crate diesel;

use self::diesel_demo::*;
use self::models::*;
use self::diesel::prelude::*;

fn main()
{
    // Allows us to import many aliases
    use diesel_demo::schema::posts::dsl::*;

    let dbConnection = establish_connection();
    let results = posts.filter(published.eq(true))
        .limit(5)
        .load::<Post>(&dbConnection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());

    for post in results
    {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
}
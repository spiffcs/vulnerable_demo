use diesel_demo::*;
use diesel_demo::models::*;
use diesel::prelude::*;

fn main() {
    use diesel_demo::schema::posts::dsl::*;

    let mut connection = diesel_demo::establish_connection();

    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(&mut connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }
}

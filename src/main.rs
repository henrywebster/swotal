extern crate mustache;

use mustache::MapBuilder;
use serde::Serialize;
use sqlite::State;
use std::fs;
use std::fs::File;

#[derive(Serialize)]
struct Post {
    title: String,
    link: String,
    category: String,
}

#[derive(Serialize)]
struct Meta {
    title: String,
    description: String,
}

fn get_posts(conn: &sqlite::Connection) -> Vec<Post> {
    let mut statement = conn
        .prepare("SELECT title, link, name AS category FROM posts LEFT JOIN tags WHERE posts.category = tags.id ORDER BY created_at DESC")
        .expect("Could not create statement");

    let mut posts = Vec::new();

    // TODO integrate directly in MapBuilder with the insert func?
    while let Ok(State::Row) = statement.next() {
        let post = Post {
            title: statement.read::<String, _>("title").unwrap(),
            link: statement.read::<String, _>("link").unwrap(),
            category: statement.read::<String, _>("category").unwrap(),
        };
        posts.push(post);
    }
    return posts;
}

fn get_meta(conn: &sqlite::Connection) -> Meta {
    let mut statement = conn
        .prepare("SELECT title, description FROM meta LIMIT 1")
        .expect("Could not create statement");

    let meta = match statement.next() {
        Ok(_) => Meta {
            title: statement.read::<String, _>("title").unwrap(),
            description: statement.read::<String, _>("description").unwrap(),
        },
        Err(_) => panic!(),
    };
    return meta;
}

fn make_website(template: &str, conn: &sqlite::Connection) {
    // TODO use stdin if available
    // TODO use environment variables
    let template = mustache::compile_str(
        &(fs::read_to_string(template).expect("Should have been able to read file")),
    )
    .unwrap();

    let posts = get_posts(conn);
    let meta = get_meta(conn);

    let data = MapBuilder::new()
        .insert("posts", &posts)
        .expect("Could not encode")
        .insert("meta", &meta)
        .expect("Could not encode meta.")
        .build();

    let mut file = File::create("index.html").expect("Could not create file");
    template.render_data(&mut file, &data).unwrap();
}

fn main() {
    let db_file = "website.db";
    let template_file = "index.mustache";
    let connection = sqlite::open(db_file).expect("Could not open sqlite connection");

    make_website(template_file, &connection);
}

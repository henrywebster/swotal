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
    tags: Vec<String>,
}

#[derive(Serialize)]
struct Meta {
    title: String,
    description: String,
}

#[derive(Serialize)]
struct Social {
    name: String,
    link: String,
}

fn get_tags_by_post(id: i64, conn: &sqlite::Connection) -> Vec<String> {
    let mut statement = conn
        .prepare("SELECT name FROM post_tags LEFT JOIN tags ON post_tags.tag_id = tags.id WHERE post_id = ?")
        .expect("Could not prepare statement");
    statement.bind((1, id)).expect("Could not bind ID");

    let mut tags = Vec::new();
    while let Ok(State::Row) = statement.next() {
        tags.push(statement.read::<String, _>("name").unwrap());
    }
    return tags;
}

fn get_posts(conn: &sqlite::Connection) -> Vec<Post> {
    // TODO remove category column from posts table
    // TODO add explicit ID column to posts for foreign key
    // TODO fix definition of post_tags to use foreign keys
    let mut statement = conn
        .prepare("SELECT rowid AS id, title, link FROM posts ORDER BY created_at DESC")
        .expect("Could not create statement");

    let mut posts = Vec::new();

    // TODO integrate directly in MapBuilder with the insert func?
    while let Ok(State::Row) = statement.next() {
        let post = Post {
            title: statement.read::<String, _>("title").unwrap(),
            link: statement.read::<String, _>("link").unwrap(),
            tags: get_tags_by_post(statement.read::<i64, _>("id").unwrap(), conn),
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

fn get_socials(conn: &sqlite::Connection) -> Vec<Social> {
    let mut statement = conn
        .prepare("SELECT name, link FROM socials ORDER BY name")
        .expect("Could not create statement");

    let mut socials = Vec::new();

    while let Ok(State::Row) = statement.next() {
        let social = Social {
            name: statement.read::<String, _>("name").unwrap(),
            link: statement.read::<String, _>("link").unwrap(),
        };
        socials.push(social);
    }
    return socials;
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
    let socials = get_socials(conn);

    let data = MapBuilder::new()
        .insert("posts", &posts)
        .expect("Could not encode")
        .insert("meta", &meta)
        .expect("Could not encode meta")
        .insert("socials", &socials)
        .expect("Could not encode socials")
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

extern crate mustache;

use mustache::MapBuilder;
use serde::Serialize;
use std::fs;
use std::fs::File;

#[derive(Serialize)]
struct Post {
    title: String,
    link: String,
}

fn make_website(template: &str) {
    // TODO use stdin if available
    // TODO use environment variables
    let template = mustache::compile_str(
        &(fs::read_to_string(template).expect("Should have been able to read file")),
    )
    .unwrap();

    // TODO (get from DB instead)
    let post = Post {
        title: "Bernie Slamders".into(),
        link: "https://hwebs.info".into(),
    };
    // TODO (get from DB instead)
    let post2 = Post {
        title: "Bernie Slamders".into(),
        link: "https://hwebs.info".into(),
    };

    // convert from sql to this
    //    let data = MapBuilder::new()
    //        .insert_vec("posts", |builder| builder.push(&post).expect("test"))
    //        .build();

    let posts = vec![post, post2];

    let data = MapBuilder::new()
        .insert("posts", &posts)
        .expect("Could not encode")
        .build();

    let mut file = File::create("index.html").expect("Could not create file");
    template.render_data(&mut file, &data).unwrap();
}

fn main() {
    make_website(&("index.mustache"));
}

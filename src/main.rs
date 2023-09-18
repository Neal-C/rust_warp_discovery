use warp::Filter;

mod todo;
use todo::todos;

mod security;

//Macroless warp discovery
// cargo watch -q -c -w src/ -x run
// cargo watch -quiet -clear -watch <folder/path> -execute <command>

const WEB_FOLDER: &str = "web-static/";

#[tokio::main]
async fn main() {
    //APIs
    let hello_world = warp::path::end()
        .and(warp::get())
        .map(|| "Hello world from root");

    let hi = warp::path("hi").and(warp::get()).map(|| "hello from hi");

    let apis = hello_world.or(hi).or(todos());

    // Static content
    let static_content = warp::fs::dir(WEB_FOLDER);
    let index_html = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file(format! {"{}/index.html", WEB_FOLDER})); // use path::

    let static_site = static_content.or(index_html);

    let routes = apis.or(static_site);

    println!("starting web server");
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}

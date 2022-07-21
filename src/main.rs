// use actix_web::{App, Error, HttpRequest, HttpResponse, HttpS&erver, web};
// use actix_web::http::{StatusCode};
// use actix_files as fs;


// async fn index() -> Result<HttpResponse, Error> {
//     Ok(
//         HttpResponse::build(StatusCode::OK)
//             .content_type("text/html; charset=utf-8")
//             .body(include_str!("../static/templates/index.html"))
//     )
// }



// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {

//         App::new()
//         .service(fs::Files::new("/static", "static").show_files_listing())


//         .route("/", web::get().to(index))

//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }



use crate::packer::{HotBuilder, RectCollection};
use std::collections::HashMap;
use itertools::Itertools;

mod packer;

fn main() {

    // temporary add rect stacks
    // init rect_collection
    let v_id: Vec<usize> = vec![1,2,3,4,5,6];
    let v_width: Vec<usize> = vec![53, 76, 137, 62, 70, 48];
    let v_height: Vec<usize> = vec![140, 220, 230, 120, 210, 156];
    let v_stock: Vec<usize> = vec![5, 5, 2, 4, 5, 7];

    let  mut collection: RectCollection= RectCollection::new();
    collection.build_collection(v_id, v_width, v_height, v_stock);


    // container data

    let references: Vec<char> = vec!['b','l','r','f'];
    let mut shapes: Vec<String> = Vec::new();
    let mut dimensions: Vec<HashMap<&str, usize>> = Vec::new();

    //back
    let shape: String = String::from("rect");
    shapes.push(shape);
    let mut dimension: HashMap<&str, usize> = HashMap::new();
    dimension.insert("container_width", 470);
    dimension.insert("container_height", 220);
    dimension.insert("container_small_height",0);
    dimension.insert("door_width", 0);
    dimension.insert("door_height", 0);
    dimensions.push(dimension);

    //l
    let shape: String = String::from("obl");
    shapes.push(shape);
    let mut dimension: HashMap<&str, usize> = HashMap::new();
    dimension.insert("container_width", 470);
    dimension.insert("container_height", 220);
    dimension.insert("container_small_height", 180);
    dimension.insert("door_width", 0);
    dimension.insert("door_height", 0);
    dimensions.push(dimension);

    //r
    let shape: String = String::from("obl");
    shapes.push(shape);
    let mut dimension: HashMap<&str, usize> = HashMap::new();
    dimension.insert("container_width", 470);
    dimension.insert("container_height", 220);
    dimension.insert("container_small_height", 180);
    dimension.insert("door_width", 0);
    dimension.insert("door_height", 0);
    dimensions.push(dimension);

    //f
    let shape: String = String::from("door");
    shapes.push(shape);
    let mut dimension: HashMap<&str, usize> = HashMap::new();
    dimension.insert("container_width", 470);
    dimension.insert("container_height", 220);
    dimension.insert("container_small_height", 0);
    dimension.insert("door_width", 200);
    dimension.insert("door_height", 120);
    dimensions.push(dimension);

    // order of construction
    let ord_l: Vec<char> = vec!['f','b', 'l', 'r'];
    for mut order in ord_l.iter().permutations(ord_l.len()).unique() {
        // order.push(&'t');
        let mut builder: HotBuilder = HotBuilder::new   (order, 
                                                    collection.clone(), 
                                                    references.clone(), 
                                                    shapes.clone(), 
                                                    dimensions.clone()
                                                    );
        builder.run();
        println!("{:?}", builder.global_score)
    } 
}
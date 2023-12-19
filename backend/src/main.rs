use std::path::PathBuf;
use std::sync::Arc;
use rocket::State;
use rocket::form::Form;
use rocket::tokio::sync::Mutex;
use rocket::{fs::NamedFile, response::{status::NotFound, content::RawHtml}};

#[macro_use] extern crate rocket;

#[derive(Debug)]
struct PaymentEntry {
    price: f64,
    goods_type: String,
    payment_type: String,
}

#[derive(Debug)]
struct PaymentDatas {
    payments: Vec<PaymentEntry>,
}

impl PaymentDatas {
    fn new() -> PaymentDatasPointer {
        let new_payment_datas = PaymentDatas {
            payments: Vec::new(),
        };
        Arc::new(Mutex::new(new_payment_datas))
    }
}

type PaymentDatasPointer = Arc<Mutex<PaymentDatas>>;


// Return the index when the url is /
#[get("/")]
async fn index() -> Result<NamedFile, NotFound<String>> {
    get_index().await
}

#[derive(FromForm, Debug)]
struct PaymentEntryRequest<'r> {
    r#price: f64,
    r#goods_type: &'r str,
    r#payment_type: &'r str,
}



#[post("/command", data = "<payment_entry>")]
async fn command(payment_entry: Form<PaymentEntryRequest<'_>>, payment_datas: &State<PaymentDatasPointer>) -> RawHtml<String> {
    println!("youpi !");
    println!("data : {}", payment_entry.payment_type);
    let entry = PaymentEntry {
        price: payment_entry.price,
        goods_type: payment_entry.goods_type.to_string(),
        payment_type: payment_entry.payment_type.to_string(),
    };
    let mut payment_datas = payment_datas.lock().await;
    payment_datas.payments.push(entry);


    println!("payment_datas.payments : {:#?}", payment_datas.payments);

    RawHtml(format!("payment_datas.payments : {:#?}", payment_datas.payments))
}

#[get("/test")]
async fn test() -> RawHtml<String> {
    RawHtml(format!("connected"))
}

#[launch]
fn rocket() -> _ {
    // You must mount the static_files route
    rocket::build()
        .manage(PaymentDatas::new())
        .mount("/", routes![index, static_files, command, test])
}

// Return the index file as a Rocket NamedFile
async fn get_index() -> Result<NamedFile, NotFound<String>> {
    NamedFile::open("../frontend/dist/index.html")
        .await
        .map_err(|e| NotFound(e.to_string()))
}

//Create a route for any url that is a path from the /
#[get("/<path..>")]
async fn static_files(path: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = PathBuf::from("../frontend/dist").join(path);
    match NamedFile::open(path).await {
        Ok(f) => Ok(f),
        Err(_) => get_index().await,
    }
}


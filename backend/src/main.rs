use std::fs::{File, self};
use std::io::{Write, Read};
use std::path::PathBuf;
use std::sync::Arc;
use common::PaymentEntry;
use rocket::State;
use rocket::form::Form;
use rocket::tokio::sync::Mutex;
use rocket::{fs::NamedFile, response::{status::NotFound, content::RawHtml}};
use serde::{Serialize, Deserialize};

#[macro_use] extern crate rocket;

#[derive(Debug, Serialize, Deserialize)]
struct PaymentDatas {
    payments: Vec<PaymentEntry>,
}

impl PaymentDatas {
    fn new() -> PaymentDatas {
        PaymentDatas {
            payments: Vec::new(),
        }
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
    r#payment_method: &'r str,
}

#[derive(FromForm, Debug)]
struct DeleteEntryRequest {
    r#id: usize,
}

#[post("/command", data = "<payment_entry>")]
async fn command(payment_entry: Form<PaymentEntryRequest<'_>>, payment_datas: &State<PaymentDatasPointer>) -> RawHtml<String> {
    let entry = PaymentEntry {
        price: payment_entry.price,
        goods_type: payment_entry.goods_type.to_string(),
        payment_method: payment_entry.payment_method.to_string(),
    };
    let mut payment_datas = payment_datas.lock().await;
    payment_datas.payments.push(entry);

    let mut file = File::create("save.txt").unwrap();
    file.write_all(format!("{:#?}", payment_datas.payments).as_bytes());

    RawHtml(format!("payment_datas.payments : {:#?}", payment_datas.payments))
}

#[post("/delete", data = "<delete_entry>")]
async fn delete(delete_entry: Form<DeleteEntryRequest>, payment_datas: &State<PaymentDatasPointer>) -> RawHtml<String> {
    let mut payment_datas = payment_datas.lock().await;
    payment_datas.payments.remove(delete_entry.id);

    let mut file = File::create("save.txt").unwrap();
    file.write_all(format!("{:#?}", payment_datas.payments).as_bytes());

    RawHtml(format!("payment_datas.payments : {:#?}", payment_datas.payments))
}

#[get("/test")]
async fn test() -> RawHtml<String> {
    RawHtml(format!("connected"))
}

#[launch]
fn rocket() -> _ {
    // You must mount the static_files route
    let data = match try_load_save() {
        Some(loaded_data) => loaded_data,
        None => PaymentDatas::new()
    };

    let data_pointer: Arc<Mutex<PaymentDatas>> = Arc::new(Mutex::new(data));

    rocket::build()
        .manage(data_pointer)
        .mount("/", routes![index, static_files, command, test, delete])
}

fn try_load_save() -> Option<PaymentDatas> {
    let mut contents = String::new();

    let mut file = match File::open("save.txt") {
        Ok(file)  => file,
        Err(e) => return None,
    };
    file.read_to_string(&mut contents);
    match serde_json::from_str(&contents) {
        Ok(data)  => data,
        Err(e) => return None,
    }
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


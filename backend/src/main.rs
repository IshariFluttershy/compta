mod file_handling;
mod payment;

use std::path::PathBuf;
use std::sync::Arc;
use common::{PaymentEntry, PaymentDatas, PaymentTotal};
use file_handling::{save_datas, try_load_save};
use payment::{calculate_payment_total, get_date_entries_readonly, get_date_entries};
use rocket::State;
use rocket::form::Form;
use rocket::tokio::sync::{Mutex, RwLock};
use rocket::{fs::NamedFile, response::{status::NotFound, content::RawHtml}};
use rocket::serde::json::Json;

#[macro_use] extern crate rocket;

type PaymentDatasPointer = Arc<RwLock<PaymentDatas>>;

const SAVE_FILE_PATH: &str = "save.json";

#[derive(FromForm, Debug)]
struct PaymentEntryRequest<'r> {
    price: f64,
    goods_type: &'r str,
    payment_method: &'r str,
    date: i64,
}

#[derive(FromForm, Debug)]
struct DeleteEntryRequest {
    id: usize,
}

#[launch]
fn rocket() -> _ {
    // You must mount the static_files route
    env_logger::init();
    let data = match try_load_save(SAVE_FILE_PATH) {
        Ok(loaded_data) => loaded_data,
        Err(e) => {
            log::error!("Error when loading save data : {}", e);
            // faire un handle d'erreur correct et rediriger vers une page d'erreur qui dit que ya eu un souci
            panic!();
        }
    };

    let data_pointer: Arc<Mutex<PaymentDatas>> = Arc::new(Mutex::new(data));

    rocket::build()
        .manage(data_pointer)
        .mount("/", routes![index, static_files, command, delete, get_data, get_total])
}

// Return the index when the url is /
#[get("/")]
async fn index() -> Result<NamedFile, NotFound<String>> {
    get_index().await
}

#[post("/command", data = "<payment_entry>")]
async fn command(payment_entry: Form<PaymentEntryRequest<'_>>, payment_datas: &State<PaymentDatasPointer>) -> Json<PaymentDatas> {
    let entry = PaymentEntry {
        price: payment_entry.price,
        goods_type: payment_entry.goods_type.to_string(),
        payment_method: payment_entry.payment_method.to_string(),
        date: payment_entry.date,
    };
    let mut payment_datas = payment_datas.write().await;
    payment_datas.payments.push(entry);

    save_datas(&payment_datas, SAVE_FILE_PATH);
    Json(payment_datas.clone())
}

#[post("/delete", data = "<delete_entry>")]
async fn delete(delete_entry: Form<DeleteEntryRequest>, payment_datas: &State<PaymentDatasPointer>) -> RawHtml<String> {
    let mut payment_datas = payment_datas.write().await;
    payment_datas.payments.remove(delete_entry.id);

    save_datas(&payment_datas, SAVE_FILE_PATH);
    RawHtml(format!("payment_datas.payments : {:#?}", payment_datas.payments))
}

#[get("/get_data?<year>&<month>")]
async fn get_data(month: u32, year: u32, payment_datas: &State<PaymentDatasPointer>) -> Json<PaymentDatas> {
    let payment_datas = payment_datas.read().await;
    Json(get_date_entries(&payment_datas, month, year))
}

#[get("/get_total?<year>&<month>")]
async fn get_total(month: u32, year: u32, payment_datas: &State<PaymentDatasPointer>) -> Json<PaymentTotal> {
    let payment_datas = payment_datas.read().await;
    let entries = get_date_entries_readonly(&payment_datas, month, year);
    Json(calculate_payment_total(&entries))
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
use std::fs::File;
use std::io::{Write, Read};
use std::ops::Add;
use std::path::PathBuf;
use std::sync::Arc;
use common::{PaymentEntry, PaymentDatas, PaymentTotal, PaymentMethod, GoodType};
use rocket::State;
use rocket::form::Form;
use rocket::tokio::sync::{Mutex, RwLock};
use rocket::{fs::NamedFile, response::{status::NotFound, content::RawHtml}};
use rocket::serde::json::Json;
use chrono::{DateTime, Datelike, Local};
use chrono::Utc;

#[macro_use] extern crate rocket;

type PaymentDatasPointer = Arc<RwLock<PaymentDatas>>;

const SAVE_FILE_PATH: &str = "save.json";

// Return the index when the url is /
#[get("/")]
async fn index() -> Result<NamedFile, NotFound<String>> {
    get_index().await
}

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

    save_datas(&payment_datas);
    Json(payment_datas.clone())
}

fn save_datas(payment_datas: &PaymentDatas) {
    if try_save_datas(payment_datas).is_err() {
        save_emergency_datas(payment_datas);
    };
}

fn try_save_datas(payment_datas: &PaymentDatas) -> Result<(), ()>
{
    let mut file = match File::create(SAVE_FILE_PATH) {
        Ok(file) => file,
        Err(e) => {
            log::error!("Error when creating save file : {}", e);
            return Err(());
        }
    };

    let json = match serde_json::to_string(&payment_datas) {
        Ok(json) => json,
        Err(e) => {
            log::error!("Error when serializing data : {}", e);
            return Err(());
        }
    };

    match file.write_all(json.as_bytes()) {
        Err(e) => {
            log::error!("Error when writing save data on file : {}", e);
            Err(())
        }
        Ok(_) => Ok(())
    }
}

fn save_emergency_datas(payment_datas: &PaymentDatas) {
    let datetime_format: String = Local::now().format("%Y%m%y_%H%M").to_string().add(".log");
    let mut file: File = File::create(datetime_format).unwrap();
    let json: String = serde_json::to_string(&payment_datas).unwrap();
    file.write_all(json.as_bytes()).unwrap();
    // C'est mieux de mettre le programme dans un état qui permet de renvoyer a l'utilisateur que c'est la merde
    // Genre faudrait définir un enum qui dit si on est dans un état normal de fonctionnement ou non
    // Et si on est pas dans un état normal, faire un retour a l'utilisateur
    panic!("Error when saving datas. Reach the maintainer of the program to fix the problem and get emergency saved datas");
}

#[post("/delete", data = "<delete_entry>")]
async fn delete(delete_entry: Form<DeleteEntryRequest>, payment_datas: &State<PaymentDatasPointer>) -> RawHtml<String> {
    let mut payment_datas = payment_datas.write().await;
    payment_datas.payments.remove(delete_entry.id);

    save_datas(&payment_datas);
    RawHtml(format!("payment_datas.payments : {:#?}", payment_datas.payments))
}

#[get("/get_data?<year>&<month>")]
async fn get_data(month: u32, year: u32, payment_datas: &State<PaymentDatasPointer>) -> Json<PaymentDatas> {
    let payment_datas = payment_datas.read().await;
    Json(get_date_entries(&payment_datas, month, year))
}

fn calculate_total(entries: &[&PaymentEntry]) -> f64 {
    entries.iter().map(|entry| entry.price).sum()
}

fn calculate_total_for_payment_method(entries: &[&PaymentEntry], method: PaymentMethod) -> f64 {
    entries.iter()
           .filter(|entry| entry.payment_method == method.as_str())
           .map(|entry| entry.price)
           .sum()
}

fn calculate_total_for_good_type(entries: &[&PaymentEntry], good_type: GoodType) -> f64 {
    entries.iter()
           .filter(|entry| entry.goods_type == good_type.as_str())
           .map(|entry| entry.price)
           .sum()
}

fn calculate_combined_total(entries: &[&PaymentEntry], method: PaymentMethod, good_type: GoodType) -> f64 {
    entries.iter()
           .filter(|entry| entry.payment_method == method.as_str() && entry.goods_type == good_type.as_str())
           .map(|entry| entry.price)
           .sum()
}

#[get("/get_total?<year>&<month>")]
async fn get_total(month: u32, year: u32, payment_datas: &State<PaymentDatasPointer>) -> Json<PaymentTotal> {
    let payment_datas = payment_datas.read().await;
    let entries = get_date_entries_readonly(&payment_datas, month, year);
    let mut result = PaymentTotal::new();

    result.total = calculate_total(&entries);

    result.cb = calculate_total_for_payment_method(&entries, PaymentMethod::CarteBleue);
    result.cash = calculate_total_for_payment_method(&entries, PaymentMethod::Especes);

    result.food = calculate_total_for_good_type(&entries, GoodType::Nourriture);
    result.charges = calculate_total_for_good_type(&entries, GoodType::Charges);
    result.miscellaneous = calculate_total_for_good_type(&entries, GoodType::Autres);

    result.cb_charges = calculate_combined_total(&entries, PaymentMethod::CarteBleue, GoodType::Charges);
    result.cb_food = calculate_combined_total(&entries, PaymentMethod::CarteBleue, GoodType::Nourriture);
    result.cb_miscellaneous = calculate_combined_total(&entries, PaymentMethod::CarteBleue, GoodType::Autres);
    result.cash_charges = calculate_combined_total(&entries, PaymentMethod::Especes, GoodType::Charges);
    result.cash_food = calculate_combined_total(&entries, PaymentMethod::Especes, GoodType::Nourriture);
    result.cash_miscellaneous = calculate_combined_total(&entries, PaymentMethod::Especes, GoodType::Autres);

    Json(result)
}

#[launch]
fn rocket() -> _ {
    // You must mount the static_files route
    env_logger::init();
    let data = match try_load_save() {
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

fn try_load_save() -> Result<PaymentDatas, Box<dyn std::error::Error>> {
    let mut contents = String::new();

    let mut file = match File::open(SAVE_FILE_PATH) {
        Ok(file)  => file,
        Err(e) => {
            log::error!("error when opening save file : {}", e);
            return Err(Box::new(e))
        },
    };
    match file.read_to_string(&mut contents) {
        Ok(_)  => (),
        Err(e) => {
            log::error!("error when reading save file : {}", e);
            return Err(Box::new(e))
        },
    };

    match serde_json::from_str(&contents) {
        Ok(data)  => Ok(data),
        Err(e) => {
            log::error!("error when deserialising : {}", e);
            Err(Box::new(e))
        },
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

fn get_date_entries_readonly(data: &PaymentDatas, month: u32, year: u32) -> Vec<&PaymentEntry>
{
    data.payments.iter().filter(|entry| {
        let entry_date = DateTime::<Utc>::from_timestamp(entry.date, 0).unwrap().date_naive();
        (month == 0 || entry_date.month() == month) &&
        (year == 0 || entry_date.year_ce() == (true, year))
    }).collect()
}

fn get_date_entries(data: &PaymentDatas, month: u32, year: u32) -> PaymentDatas
{
    PaymentDatas { payments: data.payments.clone().into_iter().filter(|entry| {
        let entry_date = DateTime::<Utc>::from_timestamp(entry.date, 0).unwrap().date_naive();
        (month == 0 || entry_date.month() == month) &&
            (year == 0 || entry_date.year_ce() == (true, year))
    }).collect()
    }
}
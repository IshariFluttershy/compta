use std::fs::File;
use std::io::{Write, Read};
use std::path::PathBuf;
use std::sync::Arc;
use common::{PaymentEntry, PaymentDatas, PaymentTotal, PaymentMethod, GoodType};
use rocket::State;
use rocket::form::Form;
use rocket::tokio::sync::Mutex;
use rocket::{fs::NamedFile, response::{status::NotFound, content::RawHtml}};
use rocket::serde::json::Json;
use chrono::{DateTime, Datelike, Local, SecondsFormat};
use chrono::Utc;

#[macro_use] extern crate rocket;

type PaymentDatasPointer = Arc<Mutex<PaymentDatas>>;

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
    let mut payment_datas = payment_datas.lock().await;
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

    let json = match serde_json::to_string(&payment_datas.clone()) {
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
    let datetime_format: String = Local::now().to_rfc3339_opts(SecondsFormat::Secs, true);
    let mut file: File = File::create(datetime_format).unwrap();
    let json: String = serde_json::to_string(&payment_datas.clone()).unwrap();
    file.write_all(json.as_bytes()).unwrap();
    // C'est mieux de mettre le programme dans un état qui permet de renvoyer a l'utilisateur que c'est la merde
    // Genre faudrait définir un enum qui dit si on est dans un état normal de fonctionnement ou non
    // Et si on est pas dans un état normal, faire un retour a l'utilisateur
    panic!("Error when saving datas. Reach the maintainer of the program to fix the problem and get emergency saved datas");
}

#[post("/delete", data = "<delete_entry>")]
async fn delete(delete_entry: Form<DeleteEntryRequest>, payment_datas: &State<PaymentDatasPointer>) -> RawHtml<String> {
    let mut payment_datas = payment_datas.lock().await;
    payment_datas.payments.remove(delete_entry.id);

    save_datas(&payment_datas);
    RawHtml(format!("payment_datas.payments : {:#?}", payment_datas.payments))
}

#[get("/get_data?<year>&<month>")]
async fn get_data(month: u32, year: u32, payment_datas: &State<PaymentDatasPointer>) -> Json<PaymentDatas> {
    let payment_datas = payment_datas.lock().await;
    Json(get_date_entries(&payment_datas, month, year))
}

#[get("/get_total?<year>&<month>")]
async fn get_total(month: u32, year: u32, payment_datas: &State<PaymentDatasPointer>) -> Json<PaymentTotal> {
    let payment_datas = payment_datas.lock().await;
    let entries = get_date_entries(&payment_datas, month, year).payments;
    let mut result = PaymentTotal::new();

    result.total = entries.iter().map(|entry| entry.price).sum();

    result.cb = entries.iter().filter(|entry| entry.payment_method == PaymentMethod::CarteBleue.as_str()).map(|entry| entry.price).sum();
    result.cash = entries.iter().filter(|entry| entry.payment_method == PaymentMethod::Especes.as_str()).map(|entry| entry.price).sum();

    result.food = entries.iter().filter(|entry| entry.goods_type == GoodType::Nourriture.as_str()).map(|entry| entry.price).sum();
    result.charges = entries.iter().filter(|entry| entry.goods_type == GoodType::Charges.as_str()).map(|entry| entry.price).sum();
    result.miscellaneous = entries.iter().filter(|entry| entry.goods_type == GoodType::Autres.as_str()).map(|entry| entry.price).sum();

    result.cb_charges = entries.iter().filter(|entry| entry.payment_method == PaymentMethod::CarteBleue.as_str() && entry.goods_type == GoodType::Charges.as_str()).map(|entry| entry.price).sum();
    result.cb_food = entries.iter().filter(|entry| entry.payment_method == PaymentMethod::CarteBleue.as_str() && entry.goods_type == GoodType::Nourriture.as_str()).map(|entry| entry.price).sum();
    result.cb_miscellaneous = entries.iter().filter(|entry| entry.payment_method == PaymentMethod::CarteBleue.as_str() && entry.goods_type == GoodType::Autres.as_str()).map(|entry| entry.price).sum();
    result.cash_charges = entries.iter().filter(|entry| entry.payment_method == PaymentMethod::Especes.as_str() && entry.goods_type == GoodType::Charges.as_str()).map(|entry| entry.price).sum();
    result.cash_food = entries.iter().filter(|entry| entry.payment_method == PaymentMethod::Especes.as_str() && entry.goods_type == GoodType::Nourriture.as_str()).map(|entry| entry.price).sum();
    result.cash_miscellaneous = entries.iter().filter(|entry| entry.payment_method == PaymentMethod::Especes.as_str() && entry.goods_type == GoodType::Autres.as_str()).map(|entry| entry.price).sum();

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
            println!("error when opening save file : {}", e);
            return Err(Box::new(e))
        },
    };
    match file.read_to_string(&mut contents) {
        Ok(_)  => (),
        Err(e) => {
            println!("error when reading save file : {}", e);
            return Err(Box::new(e))
        },
    };

    match serde_json::from_str(&contents) {
        Ok(data)  => Ok(data),
        Err(e) => {
            println!("error when deserialising : {}", e);
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

fn get_date_entries(data: &PaymentDatas, month: u32, year: u32) -> PaymentDatas
{
    PaymentDatas { payments: data.payments.clone().into_iter().filter(|entry| {
        let entry_date = DateTime::<Utc>::from_timestamp(entry.date, 0).unwrap().date_naive();
        (month == 0 || entry_date.month() == month) &&
            (year == 0 || entry_date.year_ce() == (true, year))
    }).collect()
    }
}
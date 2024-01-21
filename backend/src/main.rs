mod file_handling;
mod payment;

use std::path::PathBuf;
use std::sync::Arc;
use common::{PaymentEntry, PaymentDatas, PaymentTotal};
use file_handling::{save_datas, try_load_save};
use payment::{calculate_payment_total, get_date_entries_readonly, get_date_entries};
use rocket::http::Status;
use rocket::{State, Request, response, Response};
use rocket::form::Form;
use rocket::tokio::sync::{Mutex, RwLock};
use rocket::{fs::NamedFile, response::status::NotFound};
use rocket::serde::json::Json;
use rocket::response::Responder;

#[macro_use] extern crate rocket;

type PaymentDatasPointer = Arc<RwLock<PaymentDatas>>;
type ApplicationState = RwLock<ApplicationError>;

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

#[derive(Debug, PartialEq, Clone, Copy)]
enum ApplicationError {
    Ok,
    SaveDataError,
    LoadDataError
}

impl<'r, 'o: 'r> Responder<'r, 'o> for ApplicationError {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'o> {
        // You can customize the response based on the error type
        Response::build().status(Status::InternalServerError).ok()
    }
}

macro_rules! check_state {
    ($state:expr) => {
        let current_state = $state.read().await;
        if *current_state != ApplicationError::Ok {
            return Err(*current_state);
        }
    };
}

#[launch]
fn rocket() -> _ {
    // You must mount the static_files route
    env_logger::init();
    let mut functionnal_state = ApplicationError::Ok;
    let data = match try_load_save(SAVE_FILE_PATH) {
        Ok(loaded_data) => loaded_data,
        Err(e) => {
            log::error!("Error when loading save data : {}", e);
            functionnal_state = ApplicationError::LoadDataError;
            PaymentDatas::new()
        }
    };

    let data_pointer: Arc<Mutex<PaymentDatas>> = Arc::new(Mutex::new(data));

    rocket::build()
        .manage(data_pointer)
        .manage(functionnal_state)
        .mount("/", routes![index, static_files, command, delete, get_data, get_total])
}

// Return the index when the url is /
#[get("/")]
async fn index() -> Result<NamedFile, NotFound<String>> {
    get_index().await
}

#[post("/command", data = "<payment_entry>")]
async fn command(payment_entry: Form<PaymentEntryRequest<'_>>, payment_datas: &State<PaymentDatasPointer>, functionnal_state: &State<ApplicationState>) -> Result<Json<PaymentDatas>, ApplicationError> {
    check_state!(functionnal_state);

    let entry = PaymentEntry {
        price: payment_entry.price,
        goods_type: payment_entry.goods_type.to_string(),
        payment_method: payment_entry.payment_method.to_string(),
        date: payment_entry.date,
    };
    let mut payment_datas = payment_datas.write().await;
    payment_datas.payments.push(entry);

    if save_datas(&payment_datas, SAVE_FILE_PATH).is_err() {
        *functionnal_state.write().await = ApplicationError::SaveDataError;
    }
    Ok(Json(payment_datas.clone()))
}

#[post("/delete", data = "<delete_entry>")]
async fn delete(delete_entry: Form<DeleteEntryRequest>, payment_datas: &State<PaymentDatasPointer>, functionnal_state: &State<ApplicationState>) -> Result<(), ApplicationError> {
    check_state!(functionnal_state);
    let mut payment_datas = payment_datas.write().await;
    payment_datas.payments.remove(delete_entry.id);

    if save_datas(&payment_datas, SAVE_FILE_PATH).is_err() {
        *functionnal_state.write().await = ApplicationError::SaveDataError;
    }    
    Ok(())
}

#[get("/get_data?<year>&<month>")]
async fn get_data(month: u32, year: u32, payment_datas: &State<PaymentDatasPointer>, functionnal_state: &State<ApplicationState>) -> Result<Json<PaymentDatas>, ApplicationError> {
    check_state!(functionnal_state);
    let payment_datas = payment_datas.read().await;
    Ok(Json(get_date_entries(&payment_datas, month, year)))
}

#[get("/get_total?<year>&<month>")]
async fn get_total(month: u32, year: u32, payment_datas: &State<PaymentDatasPointer>, functionnal_state: &State<ApplicationState>) -> Result<Json<PaymentTotal>, ApplicationError> {
    check_state!(functionnal_state);
    let payment_datas = payment_datas.read().await;
    let entries = get_date_entries_readonly(&payment_datas, month, year);
    Ok(Json(calculate_payment_total(&entries)))
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
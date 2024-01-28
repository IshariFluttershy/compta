use std::{ops::Add, fs::File, io::{Write, Read}};

use chrono::Local;
use common::PaymentDatas;

pub fn save_datas(payment_datas: &PaymentDatas, save_file_path: &str) -> Result<(), ()> {
    if try_save_datas(payment_datas, save_file_path).is_err() {
        save_emergency_datas(payment_datas);
        return Err(());
    };
    Ok(())
}

fn try_save_datas(payment_datas: &PaymentDatas, save_file_path: &str) -> Result<(), ()>
{
    let mut file = match File::create(save_file_path) {
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


    // Try to create the file and return early if there's an error
    let mut file = match File::create(datetime_format) {
        Ok(file) => file,
        Err(err) => {
            log::error!("Failed to create file in emergency save: {} ", err);
            return;
        }
    };

    // Try to serialize payment_datas and return early if there's an error
    let json = match serde_json::to_string(payment_datas) {
        Ok(json) => json,
        Err(err) => {
            log::error!("Failed to serialize data in emergency save: {}", err);
            return;
        }
    };

    // Try to write to the file and log error if there's an issue
    if let Err(err) = file.write_all(json.as_bytes()) {
        log::error!("Failed to write to emergency save file: {}", err);
    }
}

pub fn try_load_data(save_file_path: &str) -> Result<PaymentDatas, Box<dyn std::error::Error>> {
    let mut contents = String::new();

    let mut file = match File::open(save_file_path) {
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
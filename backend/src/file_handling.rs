use std::{ops::Add, fs::File, io::{Write, Read}};

use chrono::Local;
use common::PaymentDatas;

pub fn save_datas(payment_datas: &PaymentDatas, save_file_path: &str) {
    if try_save_datas(payment_datas, save_file_path).is_err() {
        save_emergency_datas(payment_datas);
    };
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
    let mut file: File = File::create(datetime_format).unwrap();
    let json: String = serde_json::to_string(&payment_datas).unwrap();
    file.write_all(json.as_bytes()).unwrap();
    // C'est mieux de mettre le programme dans un état qui permet de renvoyer a l'utilisateur que c'est la merde
    // Genre faudrait définir un enum qui dit si on est dans un état normal de fonctionnement ou non
    // Et si on est pas dans un état normal, faire un retour a l'utilisateur
    panic!("Error when saving datas. Reach the maintainer of the program to fix the problem and get emergency saved datas");
}

pub fn try_load_save(save_file_path: &str) -> Result<PaymentDatas, Box<dyn std::error::Error>> {
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
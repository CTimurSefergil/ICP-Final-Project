// Vatandaş verilerinin girilmesini ve bu verilerin
// json uzantılı bir dosyaya düzenli bir şekilde kayıt edilmesini sağlayan kodlar.

use std::fs::File;
use std::io::{BufWriter, Write};
mod lib;
use lib::Citizen;
use lib::CITIZENS;

pub fn write_citizen_data(vec: &Vec<Citizen>) -> std::io::Result<()> {
    let file = File::create(
        "/home/timur/motoko_workshop/rfinal_project/Citizenship/src/Citizenship_backend/all_data/data.json",
    )?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer_pretty(&mut writer, &vec)?;
    writer.flush()?;
    Ok(())
}

fn add_citizen(tc: usize, name: String, surname: String, age: usize, about: String) {
    unsafe {
        CITIZENS.push(Citizen {
            tc: tc.try_into().unwrap(),
            name,
            surname,
            age: age.try_into().unwrap(),
            about,
        });
    }
}
fn main() {
    unsafe {
        add_citizen(
            124213,
            String::from("Timur"),
            String::from("Sefergil"),
            421332,
            String::from("Staja kabul almak için heyecanla çalışıyor"),
        );
        add_citizen(
            124341324,
            String::from("Umay"),
            String::from("Sefergil"),
            421332,
            String::from("Balkan gezisine gidiyor"),
        );
        add_citizen(
            241244512,
            String::from("Yavuz"),
            String::from("Sefergil"),
            421332,
            String::from("Takılıyor öyle böyle"),
        );
        add_citizen(
            54363452,
            String::from("Utku"),
            String::from("Akalın"),
            421332,
            String::from("Şaka yaparak oyun oynuyor"),
        );
        add_citizen(
            5234124,
            String::from("Hamza"),
            String::from("Erdoğan"),
            421332,
            String::from("Minecraft oynayıp zorbalanıyor"),
        );
        write_citizen_data(&CITIZENS).unwrap();
    }
}

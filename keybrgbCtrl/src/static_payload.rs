use crate::payload::Payload;
use std::{thread, time, process};

pub fn create_static_payload(payload: Payload) -> std::io::Result<()> {
    let first_payload = Payload::new(
        "wave".to_string(),
        payload.get_speed(),
        0,
        payload.get_direction().to_owned(),
        payload.get_color().to_owned(),
    );
    let second_payload = Payload::new(
        "shifting".to_string(),
        4,
        100,
        payload.get_direction().to_owned(),
        payload.get_color().to_owned(),
    );
    let third_payload = Payload::new(
        "shifting".to_string(),
        0,
        payload.get_brightness(),
        payload.get_direction().to_owned(),
        payload.get_color().to_owned(),
    );
    let return_code = first_payload.send_payload();
    match return_code {
        Ok(_) => {},
        Err(e) => {
            println!("ERROR: {}", e);
            process::exit(1);
        }
    }
    thread::sleep(time::Duration::from_millis(10));
    let return_code = second_payload.send_payload();
    match return_code {
        Ok(_) => {},
        Err(e) => {
            println!("ERROR: {}", e);
            process::exit(1);
        }
    }
    thread::sleep(time::Duration::from_millis(660));
    let return_code = third_payload.send_payload();
    match return_code {
        Ok(_) => {},
        Err(e) => {
            println!("ERROR: {}", e);
            process::exit(1);
        }
    }
    Ok(())
}
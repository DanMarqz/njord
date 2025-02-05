use chrono::Local;

pub fn get_time() -> String {
    let now = Local::now(); // Obtiene la fecha y hora actual en la zona horaria local
    let time_string = now.format("%H:%M:%S").to_string(); // Formatea como HH:MM:SS

    return time_string
}

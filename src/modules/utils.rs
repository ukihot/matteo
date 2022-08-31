use calamine::{open_workbook, Reader, Xlsx};
use dotenv::dotenv;
use std::env;

pub fn read_excel() {
    dotenv().ok();

    print!("{}", env::var("FILE_PATH").unwrap());
}

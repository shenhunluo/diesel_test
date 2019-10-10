use chrono::NaiveDateTime;
use diesel::Queryable;

#[derive(Queryable)]
pub struct Commodity {
    pub id: String,
    pub commodity_name: String,
    pub commodity_class: String,
    pub commodity_num: String,
    pub trademark: String,
    pub specification: String,
    pub units: String,
    pub commodity_image: Option<String>,
    pub sign: Option<i32>,
    pub start_using: Option<i32>,
    pub notes: Option<String>,
    pub index_word: Option<String>,
    pub used: Option<i32>,
    pub createBy: Option<String>,
    pub createOn: Option<NaiveDateTime>,
    pub updateBy: Option<String>,
    pub updateOn: Option<NaiveDateTime>,
    pub barcode: Option<String>,
    pub lastPutInPrice: Option<i32>,
}
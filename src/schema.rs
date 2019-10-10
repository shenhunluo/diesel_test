table! {
    commodity (id) {
        id -> Text,
        commodity_name -> Varchar,
        commodity_class -> Varchar,
        commodity_num -> Varchar,
        trademark -> Varchar,
        specification -> Varchar,
        units -> Varchar,
        commodity_image -> Nullable<Varchar>,
        sign -> Nullable<Integer>,
        start_using -> Nullable<Integer>,
        notes -> Nullable<Varchar>,
        index_word -> Nullable<Varchar>,
        used -> Nullable<Integer>,
        createBy -> Nullable<Varchar>,
        createOn -> Nullable<Timestamp>,
        updateBy -> Nullable<Varchar>,
        updateOn -> Nullable<Timestamp>,
        barcode -> Nullable<Varchar>,
        lastPutInPrice -> Nullable<Integer>,
    }
}

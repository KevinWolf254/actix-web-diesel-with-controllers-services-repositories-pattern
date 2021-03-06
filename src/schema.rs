table! {
    organisations (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Varchar>,
        vat_number -> Nullable<Varchar>,
        pin_number -> Varchar,
        created -> Timestamptz,
        updated -> Nullable<Timestamptz>,
    }
}

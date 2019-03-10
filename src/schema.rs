table! {
    hero (id) {
        id -> Nullable<Int4>,
        name -> Varchar,
        identity -> Varchar,
        hometown -> Varchar,
        age -> Int4,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        deleted_at -> Nullable<Timestamptz>,
    }
}
